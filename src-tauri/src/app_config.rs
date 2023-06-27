use std::sync::Mutex;

#[derive(Debug, Clone, Copy)]
pub enum DateReport {
    None,
    Week,
    MMDD,
    YYMMDD
}

impl DateReport {
    pub fn new(s :&str) -> DateReport {
        use DateReport::*;
        match s {
            "none" => None,
            "week" => Week,
            "mmdd" => MMDD,
            "YYMMDD" => YYMMDD,
            _ => panic!("unknown DateReport : {}", s)
        }
    }
}

pub struct Config {
    pub hour12 :bool, // 十二小时制报时
    //integral_minutes :Vec<u32>,
    pub date_report :DateReport
}

pub static CONFIG :Mutex<Config> = Mutex::new(Config {
    hour12 : false,
    date_report : DateReport::None
});

pub fn set_config(new_config :Config) {
    let mut guard = CONFIG.lock().unwrap();
    *guard = new_config;
}

pub fn get_config() -> Config {
    let guard = CONFIG.lock().unwrap();

    let config = guard;
    Config { hour12: config.hour12, date_report: config.date_report }
}