//! The sharable instance
//!
//! This communicaes with the [`NeoCam`] over channels
//!
//! The camera watch is used as an event to be triggered
//! whenever the camera is lost/updated
use anyhow::{anyhow, Context};
use futures::{stream::FuturesUnordered, FutureExt, StreamExt, TryFutureExt};
use std::sync::{Arc, Weak};
#[cfg(feature = "pushnoti")]
use tokio::sync::watch::channel as watch;
use tokio::{
    sync::{
        mpsc::Receiver as MpscReceiver, mpsc::Sender as MpscSender, oneshot::channel as oneshot,
        watch::Receiver as WatchReceiver,
    },
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;

#[cfg(feature = "pushnoti")]
use super::PushNoti;
use super::{MdState, NeoCamCommand, NeoCamThreadState, Permit, UseCounter};
use crate::{config::CameraConfig, AnyResult, Result};
use neolink_core::{
    bc_protocol::{BcCamera, StreamKind},
    bcmedia::model::BcMedia,
};

/// This instance is the primary interface used throughout the app
///
/// It uses channels to run all tasks on the actual shared `[NeoCam]`
#[derive(Clone)]
pub(crate) struct NeoInstance {
    camera_watch: WatchReceiver<Weak<BcCamera>>,
    camera_control: MpscSender<NeoCamCommand>,
    cancel: CancellationToken,
}

impl NeoInstance {
    pub(crate) fn new(
        camera_watch: WatchReceiver<Weak<BcCamera>>,
        camera_control: MpscSender<NeoCamCommand>,
        cancel: CancellationToken,
    ) -> Result<Self> {
        Ok(Self {
            camera_watch,
            camera_control,
            cancel,
        })
    }

    /// Create a new instance to the same camera
    ///
    /// Unlike clone this one will contact the NeoCam and grab it from
    /// there. There is no real benifit to this, other then being
    /// able to check if the thread is alive. Which is why it can
    /// fail.
    pub(crate) async fn subscribe(&self) -> Result<Self> {
        let (instance_tx, instance_rx) = oneshot();
        self.camera_control
            .send(NeoCamCommand::Instance(instance_tx))
            .await?;
        instance_rx.await?
    }

    /// This is a helpful convience function
    ///
    /// Given an async task it will:
    /// - Run the task with a reference to a BcCamera
    /// - If the camera instance is changed: Rerun the task with the new instance
    /// - If the camera returns a retryable error, wait for camera instance to change then rerun
    /// - else return the result of the function
    pub(crate) async fn run_task<F, T>(&self, task: F) -> AnyResult<T>
    where
        F: for<'a> Fn(
            &'a BcCamera,
        )
            -> std::pin::Pin<Box<dyn futures::Future<Output = AnyResult<T>> + Send + 'a>>,
    {
        let _permit = self.permit().await?;
        self.run_passive_task(task).await
    }

    /// This is a helpful convience function
    ///
    /// Given an async task it will:
    /// - Run the task with a reference to a BcCamera
    /// - If the camera instance is changed: Rerun the task with the new instance
    /// - If the camera returns a retryable error, wait for camera instance to change then rerun
    /// - else return the result of the function
    ///
    /// This variant does NOT take out a use permit so the camera can disconnect
    /// for inactvitity during its call. It is meant to be used for non-critial
    /// background tasks that we want to stop during certain times like low battery
    ///
    /// The streams and MD use this
    pub(crate) async fn run_passive_task<F, T>(&self, task: F) -> AnyResult<T>
    where
        F: for<'a> Fn(
            &'a BcCamera,
        )
            -> std::pin::Pin<Box<dyn futures::Future<Output = AnyResult<T>> + Send + 'a>>,
    {
        let mut camera_watch = self.camera_watch.clone();
        let mut camera = None;

        loop {
            camera = camera_watch
                .wait_for(|new_cam| {
                    let curr_as_weak = camera.as_ref().map(Arc::downgrade).unwrap_or_default();
                    !Weak::ptr_eq(new_cam, &curr_as_weak)
                })
                .map_ok(|new_cam| new_cam.upgrade())
                .await
                .with_context(|| "Camera is disconnecting")?;
            break tokio::select! {
                _ = self.cancel.cancelled() => {
                    Err(anyhow!("Camera is disconnecting"))
                }
                _ = camera_watch.wait_for(|new_cam| !Weak::ptr_eq(new_cam, &camera.as_ref().map(Arc::downgrade).unwrap_or_default())).map_ok(|new_cam| new_cam.upgrade()) => {
                    // Camera value has changed!
                    // Go back and see how it changed
                    continue;
                },
                v = async {
                    if let Some(cam) = camera.clone() {
                        let cam_ref = cam.as_ref();
                        let mut r = Err(anyhow!("No run"));
                        for i in 0..5 {
                            r = task(cam_ref).await;
                            if let Err(e) = &r {
                                log::debug!("- Task Error: {e:?}");
                            }
                            if let Err(Some(e @ neolink_core::Error::CameraServiceUnavailable{code: 400, ..})) = r.as_ref().map_err(|e| e.downcast_ref::<neolink_core::Error>()) {
                                // Retryable without a reconnect
                                // Usually occurs when camera is starting up
                                // or the connection is initialising
                                log::debug!("Got a 400 code for {e:?} retry {i}/5, ");

                                sleep(Duration::from_secs(1)).await;
                                continue;
                            } else {
                                break;
                            }
                        }
                        r
                    } else {
                        unreachable!()
                    }
                }, if camera.is_some() => {
                    match v {
                        // Ok means we are done
                        Ok(v) => Ok(v),
                        // If error we check for retryable errors
                        Err(e) => {
                            match e.downcast::<neolink_core::Error>() {
                                Ok(neolink_core::Error::DroppedConnection) | Ok(neolink_core::Error::TimeoutDisconnected) => {
                                    continue;
                                },
                                Ok(neolink_core::Error::TokioBcSendError) => {
                                    continue;
                                },
                                Ok(neolink_core::Error::Io(e)) => {
                                    use std::io::ErrorKind::*;
                                    if let ConnectionReset | ConnectionAborted | BrokenPipe | TimedOut =  e.kind() {
                                        // Resetable IO
                                        log::trace!("    - Neolink Std IO Error: Continue");
                                        continue;
                                    } else {
                                        // Check if  the inner error is the Other type and then the discomnect
                                        let is_dropped = e.get_ref().is_some_and(|e| {
                                            log::trace!("Std IO Error: Inner: {:?}", e);
                                            matches!(e.downcast_ref::<neolink_core::Error>(),
                                                    Some(neolink_core::Error::DroppedConnection) | Some(neolink_core::Error::TimeoutDisconnected)
                                            )
                                        });
                                        if is_dropped {
                                            // Retry is a None
                                            log::trace!("    - Neolink Std IO Error => Neolink: Continue");
                                            continue;
                                        } else {
                                            log::trace!("    - Neolink Std IO Error: Other");
                                            Err(e.into())
                                        }
                                    }
                                }
                                Ok(e) => {
                                    log::trace!("  - Neolink Error: Other");
                                    Err(e.into())
                                },
                                Err(e) => {
                                    // Check if it is an io error
                                    log::trace!("  - Other Error: {:?}", e);
                                    match e.downcast::<std::io::Error>() {
                                        Ok(e) => {
                                            log::trace!("    - Std IO Error");
                                            // Check if  the inner error is the Other type and then the discomnect
                                            use std::io::ErrorKind::*;
                                            if let ConnectionReset | ConnectionAborted | BrokenPipe | TimedOut =  e.kind() {
                                                // Resetable IO
                                                log::trace!("      - Std IO Error: Continue");
                                                continue;
                                            } else {
                                                let is_dropped = e.get_ref().is_some_and(|e| {
                                                    log::trace!("Std IO Error: Inner: {:?}", e);
                                                    matches!(e.downcast_ref::<neolink_core::Error>(),
                                                            Some(neolink_core::Error::DroppedConnection) | Some(neolink_core::Error::TimeoutDisconnected) | Some(neolink_core::Error::TokioBcSendError)
                                                    )
                                                });
                                                if is_dropped {
                                                    // Retry is a None
                                                    log::trace!("      - Std IO Error => Neolink Error: Continue");
                                                    continue;
                                                } else {
                                                    log::trace!("      - Std IO Error: Other");
                                                    Err(e.into())
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            log::trace!("  - Other Error: {:?}", e);
                                            Err(e)
                                        }
                                    }
                                },
                            }
                        }
                    }
                },
            };
        }
    }

    #[cfg(feature = "pushnoti")]
    pub(crate) async fn uid(&self) -> Result<String> {
        let (reply_tx, reply_rx) = oneshot();
        self.camera_control
            .send(NeoCamCommand::GetUid(reply_tx))
            .await?;
        Ok(reply_rx.await?)
    }

    #[cfg(feature = "pushnoti")]
    pub(crate) async fn push_notifications(&self) -> Result<WatchReceiver<Option<PushNoti>>> {
        let uid = self.uid().await?;
        let (instance_tx, instance_rx) = oneshot();
        self.camera_control
            .send(NeoCamCommand::PushNoti(instance_tx))
            .await?;
        let mut source_watch = instance_rx.await?;

        let (fwatch_tx, fwatch_rx) = watch(None);
        tokio::task::spawn(async move {
            loop {
                match source_watch
                    .wait_for(|i| {
                        fwatch_tx.borrow().as_ref() != i.as_ref()
                            && i.as_ref()
                                .is_some_and(|i| i.message.contains(&format!("\"{uid}\"")))
                    })
                    .await
                {
                    Ok(pn) => {
                        log::trace!("Forwarding push notification about {}", uid);
                        let _ = fwatch_tx.send_replace(pn.clone());
                    }
                    Err(e) => {
                        break Err(e);
                    }
                }
            }?;
            AnyResult::Ok(())
        });

        Ok(fwatch_rx)
    }

    pub(crate) async fn motion(&self) -> Result<WatchReceiver<MdState>> {
        let (instance_tx, instance_rx) = oneshot();
        self.camera_control
            .send(NeoCamCommand::Motion(instance_tx))
            .await?;
        Ok(instance_rx.await?)
    }

    pub(crate) async fn config(&self) -> Result<WatchReceiver<CameraConfig>> {
        let (instance_tx, instance_rx) = oneshot();
        self.camera_control
            .send(NeoCamCommand::Config(instance_tx))
            .await?;
        Ok(instance_rx.await?)
    }

    pub(crate) fn camera(&self) -> WatchReceiver<Weak<BcCamera>> {
        self.camera_watch.clone()
    }

    pub(crate) async fn connect(&self) -> Result<()> {
        let (instance_tx, instance_rx) = oneshot();
        self.camera_control
            .send(NeoCamCommand::Connect(instance_tx))
            .await?;
        Ok(instance_rx.await?)
    }

    pub(crate) async fn disconnect(&self) -> Result<()> {
        let (instance_tx, instance_rx) = oneshot();
        self.camera_control
            .send(NeoCamCommand::Disconnect(instance_tx))
            .await?;
        Ok(instance_rx.await?)
    }

    #[allow(dead_code)]
    pub(crate) async fn get_state(&self) -> Result<NeoCamThreadState> {
        let (instance_tx, instance_rx) = oneshot();
        self.camera_control
            .send(NeoCamCommand::State(instance_tx))
            .await?;
        Ok(instance_rx.await?)
    }

    pub(crate) async fn permit(&self) -> Result<Permit> {
        let (instance_tx, instance_rx) = oneshot();
        self.camera_control
            .send(NeoCamCommand::GetPermit(instance_tx))
            .await?;
        Ok(instance_rx.await?)
    }

    pub(crate) fn drop_command<F>(self, task: F, timeout: tokio::time::Duration) -> DropRunTask<F>
    where
        F: for<'a> Fn(
                &'a BcCamera,
            )
                -> std::pin::Pin<Box<dyn futures::Future<Output = Result<()>> + Send + 'a>>
            + Send
            + Sync
            + 'static,
    {
        DropRunTask {
            instance: Some(self),
            command: Some(Box::new(task)),
            timeout,
        }
    }

    /// Streams a camera source while not paused
    pub(crate) async fn stream_while_live(
        &self,
        stream: StreamKind,
    ) -> AnyResult<MpscReceiver<BcMedia>> {
        let config = self.config().await?.borrow().clone();
        let name = config.name.clone();

        let media_rx = if config.pause.on_motion {
            let (media_tx, media_rx) = tokio::sync::mpsc::channel(100);
            let counter = UseCounter::new().await;

            let mut md = self.motion().await?;
            let mut tasks = FuturesUnordered::new();
            // Stream for 5s on a new client always
            // This lets us negotiate the camera stream type
            let init_permit = counter.create_activated().await?;
            tokio::spawn(
                async {
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    drop(init_permit);
                }
                .map(|e| {
                    log::debug!("Init permit thread stopped {e:?}");
                    e
                }),
            );

            // Create the permit for controlling the motion
            let mut md_permit = {
                let md_state = md.borrow_and_update().clone();
                match md_state {
                    MdState::Start(_) => {
                        log::info!("{name}::{stream:?}: Starting with Motion");
                        counter.create_activated().await?
                    }
                    MdState::Stop(_) | MdState::Unknown => {
                        log::info!("{name}::{stream:?}: Waiting with Motion");
                        counter.create_deactivated().await?
                    }
                }
            };
            // Now listen to the motion
            let thread_name = name.clone();
            tasks.push(tokio::spawn(
                async move {
                    loop {
                        match md.changed().await {
                            Ok(_) => {
                                let md_state: MdState = md.borrow_and_update().clone();
                                match md_state {
                                    MdState::Start(_) => {
                                        log::info!("{thread_name}::{stream:?}: Motion Started");
                                        md_permit.activate().await?;
                                    }
                                    MdState::Stop(_) => {
                                        log::info!("{thread_name}::{stream:?}: Motion Stopped");
                                        md_permit.deactivate().await?;
                                    }
                                    MdState::Unknown => {}
                                }
                            }
                            Err(e) => {
                                // Use break here so we can define the full type on the async closure
                                break AnyResult::Err(e.into());
                            }
                        }
                    }?;
                    AnyResult::Ok(())
                }
                .map(|e| {
                    log::debug!("Motion thread stopped {e:?}");
                    e
                }),
            ));

            #[cfg(feature = "pushnoti")]
            {
                // Creates a permit for controlling based on the PN
                let pn_permit = counter.create_deactivated().await?;
                let mut pn = self.push_notifications().await?;
                pn.borrow_and_update(); // Ignore any PNs that have already been sent before this
                let thread_name = name.clone();
                tasks.push(tokio::spawn(
                    async move {
                        loop {
                            let noti: Option<PushNoti> = pn.borrow_and_update().clone();
                            if let Some(noti) = noti {
                                if noti.message.contains("Motion Alert from") {
                                    log::info!(
                                        "{thread_name}::{stream:?}: Push Notification Recieved"
                                    );
                                    let mut new_pn_permit = pn_permit.subscribe();
                                    new_pn_permit.activate().await?;
                                    tokio::spawn(async move {
                                        tokio::time::sleep(tokio::time::Duration::from_secs(30))
                                            .await;
                                        drop(new_pn_permit);
                                    });
                                }
                            }
                            if let Err(e) = pn.changed().await {
                                break Err(e);
                            }
                        }?;
                        AnyResult::Ok(())
                    }
                    .map(|e| {
                        log::debug!("PN thread stopped {e:?}");
                        e
                    }),
                ));
            }

            // Send the camera when the pemit is active
            let camera_permit = counter.create_deactivated().await?;
            let thread_camera = self.clone();
            tokio::spawn(
                async move {
                    loop {
                        if let Err(e) = camera_permit.aquired_users().await {
                            break AnyResult::Err(e);
                        }
                        log::debug!("Starting stream");
                        tokio::select! {
                            v = camera_permit.dropped_users() => {
                                log::debug!("Dropped users: {v:?}");
                                v
                            },
                            v = async {
                                log::debug!("Getting stream");
                                let mut stream = thread_camera.stream(stream).await?;
                                log::debug!("Got stream");
                                while let Some(media) = stream.recv().await {
                                    media_tx.send(media).await?;
                                }
                                AnyResult::Ok(())
                            } => {
                                log::debug!("Stopped stream: {v:?}");
                                v
                            },
                            v = tasks.next() => {
                                log::debug!("Task failed: {v:?}");
                                Err(anyhow!("Task ended prematurly: {v:?}"))
                            }
                        }?;
                        log::debug!("Pausing stream");
                    }?;
                    drop(counter); // Make sure counter is owned by this thread
                    AnyResult::Ok(())
                }
                .map(|e| {
                    log::debug!("Stream thread stopped {e:?}");
                    e
                }),
            );

            Ok(media_rx)
        } else {
            self.stream(stream).await
        }?;

        Ok(media_rx)
    }

    /// Streams a camera source
    pub(crate) async fn stream(&self, stream: StreamKind) -> AnyResult<MpscReceiver<BcMedia>> {
        let (media_tx, media_rx) = tokio::sync::mpsc::channel(100);
        let config = self.config().await?.borrow().clone();
        let strict = config.strict;
        let thread_camera = self.clone();
        tokio::task::spawn(
            tokio::task::spawn(async move {
                thread_camera
                    .run_task(move |cam| {
                        let media_tx = media_tx.clone();
                        Box::pin(async move {
                            let mut media_stream = cam.start_video(stream, 0, strict).await?;
                            log::trace!("Camera started");
                            while let Ok(media) = media_stream.get_data().await? {
                                media_tx.send(media).await?;
                            }
                            AnyResult::Ok(())
                        })
                    })
                    .await
            })
            .and_then(|res| async move {
                log::debug!("Camera finished streaming: {res:?}");
                Ok(())
            }),
        );

        Ok(media_rx)
    }
}

// A task that is run on a camera when the structure is dropped
pub(crate) struct DropRunTask<F>
where
    F: for<'a> Fn(
            &'a BcCamera,
        )
            -> std::pin::Pin<Box<dyn futures::Future<Output = Result<()>> + Send + 'a>>
        + Send
        + Sync
        + 'static,
{
    instance: Option<NeoInstance>,
    command: Option<Box<F>>,
    timeout: tokio::time::Duration,
}

impl<F> Drop for DropRunTask<F>
where
    F: for<'a> Fn(
            &'a BcCamera,
        )
            -> std::pin::Pin<Box<dyn futures::Future<Output = Result<()>> + Send + 'a>>
        + Send
        + Sync
        + 'static,
{
    fn drop(&mut self) {
        if let (Some(command), Some(instance)) = (self.command.take(), self.instance.take()) {
            let _gt = tokio::runtime::Handle::current().enter();
            let timeout = self.timeout;
            tokio::task::spawn(async move {
                tokio::time::timeout(timeout, instance.run_passive_task(*command)).await??;
                crate::AnyResult::Ok(())
            });
        }
    }
}
