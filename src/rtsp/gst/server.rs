//! Attempts to subclass RtspServer
//!
//! We are now messing with gstreamer glib objects
//! expect issues

use super::AnyResult;
use crate::config::*;

use anyhow::Context;
use gstreamer::glib::{self, object_subclass, MainLoop, Object};
use gstreamer_rtsp::RTSPAuthMethod;
use gstreamer_rtsp_server::{
    gio::{TlsAuthenticationMode, TlsCertificate},
    prelude::*,
    subclass::prelude::*,
    RTSPAuth, RTSPFilterResult, RTSPServer, RTSPToken, RTSP_TOKEN_MEDIA_FACTORY_ROLE,
};
use log::*;
use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::Arc,
};
use tokio::{
    sync::RwLock,
    task::JoinSet,
    time::{timeout, Duration},
};
use tokio_util::sync::CancellationToken;

glib::wrapper! {
    /// The wrapped RTSPServer
    pub(crate) struct NeoRtspServer(ObjectSubclass<NeoRtspServerImpl>) @extends RTSPServer;
}

impl Default for NeoRtspServer {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl NeoRtspServer {
    pub(crate) fn new() -> AnyResult<Self> {
        gstreamer::init().context("Gstreamer failed to initialise")?;
        let factory = Object::new::<NeoRtspServer>();

        // Setup auth
        let auth = factory.auth().unwrap_or_default();
        auth.set_supported_methods(RTSPAuthMethod::Basic);
        let mut un_authtoken = RTSPToken::builder()
            .field(
                //RTSP_TOKEN_MEDIA_FACTORY_ROLE: Means look inside the media factory settings and use the same permissions this user (`"anonymous"`) has
                RTSP_TOKEN_MEDIA_FACTORY_ROLE,
                "anonymous",
            )
            .build();
        auth.set_default_token(Some(&mut un_authtoken));
        factory.set_auth(Some(&auth));

        factory.connect_client_connected(|_, client| {
            client.connect_new_session(|_, session| {
                log::debug!("New Session");
                // Session timeout too small causes us to drop
                // some ffmpeg clients too soon
                // Too long causes too many open connections with
                // clients like frigate (that seem to open multiple
                //   connections without shutting down old ones)
                session.set_timeout(30);
            });
        });

        Ok(factory)
    }

    pub(crate) async fn run(&self, bind_addr: &str, bind_port: u16) -> AnyResult<()> {
        let server = self;
        server.set_address(bind_addr);
        server.set_service(&format!("{}", bind_port));
        // Attach server to default Glib context
        let _ = server.attach(None);
        let main_loop = Arc::new(MainLoop::new(None, false));

        // Run the Glib main loop.
        let main_loop_thread = main_loop.clone();
        let main_loop_cancel = CancellationToken::new();
        let main_loop_gaurd = main_loop_cancel.clone().drop_guard();
        let handle = tokio::task::spawn_blocking(move || {
            main_loop_thread.run();
            drop(main_loop_gaurd);
            AnyResult::Ok(())
        });
        timeout(Duration::from_secs(5), self.imp().threads.write())
            .await
            .with_context(|| "Timeout waiting to lock Server threads")?
            .spawn(async move { handle.await? });

        let clean_up_server = server.clone();
        let handle = tokio::task::spawn_blocking(move || {
            while !main_loop_cancel.is_cancelled() {
                if let Some(sessions) = clean_up_server.session_pool() {
                    let cleanups = sessions.cleanup();
                    if cleanups > 0 {
                        log::debug!("Cleaned up {cleanups} sessions");
                    }
                    sessions.filter(Some(&mut |_, session| {
                        let remaining = session.next_timeout_usec(glib::monotonic_time());
                        log::debug!(
                            "{:?}: {}/{}",
                            session.sessionid(),
                            remaining,
                            session.timeout(),
                        );
                        RTSPFilterResult::Keep
                    }));
                }
                std::thread::sleep(Duration::from_secs(5));
            }
            AnyResult::Ok(())
        });
        timeout(Duration::from_secs(5), self.imp().threads.write())
            .await
            .with_context(|| "Timeout waiting to lock Server threads")?
            .spawn(async move { handle.await? });

        // Put copy of main loop inside the rtsp server
        timeout(Duration::from_secs(5), self.imp().main_loop.write())
            .await
            .with_context(|| "Timeout waiting to lock Server main_loop")?
            .replace(main_loop);
        Ok(())
    }

