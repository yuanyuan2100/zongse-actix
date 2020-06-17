use chrono::{NaiveDateTime, prelude::*};
use std::{thread, time};

pub fn get_now() -> NaiveDateTime {
    let dt = Local::now();
    let d = NaiveDate::from_ymd(dt.year(), dt.month(), dt.day());
    let t = NaiveTime::from_hms(dt.hour(), dt.minute(), dt.second());
    NaiveDateTime::new(d, t)
}

pub fn wait(s :u64) {
    
    let duration = time::Duration::from_secs(s);

    thread::sleep(duration);
}