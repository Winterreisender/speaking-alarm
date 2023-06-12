use std::{future::Future, time::{Duration}};
use chrono::Timelike;
use tokio;

use crate::speaker;

// from https://users.rust-lang.org/t/setinterval-in-rust/41664 by alice
fn set_interval<F, Fut>(mut f: F, dur: Duration)
where
    F: Send + 'static + FnMut() -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    // Create stream of intervals.
    let mut interval = tokio::time::interval(dur);
    
    tokio::spawn(async move {
        // Skip the first tick at 0ms.
        interval.tick().await;
        loop {
            // Wait until next tick.
            interval.tick().await;
            // Spawn a task for this tick.
            tokio::spawn(f());
        }
    });
}

async fn check_and_report_time(minutes :Vec<u32>) {
    let now = chrono::Local::now();
    if minutes.contains(&now.minute()) {
        speaker::time_report(false).unwrap();
    }
    println!("Timer! {}", now)
}


pub fn start_timer() {
    async fn tick() {
        check_and_report_time(vec![10,20,30,40,50]).await
    }
    set_interval(tick, Duration::from_secs(60));
}