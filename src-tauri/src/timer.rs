use std::{time::{Duration}, sync::Mutex};
use chrono::Timelike;
use tokio::{self, task::JoinHandle};
use lazy_static::lazy_static;

use crate::speaker;

async fn check_and_report_time(minutes :Vec<u32>) {
    let now = chrono::Local::now();
    if minutes.contains(&now.minute()) {
        speaker::time_report(false,String::from("none")).unwrap();
    }
    println!("Timer! {}", now)
}



static spawn: std::sync::Mutex<Option<JoinHandle<()>>> = Mutex::new(None);

pub fn start_timer(dur :Duration) -> JoinHandle<()> {

    let mut interval = tokio::time::interval(dur);
    
    tokio::spawn(async move {
        // Skip the first tick
        interval.tick().await;
        loop {
            interval.tick().await;
            tokio::spawn(check_and_report_time(vec![0]));
        }
    })
}

pub fn change_timer() {
    let mut guard = spawn.lock().unwrap();
    *guard = Some(start_timer(Duration::from_secs(60*10)));
}