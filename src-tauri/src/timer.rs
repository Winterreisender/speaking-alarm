use std::{time::Duration, sync::Mutex};
use chrono::Timelike;
use tokio::{self, task::JoinHandle};

use crate::speaker;

static INTEGRAL_MINUTES: Mutex<Vec<u32>> = Mutex::new(vec![]);

pub fn start_timer(dur :Duration) -> JoinHandle<()> {

    let mut interval = tokio::time::interval(dur);
    
    tokio::spawn(async move {
        // Skip the first tick
        loop {
            interval.tick().await;

            let now = chrono::Local::now();
            if INTEGRAL_MINUTES.lock().unwrap().contains(&now.minute()) {
                speaker::time_report().unwrap();
            }
        }
    })
}

pub fn set_integral_minutes(minutes :&Vec<u32>) {
    let mut guard = INTEGRAL_MINUTES.lock().unwrap();
    *guard = minutes.clone();
}