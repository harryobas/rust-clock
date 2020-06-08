

use chrono::{NaiveTime, Duration};
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
     hours: i64,
     minutes: i64
}

impl Clock {

    pub fn new(hours: i64, minutes: i64) -> Self {

        let (hrs, mins) = ClockInitializer::set_hrs_and_mins(hours, minutes);

        Clock {
            hours: hrs,
            minutes: mins
        }
    }

    pub fn add_minutes(mut self, minutes: i64) -> Self {
        let mins = Duration::minutes(minutes);
        let mins = mins + Duration::minutes(self.minutes);

        if mins.num_minutes() >= 60 || mins.num_minutes() < 0{
            let (hrs, mins) = TimeNormalizer::normalize_time(self.hours, mins.num_minutes());
            self.hours = hrs;
            self.minutes = mins;
            return self;
        }
        self.minutes = mins.num_minutes();
        self
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time = NaiveTime::from_hms(self.hours as u32, self.minutes as u32, 0);
        let time = time.format("%H:%M");

        write!(f, "{}", time)
    }
}

//struct TimeNormalizer;

impl TimeNormalizer {

    fn normalize_time(mut hrs: i64, mut mins: i64) -> (i64, i64) {

        const MAX_HRS: i64 = 24;
        const MAX_MINS: i64 = 60;

        if hrs > MAX_HRS && mins > MAX_MINS {
            if hrs%MAX_HRS == 0 && mins%MAX_MINS == 0 {
                hrs = 0;
                mins = 0;
            } else{
                let v = hrs + (mins/MAX_MINS);
                hrs = (hrs + (mins/MAX_MINS))-(MAX_HRS*(v/MAX_HRS));
                mins = mins-(mins/MAX_MINS)*MAX_MINS;
            }
        }

        if hrs > MAX_HRS && mins < MAX_MINS {
            loop {
                 hrs -= MAX_HRS;
                 if hrs < MAX_HRS {
                     break;
                 }
            }
        }

        if hrs == MAX_HRS && mins < MAX_MINS {
            hrs = 0;
        }

        if hrs < MAX_HRS && mins > MAX_MINS {
            match hrs + (mins/MAX_MINS) == MAX_HRS {
                true => {
                    hrs = 0;
                    mins = mins-(mins/MAX_MINS)*MAX_MINS;
                }
                false => {
                    match hrs + (mins/MAX_MINS) < MAX_HRS {
                        true => {
                            hrs = hrs + (mins/MAX_MINS);
                            mins = mins-(mins/MAX_MINS)*MAX_MINS;
                        }
                        false => {
                            hrs = hrs + (mins/MAX_MINS);
                            loop {
                                hrs -= MAX_HRS;
                                if hrs < MAX_HRS {
                                    break;
                                }
                            }
                            mins = mins-(mins/MAX_MINS)*MAX_MINS;
                        }
                    }
                }
            }
        }

        if hrs > MAX_HRS && mins < MAX_MINS {
            hrs = hrs/MAX_HRS;
        }

        if hrs < MAX_HRS && mins > MAX_MINS {
            let hr_lmt = hrs + (mins/MAX_MINS) >= MAX_HRS;
            match hr_lmt {
                false => {
                    hrs = hrs + (mins/MAX_MINS);
                    mins = mins-(mins/MAX_MINS)*MAX_MINS;
                }
                true => {
                    hrs = hrs + (mins/MAX_MINS)-MAX_HRS;
                    mins = mins-(mins/MAX_MINS)*MAX_MINS;
                }
            }
        }

        if hrs < MAX_HRS && mins == MAX_MINS {
            hrs = hrs + 1;
            mins = 0;
        }

        if hrs < 0 && mins < MAX_MINS {
            loop {
                hrs += MAX_HRS;
                if hrs > 0 {
                    break;
                }
            }
        }
        if hrs < MAX_HRS && mins < 0 {
            match mins == -60 {
                true => {
                    hrs = hrs-1;
                    mins = 0;
                }
                false => {
                    let (hr, min) = Self::negative_minutes(hrs, mins);
                    hrs = hr;
                    mins = min;
                }
            }
        }

        (hrs, mins)
    }

    fn negative_minutes(mut hr: i64, mut min: i64) -> (i64, i64){

        let mut count = 0;
        loop {
            min += 60;
            count += 1;
            if min > 0 {
                break;
            }
        }
        if hr - count < 0 {
            hr = hr - count;
            loop{
                hr += 24;
                if hr >= 0{
                    break;
                }
            }
        }else{
            hr = hr - count;
        }
        (hr, min)
    }
}

struct ClockInitializer;

impl ClockInitializer {
    fn set_hrs_and_mins(mut hours: i64, mut minutes: i64) -> (i64, i64,){
        let mut hrs_map: HashMap<i64, i64> = HashMap::new();
        let mut mins_map: HashMap<i64, i64> = HashMap::new();

        let hrs_rng: Vec<i64> = (0..25).collect();
        let mins_rng: Vec<i64> = (0..61).collect();

        let valid_hrs = hours < *(hrs_rng.last().unwrap()) && hrs_rng.contains(&hours);
        let valid_mins = minutes < *(mins_rng.last().unwrap()) && mins_rng.contains(&minutes);

        Self::intitialize_clock(&mut hrs_map, &mut mins_map, &hrs_rng[..], &mins_rng[..]);

        if valid_hrs && valid_mins{
            if let Some(hrs) = hrs_map.get(&hours) {
                hours = *hrs;
            }
            if let Some(mins) = mins_map.get(&minutes) {
                minutes = *mins;
            }
            return (hours, minutes);
        }

        TimeNormalizer::normalize_time(hours, minutes)
    }

    fn intitialize_clock(hrs_map: &mut HashMap<i64, i64>, mins_map: &mut HashMap<i64, i64>,
            hrs_rng: &[i64], mins_rng: &[i64]) {
              Self::init_map(hrs_map, hrs_rng);
              Self::init_map(mins_map, mins_rng);
         }

    fn init_map(map: &mut HashMap<i64, i64>,  range: &[i64]){
        for &i in range {
            if i < 10 && i.to_string().len() != 2 {
                let v = format!("{}{}", 0, i);
                let v = v.parse::<i64>().unwrap();
                map.insert(i, v);
            }
            if i >= 10 && i != *(range.last().unwrap()) {
                map.insert(i, i);
            }
        }
    }
}
