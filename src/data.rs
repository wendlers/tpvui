use std::{sync::{Arc, Mutex}, thread, time};
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

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct TpvNearest {
    pub name: String,
    pub country: String,
    pub team: String,
    pub teamCode: String,
    pub speed: u32,
    pub timeGap: i32,
    pub position: u32,
    pub distance: u32,
    pub isEliminated: bool,    
}

impl TpvNearest {
    #[allow(dead_code)]
    fn new() -> TpvNearest {
        TpvNearest {
            name: String::from("--"),
            country: String::from("--"),
            team: String::from("--"),
            teamCode: String::from("--"),
            speed: 0,
            timeGap: 0,
            position: 0,
            distance: 0,
            isEliminated: false,    
        }
    }    
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct TpvEvent{
    pub name: String,
    pub route: String,
    pub laps: u32,
    pub distance: u32,
    pub height: u32,
    pub locations: u32,
    pub type_: String, 
}

impl TpvEvent {
    #[allow(dead_code)]
    fn new() -> TpvEvent {
        TpvEvent {
            name: String::from("--"),
            route: String::from("--"),
            laps: 0,
            distance: 0,
            height: 0,
            locations: 0,
            type_: String::from("--"), 
        }
    }    
}

#[derive(Clone, PartialEq)]
pub enum DataSourceStatus {
    Ok,
    NotOk,
}

#[derive(Clone)]
pub struct DataSource {
    pub started: bool,
    pub status: DataSourceStatus,
    pub frame: u64,
}

impl DataSource {
    pub fn new() -> DataSource {
        DataSource {
            started: false,
            status: DataSourceStatus::NotOk,
            frame: 0,
        }
    }
}

#[derive(Clone)]
pub struct DataCollector {
    // TPV raw 'focus' data
    source_focus: Arc<Mutex<DataSource>>, 
    data_focus: Arc<Mutex<TpvFocus>>,
    // TPV raw 'nearest' data
    source_nearest: Arc<Mutex<DataSource>>, 
    data_nearest: Arc<Mutex<Vec<TpvNearest>>>,
    // TPV raw 'event' data
    source_event: Arc<Mutex<DataSource>>,
    data_event: Arc<Mutex<TpvEvent>>,
}

impl DataCollector {
    pub fn new() -> DataCollector {
        DataCollector {
            source_focus:  Arc::new(Mutex::new(DataSource::new())),
            data_focus:  Arc::new(Mutex::new(TpvFocus::new())),
            source_nearest:  Arc::new(Mutex::new(DataSource::new())),
            data_nearest:  Arc::new(Mutex::new(Vec::new())),
            source_event:  Arc::new(Mutex::new(DataSource::new())),
            data_event: Arc::new(Mutex::new(TpvEvent::new())),
        }
    }

