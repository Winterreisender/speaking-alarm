use std::{sync::Mutex};
use chrono::{Timelike, Local};
#[cfg(target_os = "macos")]
use cocoa_foundation::base::id;
#[cfg(target_os = "macos")]
use cocoa_foundation::foundation::NSRunLoop;
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};
use lazy_static::lazy_static;
use tauri::api::path::data_dir;
use tts::Tts;

use crate::app_config::{get_config, DateReport};

lazy_static! {
    static ref TTS: std::sync::Mutex<tts::Tts> = {
        let tts = Tts::default().unwrap();
        Mutex::new(tts)
    };
}

pub fn time_report() -> Result<(), tts::Error> {
    dbg!(get_config().hour12);
    const AM :bool = false;
    const PM :bool = true;

    let now = Local::now();

    let ref time_text = if get_config().hour12 {
        let (m,hour) = now.hour12();
        format!(
            "现在是 {} {:02}:{:02}",
            match m {
                AM => "上午",
                PM => "下午"
            },
            hour,
            now.minute()
        )
    } else {
        format!(
            "现在是 {:02}:{:02}",
            now.hour(),
            now.minute()
        )
    };
    
    use DateReport::*;
    let text = match get_config().date_report {
        None => time_text,
        _ => panic!("unknown date_report {:?}",get_config().date_report)
    };

    TTS.lock().unwrap().speak(text, true)?;

    Ok(())
}