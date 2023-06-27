use std::{time::{Duration}, sync::Mutex};
use chrono::Timelike;
use tokio::{self, task::JoinHandle};

use crate::speaker;

static integral_minutes: Mutex<Vec<u32>> = Mutex::new(vec![]);

pub fn start_timer(dur :Duration) -> JoinHandle<()> {

    let mut interval = tokio::time::interval(dur);
    
    tokio::spawn(async move {
        // Skip the first tick
        interval.tick().await;
        loop {
            interval.tick().await;

            let now = chrono::Local::now();
            if integral_minutes.lock().unwrap().contains(&now.minute()) {
                speaker::time_report().unwrap();
            }
        }
    })
}

pub fn set_integral_minutes(minutes :&Vec<u32>) {
    let mut guard = integral_minutes.lock().unwrap();
    *guard = minutes.clone();
}