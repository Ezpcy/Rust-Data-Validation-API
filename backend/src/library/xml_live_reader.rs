//! This module is build upon the [notify](https://docs.rs/notify/latest/notify/) crate. It is used to read values from a xml file which will be used as validation settings.

use crate::AppState;
use actix_web::web::Data;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config as Nconfig, Event, RecommendedWatcher, RecursiveMode, Watcher};
use quick_xml::de::from_str;

///# Reads the xml file
///
/// This function reads the xml file and returns a Result with the config struct or an error.
/// The config struct is used as validation rules.
pub fn read_xml(file: &str, state: Data<AppState>) -> Result<(), quick_xml::DeError> {
    let xml = std::fs::read_to_string(file);
    let xml = match xml {
        Ok(xml) => xml,
        Err(e) => {
            dbg!(e);
            return Ok(());
        }
    };

    let config = state.valid_config.lock();
    match config {
        Ok(mut valid_config) => {
            *valid_config = from_str(&xml)?;
            dbg!(&valid_config);
        }
        Err(e) => {
            dbg!(e);
            return Ok(());
        }
    }

    Ok(())
}

///# Creates a watcher for the xml file
///
/// This function creates a watcher for the xml file and returns the watcher and a receiver.
/// The receiver is used to get the events from the watcher.
pub fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Nconfig::default(),
    )?;

    Ok((watcher, rx))
}

///# Starts the watcher for the xml file
///
/// This function gets called first from the main function and starts the watcher for the xml file.
pub async fn async_watch(file: &str, state: Data<AppState>) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(file.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(_event) => match read_xml(&file, state.clone()) {
                Ok(_) => {
                    dbg!("ok");
                }
                Err(e) => {
                    dbg!(e);
                }
            },
            Err(e) => {
                dbg!(e);
            }
        }
    }
    Ok(())
}
