use std::{sync::Mutex};
use chrono::{Timelike, Local};
#[cfg(target_os = "macos")]
use cocoa_foundation::base::id;
#[cfg(target_os = "macos")]
use cocoa_foundation::foundation::NSRunLoop;
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};
use lazy_static::lazy_static;
use tts::Tts;


lazy_static! {
    static ref TTS: std::sync::Mutex<tts::Tts> = {
        let tts = Tts::default().unwrap();
        Mutex::new(tts)
    };
}

pub fn time_report() {
    let now = Local::now();
    let ref text = format!(
        "现在是 {:02}:{:02}",
        now.hour(),
        now.minute()
    );
    TTS.lock().unwrap().speak(text, true).unwrap();
}