    fn collect_focus(&self) {
        let source = Arc::clone(&self.source_focus);
        let focus = Arc::clone(&self.data_focus);

        thread::spawn(move || {
            log::info!("DataCollector thread for 'focus' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                let status_clone = Arc::clone(&status);
                let body_clone = Arc::clone(&body);
                let request = ehttp::Request::get("http://localhost:8080/bcast/focus");

                let (last_status, last_body) = get_data_http(&status, &body, status_clone, body_clone, request);

                if last_status != 200 {
                    log::warn!("Failed to retrive 'focus' data");
                    // failed to get the data
                    update_source(&source, DataSourceStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut focus_list: Vec<TpvFocus> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => focus_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'focus' json:\n{focus_list:#?}");

                    // all good, we got some data
                    update_source(&source, DataSourceStatus::Ok);
                    {
                        let mut focus_locked = focus.lock().unwrap();

                        match focus_list.get(0) {
                            Some(f) => *focus_locked = f.clone(),
                            None => (), // empty json array -> ignore
                        }
                    }
                    thread::sleep(time::Duration::from_millis(250));
                }           
            }
        });
    }

    fn collect_nearest(&self) {
        let source = Arc::clone(&self.source_nearest);
        let focus = Arc::clone(&self.data_nearest);

        thread::spawn(move || {
            log::info!("DataCollector thread for 'nearest' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                let status_clone = Arc::clone(&status);
                let body_clone = Arc::clone(&body);
                let request = ehttp::Request::get("http://localhost:8080/bcast/nearest");

                let (last_status, last_body) = get_data_http(&status, &body, status_clone, body_clone, request);

                if last_status != 200 {
                    log::warn!("Failed to retrive 'nearest' data");
                    // failed to get the data
                    update_source(&source, DataSourceStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut nearest_list: Vec<TpvNearest> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => nearest_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'nearest' json:\n{nearest_list:#?}");

                    // all good, we got some data
                    update_source(&source, DataSourceStatus::Ok);
                    {
                        let mut nearest_locked = focus.lock().unwrap();
                        *nearest_locked = nearest_list;
                    }
                    thread::sleep(time::Duration::from_millis(250));
                }           
            }
        });
    }

    fn collect_event(&self) {
        let source = Arc::clone(&self.source_event);
        let event = Arc::clone(&self.data_event);

        thread::spawn(move || {
            log::info!("DataCollector thread for 'event' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                let status_clone = Arc::clone(&status);
                let body_clone = Arc::clone(&body);
                let request = ehttp::Request::get("http://localhost:8080/bcast/event");

                let (last_status, last_body) = get_data_http(&status, &body, status_clone, body_clone, request);

                if last_status != 200 {
                    // failed to get the data
                    log::warn!("Failed to retrive 'event' data");
                    update_source(&source, DataSourceStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    // 'type' is a reserved RUST keyword. Thus we can not have a field named 'type'
                    // in the struct for the 'serde' bindings. So what we do here is to change
                    // 'type' to 'type_' in the received body:
                    let last_body = last_body.replace("\"type\":", "\"type_\":");
                    let mut event_list: Vec<TpvEvent> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => event_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'event' json:\n{event_list:#?}");

                    // all good, we got some data
                    update_source(&source, DataSourceStatus::Ok);
                    {
                        let mut event_locked = event.lock().unwrap();

                        match event_list.get(0) {
                            Some(f) => *event_locked = f.clone(),
                            None => (), // empty json array -> ignore
                        }
                    }
                    thread::sleep(time::Duration::from_millis(5000));
                }           
            }
        });
    }

    pub fn start(&mut self) {
        let mut s = self.source_focus.lock().unwrap();
        if !s.started  {
            log::info!("DataCollector started");
            s.started = true;

            self.collect_focus();
            self.collect_nearest();
            self.collect_event();
        }
    }

    pub fn get_source_focus(&self) -> DataSource {
        self.source_focus.lock().unwrap().clone()
    }

    pub fn get_source_nearest(&self) -> DataSource {
        self.source_nearest.lock().unwrap().clone()
    }

    pub fn get_source_event(&self) -> DataSource {
        self.source_event.lock().unwrap().clone()
    }

    pub fn get_focus(&self) -> TpvFocus {
         self.data_focus.lock().unwrap().clone()
    }

    pub fn get_nearest(&self) -> Vec<TpvNearest> {
        self.data_nearest.lock().unwrap().clone()
    }

    pub fn get_event(&self) -> TpvEvent {
        self.data_event.lock().unwrap().clone()
    }
}

fn update_source(source: &Arc<Mutex<DataSource>>, s: DataSourceStatus) {
    let mut source_locked = source.lock().unwrap();
   
    if s == DataSourceStatus::Ok {
        source_locked.frame += 1;
    }
   
    source_locked.status = s;
}

fn get_data_http(status: &Arc<Mutex<u16>>, body: &Arc<Mutex<String>>, status_clone: Arc<Mutex<u16>>, body_clone: Arc<Mutex<String>>, request: ehttp::Request) -> (u16, String) {
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        match result {
            Ok(r) => (|response: &ehttp::Response| {
                let mut status_locked = status_clone.lock().unwrap();
                *status_locked = response.status;
                match response.text() {
                    Some(s) => (|body: &str|{
                        let mut body_locked = body_clone.lock().unwrap();
                        *body_locked = String::from(body);
                    })(s),
                    None => (),
                }

            })(&r),
            Err(_) => (|| {
                let mut status_locked = status_clone.lock().unwrap();
                *status_locked = 0;
            })(),
        }
    });

    let last_status: u16;
    let last_body: String;
    {
        last_status = *status.lock().unwrap();
        last_body = body.lock().unwrap().clone();
    }
    (last_status, last_body)
}

