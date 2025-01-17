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

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct TpvEntries {
    pub bibNum: u32,
    pub name: String,
    pub country: String,
    pub team: String,
    pub teamCode: String,    
}

impl TpvEntries {
    #[allow(dead_code)]
    fn new() -> TpvEntries {
        TpvEntries {
            bibNum: 0,
            name: String::from("--"),
            country: String::from("--"),
            team: String::from("--"),
            teamCode: String::from("--"),  
        }
    }    
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct TpvGroups {
    pub groupNum1: u32,
    pub groupNum2: u32,
    pub leader: String,
    pub size: u32,
    pub timeGap1: i32,
    pub timeGap2: i32,
    pub isPeloton: bool,
}

impl TpvGroups {
    #[allow(dead_code)]
    fn new() -> TpvGroups {
        TpvGroups {
            groupNum1: 0,
            groupNum2: 0,
            leader: String::from("--"),
            size: 0,
            timeGap1: 0,
            timeGap2: 0,
            isPeloton: false,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct TpvResultsIndv {
    pub location: u32,
    pub position: u32,
    pub name: String,
    pub country: String,
    pub team: String,
    pub teamCode: String,
    pub points: u32,
    pub pointsTotal: u32,
    pub time: u32,
    pub deltaTime: i32,
    pub isEliminated: bool,
}

impl TpvResultsIndv {
    #[allow(dead_code)]
    fn new() -> TpvResultsIndv {
        TpvResultsIndv {
            location: 0,
            position: 0,
            name: String::from("--"),
            country: String::from("--"),
            team: String::from("--"),
            teamCode: String::from("--"),
            points: 0,
            pointsTotal: 0,
            time: 0,
            deltaTime: 0,
            isEliminated: false,        
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct TpvResultsTeam {
    pub location: u32,
    pub position: u32,
    pub team: String,
    pub teamCode: String,
    pub pointsTotal: u32,
    pub time: f32,
    pub deltaTime: f32,
}

impl TpvResultsTeam {
    #[allow(dead_code)]
    fn new() -> TpvResultsTeam {
        TpvResultsTeam {
            location: 0,
            position: 0,
            team: String::from("--"),
            teamCode: String::from("--"),
            pointsTotal: 0,
            time: 0.0,
            deltaTime: 0.0,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum DataSourceStatus {
    Unknown,
    Ok,
    NotOk,
}

#[derive(Clone)]
pub struct DataSource {
    pub started: bool,
    pub stopped: bool,
    pub status: DataSourceStatus,
    pub frame: u64,
}

impl DataSource {
    pub fn new() -> DataSource {
        DataSource {
            started: false,
            stopped: true,
            status: DataSourceStatus::Unknown,
            frame: 0,
        }
    }
}

#[derive(Clone)]
pub struct DataCollector {
    base_uri: String,
    // TPV raw 'focus' data
    source_focus: Arc<Mutex<DataSource>>, 
    data_focus: Arc<Mutex<TpvFocus>>,
    // TPV raw 'nearest' data
    source_nearest: Arc<Mutex<DataSource>>, 
    data_nearest: Arc<Mutex<Vec<TpvNearest>>>,
    // TPV raw 'event' data
    source_event: Arc<Mutex<DataSource>>,
    data_event: Arc<Mutex<TpvEvent>>,
    // TPV raw 'entries' data
    source_entries: Arc<Mutex<DataSource>>,
    data_entries: Arc<Mutex<Vec<TpvEntries>>>,
    // TPV  raw 'groups' data
    source_groups: Arc<Mutex<DataSource>>,
    data_groups: Arc<Mutex<Vec<TpvGroups>>>,
    // TPV  raw 'resultsIndv' data
    source_results_indv: Arc<Mutex<DataSource>>,
    data_results_indv: Arc<Mutex<Vec<TpvResultsIndv>>>,
    // TPV  raw 'resultsTeam' data
    source_results_team: Arc<Mutex<DataSource>>,
    data_results_team: Arc<Mutex<Vec<TpvResultsTeam>>>,
}

impl DataCollector {
    pub fn new() -> DataCollector {
        DataCollector {
            base_uri: String::from("http://localhost:8080"),
            source_focus:  Arc::new(Mutex::new(DataSource::new())),
            data_focus:  Arc::new(Mutex::new(TpvFocus::new())),
            source_nearest:  Arc::new(Mutex::new(DataSource::new())),
            data_nearest:  Arc::new(Mutex::new(vec![TpvNearest::new()])),
            source_event:  Arc::new(Mutex::new(DataSource::new())),
            data_event: Arc::new(Mutex::new(TpvEvent::new())),
            source_entries:  Arc::new(Mutex::new(DataSource::new())),
            data_entries: Arc::new(Mutex::new(vec![TpvEntries::new()])),
            source_groups:  Arc::new(Mutex::new(DataSource::new())),
            data_groups: Arc::new(Mutex::new(vec![TpvGroups::new()])),
            source_results_indv:  Arc::new(Mutex::new(DataSource::new())),
            data_results_indv: Arc::new(Mutex::new(vec![TpvResultsIndv::new()])),
            source_results_team:  Arc::new(Mutex::new(DataSource::new())),
            data_results_team: Arc::new(Mutex::new(vec![TpvResultsTeam::new()])),
        }
    }

    fn collect_focus(&self) {
        let source = Arc::clone(&self.source_focus);
        let focus = Arc::clone(&self.data_focus);
        let url = format!("{}/{}", self.base_uri, "bcast/focus");

        thread::spawn(move || {
            set_source_started(&source, true);

            log::info!("DataCollector thread for 'focus' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !is_source_started(&source) {
                    break;
                }

                let (last_status, last_body) = get_data_http(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'focus' data");
                    // failed to get the data
                    update_source_status(&source, DataSourceStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut focus_list: Vec<TpvFocus> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => focus_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'focus' json:\n{focus_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, DataSourceStatus::Ok);
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
            log::info!("DataCollector thread for 'focus' stopped");
            set_source_started(&source, false);
            update_source_status(&source, DataSourceStatus::Unknown);
        });
    }

    fn collect_nearest(&self) {
        let source = Arc::clone(&self.source_nearest);
        let focus = Arc::clone(&self.data_nearest);
        let url = format!("{}/{}", self.base_uri, "bcast/nearest");

        thread::spawn(move || {
            set_source_started(&source, true);

            log::info!("DataCollector thread for 'nearest' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !is_source_started(&source) {
                    break;
                }

                let (last_status, last_body) = get_data_http(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'nearest' data");
                    // failed to get the data
                    update_source_status(&source, DataSourceStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut nearest_list: Vec<TpvNearest> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => nearest_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'nearest' json:\n{nearest_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, DataSourceStatus::Ok);
                    {
                        let mut nearest_locked = focus.lock().unwrap();
                        *nearest_locked = nearest_list;
                    }
                    thread::sleep(time::Duration::from_millis(250));
                }           
            }
            log::info!("DataCollector thread for 'nearest' stopped");
            set_source_started(&source, false);
            update_source_status(&source, DataSourceStatus::Unknown);
        });
    }

    fn collect_event(&self) {
        let source = Arc::clone(&self.source_event);
        let event = Arc::clone(&self.data_event);
        let url = format!("{}/{}", self.base_uri, "bcast/event");

        thread::spawn(move || {
            set_source_started(&source, true);

            log::info!("DataCollector thread for 'event' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !is_source_started(&source) {
                    break;
                }

                let (last_status, last_body) = get_data_http(&status, &body, url.as_str());

                if last_status != 200 {
                    // failed to get the data
                    log::warn!("Failed to retrive 'event' data");
                    update_source_status(&source, DataSourceStatus::NotOk);
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
                    update_source_status(&source, DataSourceStatus::Ok);
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
            log::info!("DataCollector thread for 'event' stopped");
            set_source_started(&source, false);
            update_source_status(&source, DataSourceStatus::Unknown);            
        });
    }

    fn collect_entries(&self) {
        let source = Arc::clone(&self.source_entries);
        let focus = Arc::clone(&self.data_entries);
        let url = format!("{}/{}", self.base_uri, "bcast/entries");

        thread::spawn(move || {
            set_source_started(&source, true);

            log::info!("DataCollector thread for 'entries' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {        
                if !is_source_started(&source) {
                    break;
                }

                let (last_status, last_body) = get_data_http(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'entries' data");
                    // failed to get the data
                    update_source_status(&source, DataSourceStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut entries_list: Vec<TpvEntries> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => entries_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'entries' json:\n{entries_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, DataSourceStatus::Ok);
                    {
                        let mut entries_locked = focus.lock().unwrap();
                        *entries_locked = entries_list;
                    }
                    thread::sleep(time::Duration::from_millis(5000));
                }           
            }
            log::info!("DataCollector thread for 'entries' stopped");
            set_source_started(&source, false);
            update_source_status(&source, DataSourceStatus::Unknown);            
        });
    }

    fn collect_groups(&self) {
        let source = Arc::clone(&self.source_groups);
        let focus = Arc::clone(&self.data_groups);
        let url = format!("{}/{}", self.base_uri, "bcast/groups");

        thread::spawn(move || {
            set_source_started(&source, true);

            log::info!("DataCollector thread for 'groups' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {  
                if !is_source_started(&source) {
                    break;
                }

                let (last_status, last_body) = get_data_http(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'groups' data");
                    // failed to get the data
                    update_source_status(&source, DataSourceStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let last_body = last_body.replace(": null,", ": \"\",");
                    let mut groups_list: Vec<TpvGroups> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => groups_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'groups' json:\n{groups_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, DataSourceStatus::Ok);
                    {
                        let mut groups_locked = focus.lock().unwrap();
                        *groups_locked = groups_list;
                    }
                    thread::sleep(time::Duration::from_millis(5000));
                }           
            }
            log::info!("DataCollector thread for 'groups' stopped");
            set_source_started(&source, false);
            update_source_status(&source, DataSourceStatus::Unknown);
        });
    }

    fn collect_results_indv(&self) {
        let source = Arc::clone(&self.source_results_indv);
        let focus = Arc::clone(&self.data_results_indv);
        let url = format!("{}/{}", self.base_uri, "bcast/resultsIndv");

        thread::spawn(move || {
            set_source_started(&source, true);

            log::info!("DataCollector thread for 'results_indv' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {       
                if !is_source_started(&source) {
                    break;
                }

                let (last_status, last_body) = get_data_http(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'results_indv' data");
                    // failed to get the data
                    update_source_status(&source, DataSourceStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut results_indv_list: Vec<TpvResultsIndv> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => results_indv_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'results_indv' json:\n{results_indv_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, DataSourceStatus::Ok);
                    {
                        let mut results_indv_locked = focus.lock().unwrap();
                        *results_indv_locked = results_indv_list;
                    }
                    thread::sleep(time::Duration::from_millis(5000));
                }           
            }
            log::info!("DataCollector thread for 'results_indv' stopped");
            set_source_started(&source, false);
            update_source_status(&source, DataSourceStatus::Unknown);
        });
    }

    fn collect_results_team(&self) {
        let source = Arc::clone(&self.source_results_team);
        let focus = Arc::clone(&self.data_results_team);
        let url = format!("{}/{}", self.base_uri, "bcast/resultsTeam");

        thread::spawn(move || {
            set_source_started(&source, true);

            log::info!("DataCollector thread for 'results_team' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {      
                if !is_source_started(&source) {
                    break;
                }

                let (last_status, last_body) = get_data_http(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'results_team' data");
                    // failed to get the data
                    update_source_status(&source, DataSourceStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut results_team_list: Vec<TpvResultsTeam> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => results_team_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'results_team' json:\n{results_team_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, DataSourceStatus::Ok);
                    {
                        let mut results_team_locked = focus.lock().unwrap();
                        *results_team_locked = results_team_list;
                    }
                    thread::sleep(time::Duration::from_millis(5000));
                }
            }
            log::info!("DataCollector thread for 'results_team' stopped");
            set_source_started(&source, false);
            update_source_status(&source, DataSourceStatus::Unknown);
        });
    }

    pub fn start(&mut self) {
        let s = self.source_focus.lock().unwrap();
        if !s.started && s.stopped {
            self.collect_focus();
        }
        let s = self.source_nearest.lock().unwrap();
        if !s.started && s.stopped  {
            self.collect_nearest();
        }
        let s = self.source_event.lock().unwrap();
        if !s.started && s.stopped  {
            self.collect_event();
        }
        let s = self.source_entries.lock().unwrap();
        if !s.started && s.stopped  {
            self.collect_entries();
        }
        let s = self.source_groups.lock().unwrap();
        if !s.started && s.stopped  {
            self.collect_groups();
        }
        let s = self.source_results_indv.lock().unwrap();
        if !s.started && s.stopped  {
            self.collect_results_indv();
        }
        let s = self.source_results_team.lock().unwrap();
        if !s.started && s.stopped  {
            self.collect_results_team();
        }
    }

    pub fn stop(&self) {
        let mut s = self.source_focus.lock().unwrap();
        if s.started && !s.stopped {
            s.started = false;
        }
        let mut s = self.source_nearest.lock().unwrap();
        if s.started && !s.stopped  {
            s.started = false;
        }
        let mut s = self.source_event.lock().unwrap();
        if s.started && !s.stopped  {
            s.started = false;
        }
        let mut s = self.source_entries.lock().unwrap();
        if s.started && !s.stopped  {
            s.started = false;
        }
        let mut s = self.source_groups.lock().unwrap();
        if s.started && !s.stopped  {
            s.started = false;
        }
        let mut s = self.source_results_indv.lock().unwrap();
        if s.started && !s.stopped  {
            s.started = false;
        }
        let mut s = self.source_results_team.lock().unwrap();
        if s.started && !s.stopped  {
            s.started = false;
        }
    }

    pub fn is_running(&self) -> bool {
        let s = self.source_focus.lock().unwrap();
        if !s.started && s.stopped {
            return false;
        }
        let s = self.source_nearest.lock().unwrap();
        if !s.started && s.stopped {
            return false;
        }
        let s = self.source_event.lock().unwrap();
        if !s.started && s.stopped {
            return false;
        }
        let s = self.source_entries.lock().unwrap();
        if !s.started && s.stopped {
            return false;
        }
        let s = self.source_groups.lock().unwrap();
        if !s.started && s.stopped {
            return false;
        }
        let s = self.source_results_indv.lock().unwrap();
        if !s.started && s.stopped {
            return false;
        }
        let s = self.source_results_team.lock().unwrap();
        if !s.started && s.stopped {
            return false;
        }
        true
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

    pub fn get_source_entries(&self) -> DataSource {
        self.source_entries.lock().unwrap().clone()
    }

    pub fn get_source_groups(&self) -> DataSource {
        self.source_groups.lock().unwrap().clone()
    }

    pub fn get_source_results_indv(&self) -> DataSource {
        self.source_results_indv.lock().unwrap().clone()
    }

    pub fn get_source_results_team(&self) -> DataSource {
        self.source_results_team.lock().unwrap().clone()
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

    pub fn get_entries(&self) -> Vec<TpvEntries> {
        self.data_entries.lock().unwrap().clone()
    }

    pub fn get_groups(&self) -> Vec<TpvGroups> {
        self.data_groups.lock().unwrap().clone()
    }

    pub fn get_results_indv(&self) -> Vec<TpvResultsIndv> {
        self.data_results_indv.lock().unwrap().clone()
    }

    pub fn get_results_team(&self) -> Vec<TpvResultsTeam> {
        self.data_results_team.lock().unwrap().clone()
    }

    pub fn set_base_uri(&mut self, uri: String) {
        self.base_uri = uri;
    }
}

fn update_source_status(source: &Arc<Mutex<DataSource>>, s: DataSourceStatus) {
    let mut source_locked = source.lock().unwrap();
   
    if s == DataSourceStatus::Ok {
        source_locked.frame += 1;
    }
   
    source_locked.status = s;
}

fn set_source_started(source: &Arc<Mutex<DataSource>>, started: bool) {
    let mut source_locked = source.lock().unwrap();
    source_locked.started = started;
    source_locked.stopped = !started;
}

fn is_source_started(source: &Arc<Mutex<DataSource>>) -> bool {
    let source_locked = source.lock().unwrap();
    source_locked.started 
}

fn get_data_http(status: &Arc<Mutex<u16>>, body: &Arc<Mutex<String>>, url: &str) -> (u16, String) {
    let status_clone = Arc::clone(&status);
    let body_clone = Arc::clone(&body);
    let request = ehttp::Request::get(url);
 

    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        let mut status_locked = status_clone.lock().unwrap();
        let mut body_locked = body_clone.lock().unwrap();

        match result {
            Ok(r) => (move |response: &ehttp::Response| {
                *status_locked = response.status;
                match response.text() {
                    Some(s) => (move |body: &str|{
                        *body_locked = String::from(body);
                        // log::info!("{}", *body_locked);
                    })(s),
                    None => (),
                }

            })(&r),
            Err(_) => (move || {
                *status_locked = 0;
            })(),
        }
    });

    let last_status: u16 = *status.lock().unwrap();
    let last_body: String = body.lock().unwrap().clone();

    (last_status, last_body)
}

