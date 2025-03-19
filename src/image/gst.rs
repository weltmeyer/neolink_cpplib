use std::path::Path;

use anyhow::{anyhow, Context, Result};
use gstreamer::{
    parse::launch_full, prelude::*, ClockTime, MessageView, ParseFlags, Pipeline, State,
};
use gstreamer_app::AppSrc;
use neolink_core::bcmedia::model::VideoType;
use tokio::{
    sync::{
        self,
        mpsc::{channel, Sender},
    },
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;

use crate::AnyResult;

#[derive(Debug)]
enum GstControl {
    Data(std::sync::Arc<Vec<u8>>),
    Eos,
}

pub(super) struct GstSender {
    sender: Sender<GstControl>,
    set: JoinSet<Result<()>>,
    finished: sync::oneshot::Receiver<Result<()>>,
    cancel: CancellationToken,
}

impl GstSender {
    pub(super) async fn send(&self, buf: std::sync::Arc<Vec<u8>>) -> Result<()> {
        self.sender
            .send(GstControl::Data(buf))
            .await
            .map_err(|e| anyhow!("Failed to send buffer: {:?}", e))
    }

    pub(super) async fn eos(&self) -> Result<()> {
        self.sender
            .send(GstControl::Eos)
            .await
            .map_err(|e| anyhow!("Failed to send eos: {:?}", e))
    }

    pub(super) async fn is_finished(&mut self) -> Option<Result<()>> {
        match self.finished.try_recv() {
            Ok(res) => Some(res),
            Err(sync::oneshot::error::TryRecvError::Empty) => None,
            Err(sync::oneshot::error::TryRecvError::Closed) => {
                Some(Err(anyhow!("Gstreamer finished channel is closed")))
            }
        }
    }

    pub(super) async fn join(mut self) -> Result<()> {
        while self.set.join_next().await.is_some() {}
        Ok(())
    }
}

impl Drop for GstSender {
    fn drop(&mut self) {
        log::trace!("Drop GstSender");
        self.cancel.cancel();
        let _gt = tokio::runtime::Handle::current().enter();
        let mut set = std::mem::take(&mut self.set);
        tokio::task::spawn(async move {
            while set.join_next().await.is_some() {}
            log::trace!("Dropped GstSender");
        });
    }
}

pub(super) async fn from_input<T: AsRef<Path>>(
    format: VideoType,
    out_file: T,
) -> Result<GstSender> {
    let pipeline = create_pipeline(format, out_file.as_ref())?;
    output(pipeline).await
}

async fn output(pipeline: Pipeline) -> Result<GstSender> {
    let source = get_source(&pipeline)?;
    let (sender, mut reciever) = channel::<GstControl>(100);
    let mut set = JoinSet::<AnyResult<()>>::new();
    let cancel = CancellationToken::new();
    let thread_cancel = cancel.clone();
    set.spawn(async move {
        tokio::select!{
            _ = thread_cancel.cancelled() => Result::Ok(()),
            v = async {
                while let Some(control) = reciever.recv().await {
                    tokio::task::yield_now().await;
                    match control {
                        GstControl::Data(buf) => {
                            let mut gst_buf = gstreamer::Buffer::with_size(buf.len()).unwrap();
                            {
                                let gst_buf_mut = gst_buf.get_mut().unwrap();
                                let mut gst_buf_data = gst_buf_mut.map_writable().unwrap();
                                gst_buf_data.copy_from_slice(&buf);
                            }
                            source.push_buffer(gst_buf).map_err(|e| anyhow!("Streamer Error: {e:?}"))?;
                        }
                        GstControl::Eos => {
                            source.end_of_stream().map_err(|e| anyhow!("Streamer Error: {e:?}"))?;
                            break;
                        }
                    }
                }
                Ok(())
            } => v,
        }
    });

    let (tx, finished) = sync::oneshot::channel();
    set.spawn_blocking(move || {
        let res = start_pipeline(pipeline);
        if let Err(e) = &res {
            log::error!("Failed to run pipeline: {:?}", e);
        }
        let _ = tx.send(res);
        Ok(())
    });

    Ok(GstSender {
        sender,
        set,
        finished,
        cancel,
    })
}

fn start_pipeline(pipeline: Pipeline) -> Result<()> {
    pipeline.set_state(State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline
                    .set_state(State::Null)
                    .context("Error in gstreamer when setting state to Null")?;
                log::warn!(
                    "Error from gstreamer when setting the play state {:?} setting to Null instead",
                    err
                );
            }
            _ => (),
        }
    }

    pipeline
        .set_state(State::Null)
        .context("Error in gstreamer when setting state to Null")?;

    Ok(())
}

fn get_source(pipeline: &Pipeline) -> Result<AppSrc> {
    let source = pipeline
        .by_name("thesource")
        .expect("There shoud be a `thesource`");
    source
        .dynamic_cast::<AppSrc>()
        .map_err(|_| anyhow!("Cannot find appsource in gstreamer, check your gstreamer plugins"))
}

fn create_pipeline(format: VideoType, file_path: &Path) -> Result<Pipeline> {
    gstreamer::init()
        .context("Unable to start gstreamer ensure it and all plugins are installed")?;
    let file_path = file_path.with_extension("jpeg");

    let launch_str = match format {
        VideoType::H264 => {
            format!(
                "appsrc name=thesource \
                ! h264parse \
                ! decodebin \
                ! jpegenc snapshot=TRUE
                ! filesink location={}",
                file_path.display()
            )
        }
        VideoType::H265 => {
            format!(
                "appsrc name=thesource \
                ! h265parse \
                ! decodebin \
                ! jpegenc snapshot=TRUE
                ! filesink location={}",
                file_path.display()
            )
        }
    };

    log::info!("{}", launch_str);

    // Parse the pipeline we want to probe from a static in-line string.
    // Here we give our audiotestsrc a name, so we can retrieve that element
    // from the resulting pipeline.
    let pipeline = launch_full(&launch_str, None, ParseFlags::empty())
        .context("Unable to load gstreamer pipeline ensure all gstramer plugins are installed")?;
    let pipeline = pipeline.dynamic_cast::<Pipeline>().map_err(|_| {
        anyhow!("Unable to create gstreamer pipeline ensure all gstramer plugins are installed")
    })?;

    // let appssource = get_source(&pipeline)?;

    // Tell the appsink what format we produce.
    // let caps = match format {
    //     VideoType::H264 => Caps::new_simple("video/x-h264", &[("parsed", &false)]),
    //     VideoType::H265 => Caps::new_simple("video/x-h265", &[("parsed", &false)]),
    // };
    // appssource.set_caps(Some(&caps));

    Ok(pipeline)
}
