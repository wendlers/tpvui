use std::{sync::{Arc, Mutex}, thread, time};
use reqwest::blocking::Response;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct TpvFocus {
    pub name: String,
    pub country: String,
    pub team: String,
    pub teamCode: String,
    pub power: u32,
    pub avgPower: u32,
    pub nrmPower: u32,
    pub maxPower: u32,
    pub cadence: u32,
    pub avgCadence: u32,
    pub maxCadence: u32,
    pub heartrate: u32,
    pub avgHeartrate: u32,
    pub maxHeartrate: u32,
    pub time: u32,
    pub distance: u32,
    pub height: u32,
    pub speed: u32,
    pub tss: u32,
    pub calories: u32,
    pub draft: u32,
    pub windSpeed: u32,
    pub windAngle: u32,
    pub slope: i32,
    pub eventLapsTotal: u32,
    pub eventLapsDone: i32,
    pub eventDistanceTotal: u32,
    pub eventDistanceDone: u32,
    pub eventDistanceToNextLocation: u32,
    pub eventNextLocation: u32,
    pub eventPosition: u32,
}

impl TpvFocus {
    fn new() -> TpvFocus {
        TpvFocus {
            name: String::from("--"),
            country:  String::from("--"),
            team: String::from("--"),
            teamCode: String::from("--"),
            power: 0,
            avgPower: 0,
            nrmPower: 0,
            maxPower: 0,
            cadence: 0,
            avgCadence: 0,
            maxCadence: 0,
            heartrate: 0,
            avgHeartrate: 0,
            maxHeartrate: 0,
            time: 0,
            distance: 0,
            height: 0,
            speed: 0,
            tss: 0,
            calories: 0,
            draft: 0,
            windSpeed: 0,
            windAngle: 0,
            slope: 0,
            eventLapsTotal: 0,
            eventLapsDone: 0,
            eventDistanceTotal: 0,
            eventDistanceDone: 0,
            eventDistanceToNextLocation: 0,
            eventNextLocation: 0,
            eventPosition: 0,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum DataSourceStatus {
    Connected,
    Disconnected,
}

#[derive(Clone)]
pub struct DataSource {
    pub started: bool,
    pub status: DataSourceStatus,
    pub uri: String,
    pub frame: u64,
}

impl DataSource {
    pub fn new() -> DataSource {
        DataSource {
            started: false,
            status: DataSourceStatus::Disconnected,
            uri: String::from("http://192.168.2.118:8080"),
            frame: 0,
        }
    }
}

#[derive(Clone)]
pub struct DataCollector {
    source: Arc<Mutex<DataSource>>, 
    focus: Arc<Mutex<TpvFocus>>,
}


impl DataCollector {
    pub fn new() -> DataCollector {
        // let driver_master = Arc::new(Mutex::new(0));
        // let driver  = Arc::clone(&driver_master);
        DataCollector {
            source:  Arc::new(Mutex::new(DataSource::new())),
            focus:  Arc::new(Mutex::new(TpvFocus::new())),
        }
    }

    fn collect_focus(&self) {
        let source = Arc::clone(&self.source);
        let focus = Arc::clone(&self.focus);

        thread::spawn(move || {
            log::info!("DataCollector thread for 'focus' started");

            loop {
                loop {
                    // thread::sleep(time::Duration::from_millis(1000));
                    log::info!("Collecting 'focus' data");

                    let url: String; 
                    {
                        let source_locked = source.lock().unwrap();
                        url  = format!("{}/{}", source_locked.uri.clone(), "bcast/focus");
                    }
                    log::info!("GET {}", url);

                    let res: Response;

                    match reqwest::blocking::get(url) {
                        Ok(r) => res = r,
                        Err(_) => break,
                    }

                    log::info!("'focus' Response:\n{:?} {}\nHeaders: {:#?}", res.version(), res.status(), res.headers());

                    let mut focus_list: Vec<TpvFocus>;
                    
                    match res.json() {
                        Ok(j) => focus_list = j,
                        Err(_) => break,    
                    }

                    log::info!("'focus' json:\n{focus_list:#?}");
                    let mut focus_locked = focus.lock().unwrap();

                    match focus_list.pop() {
                        Some(f) => *focus_locked = f,
                        None => log::warn!("Invalid 'focus' data"),
                    }

                    // all good, we got the data
                    {
                        let mut source_locked = source.lock().unwrap();
                        source_locked.status = DataSourceStatus::Connected;
                        source_locked.frame += 1;
                    }
                    thread::sleep(time::Duration::from_millis(250));
                }
                // failed to get  the data
                {
                    let mut source_locked = source.lock().unwrap();
                    source_locked.status = DataSourceStatus::Disconnected;
                }

                log::warn!("Failed to retrive 'focus' data");
                // retry ..
                thread::sleep(time::Duration::from_millis(1000));
            }
        });
    }

    pub fn start(&mut self) {
        let mut s = self.source.lock().unwrap();
        if !s.started  {
            log::info!("DataCollector started");
            s.started = true;
            self.collect_focus();
        }
    }

    pub fn get_data_source(&self) -> DataSource {
        self.source.lock().unwrap().clone()
    }

    pub fn get_focus(&self) -> TpvFocus {
         self.focus.lock().unwrap().clone()
    }
}

