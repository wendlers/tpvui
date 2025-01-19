use std::{sync::{Arc, Mutex}, thread, time};

pub mod tpvbc;

#[derive(Clone)]
pub struct DataCollector {
    base_uri: String,
    // TPV raw 'focus' data
    source_focus: Arc<Mutex<tpvbc::BcastState>>, 
    data_focus: Arc<Mutex<tpvbc::Focus>>,
    // TPV raw 'nearest' data
    source_nearest: Arc<Mutex<tpvbc::BcastState>>, 
    data_nearest: Arc<Mutex<Vec<tpvbc::Nearest>>>,
    // TPV raw 'event' data
    source_event: Arc<Mutex<tpvbc::BcastState>>,
    data_event: Arc<Mutex<tpvbc::Event>>,
    // TPV raw 'entries' data
    source_entries: Arc<Mutex<tpvbc::BcastState>>,
    data_entries: Arc<Mutex<Vec<tpvbc::Entries>>>,
    // TPV  raw 'groups' data
    source_groups: Arc<Mutex<tpvbc::BcastState>>,
    data_groups: Arc<Mutex<Vec<tpvbc::Groups>>>,
    // TPV  raw 'resultsIndv' data
    source_results_indv: Arc<Mutex<tpvbc::BcastState>>,
    data_results_indv: Arc<Mutex<Vec<tpvbc::ResultsIndv>>>,
    // TPV  raw 'resultsTeam' data
    source_results_team: Arc<Mutex<tpvbc::BcastState>>,
    data_results_team: Arc<Mutex<Vec<tpvbc::ResultsTeam>>>,
}

impl DataCollector {
    pub fn new() -> DataCollector {
        DataCollector {
            base_uri: String::from("http://localhost:8080"),
            source_focus:  Arc::new(Mutex::new(tpvbc::BcastState::new())),
            data_focus:  Arc::new(Mutex::new(tpvbc::Focus::new())),
            source_nearest:  Arc::new(Mutex::new(tpvbc::BcastState::new())),
            data_nearest:  Arc::new(Mutex::new(vec![tpvbc::Nearest::new()])),
            source_event:  Arc::new(Mutex::new(tpvbc::BcastState::new())),
            data_event: Arc::new(Mutex::new(tpvbc::Event::new())),
            source_entries:  Arc::new(Mutex::new(tpvbc::BcastState::new())),
            data_entries: Arc::new(Mutex::new(vec![tpvbc::Entries::new()])),
            source_groups:  Arc::new(Mutex::new(tpvbc::BcastState::new())),
            data_groups: Arc::new(Mutex::new(vec![tpvbc::Groups::new()])),
            source_results_indv:  Arc::new(Mutex::new(tpvbc::BcastState::new())),
            data_results_indv: Arc::new(Mutex::new(vec![tpvbc::ResultsIndv::new()])),
            source_results_team:  Arc::new(Mutex::new(tpvbc::BcastState::new())),
            data_results_team: Arc::new(Mutex::new(vec![tpvbc::ResultsTeam::new()])),
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
                    update_source_status(&source, tpvbc::BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut focus_list: Vec<tpvbc::Focus> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => focus_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'focus' json:\n{focus_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, tpvbc::BcastStatus::Ok);
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
            update_source_status(&source, tpvbc::BcastStatus::Unknown);
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
                    update_source_status(&source, tpvbc::BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut nearest_list: Vec<tpvbc::Nearest> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => nearest_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'nearest' json:\n{nearest_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, tpvbc::BcastStatus::Ok);
                    {
                        let mut nearest_locked = focus.lock().unwrap();
                        *nearest_locked = nearest_list;
                    }
                    thread::sleep(time::Duration::from_millis(250));
                }           
            }
            log::info!("DataCollector thread for 'nearest' stopped");
            set_source_started(&source, false);
            update_source_status(&source, tpvbc::BcastStatus::Unknown);
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
                    update_source_status(&source, tpvbc::BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    // 'type' is a reserved RUST keyword. Thus we can not have a field named 'type'
                    // in the struct for the 'serde' bindings. So what we do here is to change
                    // 'type' to 'type_' in the received body:
                    let last_body = last_body.replace("\"type\":", "\"type_\":");
                    let mut event_list: Vec<tpvbc::Event> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => event_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'event' json:\n{event_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, tpvbc::BcastStatus::Ok);
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
            update_source_status(&source, tpvbc::BcastStatus::Unknown);            
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
                    update_source_status(&source, tpvbc::BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut entries_list: Vec<tpvbc::Entries> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => entries_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'entries' json:\n{entries_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, tpvbc::BcastStatus::Ok);
                    {
                        let mut entries_locked = focus.lock().unwrap();
                        *entries_locked = entries_list;
                    }
                    thread::sleep(time::Duration::from_millis(5000));
                }           
            }
            log::info!("DataCollector thread for 'entries' stopped");
            set_source_started(&source, false);
            update_source_status(&source, tpvbc::BcastStatus::Unknown);            
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
                    update_source_status(&source, tpvbc::BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let last_body = last_body.replace(": null,", ": \"\",");
                    let mut groups_list: Vec<tpvbc::Groups> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => groups_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'groups' json:\n{groups_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, tpvbc::BcastStatus::Ok);
                    {
                        let mut groups_locked = focus.lock().unwrap();
                        *groups_locked = groups_list;
                    }
                    thread::sleep(time::Duration::from_millis(5000));
                }           
            }
            log::info!("DataCollector thread for 'groups' stopped");
            set_source_started(&source, false);
            update_source_status(&source, tpvbc::BcastStatus::Unknown);
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
                    update_source_status(&source, tpvbc::BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut results_indv_list: Vec<tpvbc::ResultsIndv> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => results_indv_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'results_indv' json:\n{results_indv_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, tpvbc::BcastStatus::Ok);
                    {
                        let mut results_indv_locked = focus.lock().unwrap();
                        *results_indv_locked = results_indv_list;
                    }
                    thread::sleep(time::Duration::from_millis(5000));
                }           
            }
            log::info!("DataCollector thread for 'results_indv' stopped");
            set_source_started(&source, false);
            update_source_status(&source, tpvbc::BcastStatus::Unknown);
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
                    update_source_status(&source, tpvbc::BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut results_team_list: Vec<tpvbc::ResultsTeam> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => results_team_list = obj,
                        Err(_) => () 
                    }
                    
