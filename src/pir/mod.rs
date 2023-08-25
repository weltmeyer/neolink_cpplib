///
/// # Neolink PIR
///
/// This module handles the controls of the pir sensor alarm
///
///
/// # Usage
///
/// ```bash
/// # To turn the pir sensor on
/// neolink pir --config=config.toml CameraName on
/// # Or off
/// neolink pir --config=config.toml CameraName off
/// ```
///
use anyhow::{Context, Result};

mod cmdline;

use super::config::Config;
use crate::common::neocam::NeoReactor;
pub(crate) use cmdline::Opt;

/// Entry point for the pir subcommand
///
/// Opt is the command line options
pub(crate) async fn main(opt: Opt, config: Config, reactor: NeoReactor) -> Result<()> {
    let config = config.get_camera_config(&opt.camera)?;
    let camera = reactor.get_or_insert(config.clone()).await?;

    if let Some(on) = opt.on {
        camera
            .run_task(|cam| {
                Box::pin(async move {
                    cam.pir_set(on)
                        .await
                        .context("Unable to set camera PIR state")
                })
            })
            .await?;
    } else {
        let pir_state = camera
            .run_task(|cam| {
                Box::pin(async move {
                    cam.get_pirstate()
                        .await
                        .context("Unable to get camera PIR state")
                })
            })
            .await?;
        let pir_ser = String::from_utf8(
            yaserde::ser::serialize_with_writer(&pir_state, vec![], &Default::default())
                .expect("Should Ser the struct"),
        )
        .expect("Should be UTF8");
        println!("{}", pir_ser);
    }

    camera.shutdown().await;

    Ok(())
}