    pub(crate) async fn quit(&self) -> AnyResult<()> {
        if let Some(main_loop) = self.imp().main_loop.read().await.as_ref() {
            main_loop.quit();
        }
        Ok(())
    }

    pub(crate) async fn join(&self) -> AnyResult<()> {
        let mut threads = self.imp().threads.write().await;
        while let Some(thread) = threads.join_next().await {
            thread??;
        }
        Ok(())
    }

    pub(crate) fn set_up_tls(&self, config: &Config) -> AnyResult<()> {
        self.imp().set_up_tls(config)
    }

    pub(crate) async fn add_user(&self, username: &str, password: &str) -> AnyResult<()> {
        self.imp().add_user(username, password).await
    }

    pub(crate) async fn remove_user(&self, username: &str) -> AnyResult<()> {
        self.imp().remove_user(username).await
    }

    pub(crate) async fn get_users(&self) -> AnyResult<HashSet<String>> {
        self.imp().get_users().await
    }
}

unsafe impl Send for NeoRtspServer {}
unsafe impl Sync for NeoRtspServer {}

#[derive(Default)]
pub(crate) struct NeoRtspServerImpl {
    threads: RwLock<JoinSet<AnyResult<()>>>,
    users: RwLock<HashMap<String, String>>,
    main_loop: RwLock<Option<Arc<MainLoop>>>,
}

impl ObjectImpl for NeoRtspServerImpl {}
impl RTSPServerImpl for NeoRtspServerImpl {}

#[object_subclass]
impl ObjectSubclass for NeoRtspServerImpl {
    const NAME: &'static str = "NeoRtspServer";
    type Type = NeoRtspServer;
    type ParentType = RTSPServer;
}

impl NeoRtspServerImpl {
    pub(crate) fn set_tls(
        &self,
        cert_file: &str,
        client_auth: TlsAuthenticationMode,
    ) -> AnyResult<()> {
        debug!("Setting up TLS using {}", cert_file);
        let auth = self.obj().auth().unwrap_or_default();

        // We seperate reading the file and changing to a PEM so that we get different error messages.
        let cert_contents = fs::read_to_string(cert_file).with_context(|| "TLS file not found")?;
        let cert = TlsCertificate::from_pem(&cert_contents)
            .with_context(|| "Not a valid TLS certificate")?;
        auth.set_tls_certificate(Some(&cert));
        auth.set_tls_authentication_mode(client_auth);

        self.obj().set_auth(Some(&auth));
        Ok(())
    }

    pub(crate) fn set_up_tls(&self, config: &Config) -> AnyResult<()> {
        let tls_client_auth = match &config.tls_client_auth as &str {
            "request" => TlsAuthenticationMode::Requested,
            "require" => TlsAuthenticationMode::Required,
            "none" => TlsAuthenticationMode::None,
            _ => unreachable!(),
        };
        if let Some(cert_path) = &config.certificate {
            self.set_tls(cert_path, tls_client_auth)
                .with_context(|| "Failed to set up TLS")?;
        }
        Ok(())
    }

    pub(crate) async fn add_user(&self, username: &str, password: &str) -> AnyResult<()> {
        let mut locked_users = self.users.write().await;
        let auth = self.obj().auth().unwrap();

        let token = RTSPToken::builder()
            .field(RTSP_TOKEN_MEDIA_FACTORY_ROLE, username)
            .build();
        let basic = RTSPAuth::make_basic(username, password);

        if let Some(old_basic) = locked_users.get(username) {
            if basic.as_str() == old_basic {
                // Password is the same
                return Ok(());
            } else {
                // Different password
                auth.remove_basic(old_basic);
            }
        }

        auth.add_basic(basic.as_str(), &token);

        locked_users.insert(username.to_string(), basic.to_string());
        Ok(())
    }

    pub(crate) async fn remove_user(&self, username: &str) -> AnyResult<()> {
        let mut locked_users = self.users.write().await;
        let auth = self.obj().auth().unwrap();

        if let Some(old_basic) = locked_users.get(username) {
            auth.remove_basic(old_basic);
        }

        locked_users.remove(username);
        Ok(())
    }

    pub(crate) async fn get_users(&self) -> AnyResult<HashSet<String>> {
        let locked_users = self.users.read().await;
        Ok(locked_users.keys().cloned().collect())
    }
}