                    log::info!("'results_team' json:\n{results_team_list:#?}");

                    // all good, we got some data
                    update_source_status(&source, tpvbc::BcastStatus::Ok);
                    {
                        let mut results_team_locked = focus.lock().unwrap();
                        *results_team_locked = results_team_list;
                    }
                    thread::sleep(time::Duration::from_millis(5000));
                }
            }
            log::info!("DataCollector thread for 'results_team' stopped");
            set_source_started(&source, false);
            update_source_status(&source, tpvbc::BcastStatus::Unknown);
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

    pub fn get_source_focus(&self) -> tpvbc::BcastState {
        self.source_focus.lock().unwrap().clone()
    }

    pub fn get_source_nearest(&self) -> tpvbc::BcastState {
        self.source_nearest.lock().unwrap().clone()
    }

    pub fn get_source_event(&self) -> tpvbc::BcastState {
        self.source_event.lock().unwrap().clone()
    }

    pub fn get_source_entries(&self) -> tpvbc::BcastState {
        self.source_entries.lock().unwrap().clone()
    }

    pub fn get_source_groups(&self) -> tpvbc::BcastState {
        self.source_groups.lock().unwrap().clone()
    }

    pub fn get_source_results_indv(&self) -> tpvbc::BcastState {
        self.source_results_indv.lock().unwrap().clone()
    }

    pub fn get_source_results_team(&self) -> tpvbc::BcastState {
        self.source_results_team.lock().unwrap().clone()
    }

    pub fn get_focus(&self) -> tpvbc::Focus {
         self.data_focus.lock().unwrap().clone()
    }

    pub fn get_nearest(&self) -> Vec<tpvbc::Nearest> {
        self.data_nearest.lock().unwrap().clone()
    }

    pub fn get_event(&self) -> tpvbc::Event {
        self.data_event.lock().unwrap().clone()
    }

    pub fn get_entries(&self) -> Vec<tpvbc::Entries> {
        self.data_entries.lock().unwrap().clone()
    }

    pub fn get_groups(&self) -> Vec<tpvbc::Groups> {
        self.data_groups.lock().unwrap().clone()
    }

    pub fn get_results_indv(&self) -> Vec<tpvbc::ResultsIndv> {
        self.data_results_indv.lock().unwrap().clone()
    }

    pub fn get_results_team(&self) -> Vec<tpvbc::ResultsTeam> {
        self.data_results_team.lock().unwrap().clone()
    }

    pub fn set_base_uri(&mut self, uri: String) {
        self.base_uri = uri;
    }
}

fn update_source_status(source: &Arc<Mutex<tpvbc::BcastState>>, s: tpvbc::BcastStatus) {
    let mut source_locked = source.lock().unwrap();
   
    if s == tpvbc::BcastStatus::Ok {
        source_locked.frame += 1;
    }
   
    source_locked.status = s;
}

fn set_source_started(source: &Arc<Mutex<tpvbc::BcastState>>, started: bool) {
    let mut source_locked = source.lock().unwrap();
    source_locked.started = started;
    source_locked.stopped = !started;
}

fn is_source_started(source: &Arc<Mutex<tpvbc::BcastState>>) -> bool {
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

pub struct Facade {
    tpv: tpvbc::http::BcastStream,
}

impl Facade {
    pub fn new() -> Facade {
        Facade {
            tpv: tpvbc::http::BcastStream::new(),
        }
    }

    pub fn start(&self) {
        log::info!("Facade::start");
        self.tpv.start();
    }

    pub fn stop(&self) {
        log::info!("Facade::stop");
        self.tpv.stop();
    }

    pub fn running(&self) -> bool {
        self.tpv.running()
    }

    pub fn tpv_focus_data(&self) -> tpvbc::Focus {
        self.tpv.focus.stream.data().clone()
    }

    pub fn tpv_focus_state(&self) -> tpvbc::BcastState {
        self.tpv.focus.stream.state().clone()
    }

    pub fn tpv_nearest_data(&self) -> Vec<tpvbc::Nearest> {
        self.tpv.nearest.stream.data().clone()
    }

    pub fn tpv_nearest_state(&self) -> tpvbc::BcastState {
        self.tpv.nearest.stream.state().clone()
    }

    pub fn tpv_event_data(&self) -> tpvbc::Event {
        self.tpv.event.stream.data().clone()
    }

    pub fn tpv_event_state(&self) -> tpvbc::BcastState {
        self.tpv.event.stream.state().clone()
    }
}
