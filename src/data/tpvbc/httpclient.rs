use std::{sync::{Arc, Mutex}, thread, time};

use super::{
    interface::BcastStreamIf, 
    BcastState, 
    BcastStatus, 
    BcastStreamBase, 
    BcastStreamEntries, 
    BcastStreamEvent, 
    BcastStreamFocus, 
    BcastStreamGroups, 
    BcastStreamNearest, 
    BcastStreamResultsIndv, 
    BcastStreamResultsTeam, 
    Entries, 
    Event, 
    Focus, 
    Groups, 
    Nearest, 
    ResultsIndv, 
    ResultsTeam
};

fn http_get_blocking(status: &Arc<Mutex<u16>>, body: &Arc<Mutex<String>>, url: &str) -> (u16, String) {
    let status_clone = Arc::clone(&status);
    let body_clone = Arc::clone(&body);
    let request = ehttp::Request::get(url);
 
    log::debug!("GET {}", url);

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

pub struct BcastStreamFocusWorker {
    pub stream: BcastStreamFocus,
    pub url: String,
}

impl BcastStreamFocusWorker {
    pub fn new() -> BcastStreamFocusWorker {
        BcastStreamFocusWorker {
            stream: BcastStreamFocus::new(),
            url: String::from("http://localhost:8080/bcast/focus"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamFocusWorker::start");
            self.url = url;
            self.url.push_str("/bcast/focus");
            self.collect();
        }
    }

    pub fn stop(&self) {
        if !self.stream.stopped() && self.stream.started() {
            log::info!("BcastStreamFocusWorker::stop");
            self.stream.set_started(false);
        }
    }

    pub fn running(&self) -> bool {
        self.stream.started()
    }

    fn collect(&self) {
        let source = Arc::clone(&self.stream.state);
        let focus = Arc::clone(&self.stream.data);
        let ride = Arc::clone(&self.stream.ride);
        let url = self.url.clone();

        thread::spawn(move || {
            <BcastStreamFocus as BcastStreamBase>::set_started_t(&source, true);

            log::info!("Worker thread for 'focus' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !<BcastStreamFocus as BcastStreamBase>::started_t(&source) {
                    break;
                }

                let (last_status, last_body) = http_get_blocking(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'focus' data");
                    // failed to get the data
                    <BcastStreamFocus as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut focus_list: Vec<Focus> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => focus_list = obj,
                        Err(_) => () 
                    }
                    
                    log::debug!("'focus' json:\n{focus_list:#?}");

                    // all good, we got some data
                    <BcastStreamFocus as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                    {
                        let mut focus_locked = focus.lock().unwrap();

                        match focus_list.get(0) {
                            Some(f) => *focus_locked = f.clone(),
                            None => (), // empty json array -> ignore
                        }

                        let mut ride_locked = ride.lock().unwrap();
                        ride_locked.update(focus_locked.clone());
                    }
                    thread::sleep(time::Duration::from_millis(250));
                }
            }
            log::info!("Worker thread for 'focus' stopped");
            <BcastStreamFocus as BcastStreamBase>::set_started_t(&source, false);
            <BcastStreamFocus as BcastStreamBase>::update_state_t(&source, BcastStatus::Unknown);
        });
    }
}

pub struct BcastStreamNearestWorker {
    pub stream: BcastStreamNearest,
    pub url: String,
}

impl BcastStreamNearestWorker {
    pub fn new() -> BcastStreamNearestWorker {
        BcastStreamNearestWorker {
            stream: BcastStreamNearest::new(),
            url: String::from("http://localhost:8080/bcast/nearest"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamNearestWorker::start");
            self.url = url;
            self.url.push_str("/bcast/nearest");
            self.collect();
        }
    }

    pub fn stop(&self) {
        if !self.stream.stopped() && self.stream.started() {
            log::info!("BcastStreamNearestWorker::stop");
            self.stream.set_started(false);
        }
    }

    pub fn running(&self) -> bool {
        self.stream.started()
    }

    fn collect(&self) {
        let source = Arc::clone(&self.stream.state);
        let nearest = Arc::clone(&self.stream.data);
        let url = self.url.clone();

        thread::spawn(move || {
            <BcastStreamNearest as BcastStreamBase>::set_started_t(&source, true);

            log::info!("Worker thread for 'nearest' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !<BcastStreamNearest as BcastStreamBase>::started_t(&source) {
                    break;
                }
                let (last_status, last_body) = http_get_blocking(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'nearest' data");
                    // failed to get the data
                    <BcastStreamNearest as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut nearest_list: Vec<Nearest> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => nearest_list = obj,
                        Err(_) => () 
                    }
                    
                    log::debug!("'nearest' json:\n{nearest_list:#?}");

                    // all good, we got some data
                    <BcastStreamNearest as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                    {
                        let mut nearest_locked = nearest.lock().unwrap();
                        *nearest_locked = nearest_list;
                    }
                    thread::sleep(time::Duration::from_millis(1000));
                }           
            }
            log::info!("Worker thread for 'nearest' stopped");
            <BcastStreamNearest as BcastStreamBase>::set_started_t(&source, false);
            <BcastStreamNearest as BcastStreamBase>::update_state_t(&source, BcastStatus::Unknown);
        });
    }
}

pub struct BcastStreamEventWorker {
    pub stream: BcastStreamEvent,
    pub url: String,
}

impl BcastStreamEventWorker {
    pub fn new() -> BcastStreamEventWorker {
        BcastStreamEventWorker {
            stream: BcastStreamEvent::new(),
            url: String::from("http://localhost:8080/bcast/event"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamEventWorker::start");
            self.url = url;
            self.url.push_str("/bcast/event");
            self.collect();
        }
    }

    pub fn stop(&self) {
        if !self.stream.stopped() && self.stream.started() {
            log::info!("BcastStreamEventWorker::stop");
            self.stream.set_started(false);
        }
    }

    pub fn running(&self) -> bool {
        self.stream.started()
    }

    fn collect(&self) {
        let source = Arc::clone(&self.stream.state);
        let event: Arc<Mutex<Event>> = Arc::clone(&self.stream.data);
        let url = self.url.clone();

        thread::spawn(move || {
            <BcastStreamEvent as BcastStreamBase>::set_started_t(&source, true);

            log::info!("Worker thread for 'event' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !<BcastStreamEvent as BcastStreamBase>::started_t(&source) {
                    break;
                }

                let (last_status, last_body) = http_get_blocking(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'event' data");
                    // failed to get the data
                    <BcastStreamEvent as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    // 'type' is a reserved RUST keyword. Thus we can not have a field named 'type'
                    // in the struct for the 'serde' bindings. So what we do here is to change
                    // 'type' to 'type_' in the received body:
                    let last_body = last_body.replace("\"type\":", "\"type_\":");
                    let mut event_list: Vec<Event> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => event_list = obj,
                        Err(_) => () 
                    }
                    
                    log::debug!("'event' json:\n{event_list:#?}");

                    // all good, we got some data
                    <BcastStreamEvent as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                    {
                        let mut event_locked = event.lock().unwrap();

                        match event_list.get(0) {
                            Some(f) => *event_locked = f.clone(),
                            None => (), // empty json array -> ignore
                        }
                    }
                    thread::sleep(time::Duration::from_millis(1000));
                }
            }
            log::info!("Worker thread for 'event' stopped");
            <BcastStreamEvent as BcastStreamBase>::set_started_t(&source, false);
            <BcastStreamEvent as BcastStreamBase>::update_state_t(&source, BcastStatus::Unknown);
        });
    }
}

pub struct BcastStreamEntriesWorker {
    pub stream: BcastStreamEntries,
    pub url: String,
}

impl BcastStreamEntriesWorker {
    pub fn new() -> BcastStreamEntriesWorker {
        BcastStreamEntriesWorker {
            stream: BcastStreamEntries::new(),
            url: String::from("http://localhost:8080/bcast/entries"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamEntriesWorker::start");
            self.url = url;
            self.url.push_str("/bcast/entries");
            self.collect();
        }
    }

    pub fn stop(&self) {
        if !self.stream.stopped() && self.stream.started() {
            log::info!("BcastStreamEntriesWorker::stop");
            self.stream.set_started(false);
        }
    }

    pub fn running(&self) -> bool {
        self.stream.started()
    }

    fn collect(&self) {
        let source = Arc::clone(&self.stream.state);
        let entries = Arc::clone(&self.stream.data);
        let url = self.url.clone();

        thread::spawn(move || {
            <BcastStreamEntries as BcastStreamBase>::set_started_t(&source, true);

            log::info!("Worker thread for 'entries' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !<BcastStreamEntries as BcastStreamBase>::started_t(&source) {
                    break;
                }
                let (last_status, last_body) = http_get_blocking(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'entries' data");
                    // failed to get the data
                    <BcastStreamEntries as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    let mut entries_list: Vec<Entries> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => entries_list = obj,
                        Err(_) => () 
                    }
                    
                    log::debug!("'entries' json:\n{entries_list:#?}");

                    // all good, we got some data
                    <BcastStreamEntries as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                    {
                        let mut entries_locked = entries.lock().unwrap();
                        *entries_locked = entries_list;
                    }
                    thread::sleep(time::Duration::from_millis(1000));
                }           
            }
            log::info!("Worker thread for 'entries' stopped");
            <BcastStreamEntries as BcastStreamBase>::set_started_t(&source, false);
            <BcastStreamEntries as BcastStreamBase>::update_state_t(&source, BcastStatus::Unknown);
        });
    }
}

pub struct BcastStreamGroupsWorker {
    pub stream: BcastStreamGroups,
    pub url: String,
}

impl BcastStreamGroupsWorker {
    pub fn new() -> BcastStreamGroupsWorker {
        BcastStreamGroupsWorker {
            stream: BcastStreamGroups::new(),
            url: String::from("http://localhost:8080/bcast/groups"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamGroupsWorker::start");
            self.url = url;
            self.url.push_str("/bcast/groups");
            self.collect();
        }
    }

    pub fn stop(&self) {
        if !self.stream.stopped() && self.stream.started() {
            log::info!("BcastStreamGroupsWorker::stop");
            self.stream.set_started(false);
        }
    }

    pub fn running(&self) -> bool {
        self.stream.started()
    }

    fn collect(&self) {
        let source = Arc::clone(&self.stream.state);
        let groups = Arc::clone(&self.stream.data);
        let url = self.url.clone();

        thread::spawn(move || {
            <BcastStreamGroups as BcastStreamBase>::set_started_t(&source, true);

            log::info!("Worker thread for 'groups' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !<BcastStreamGroups as BcastStreamBase>::started_t(&source) {
                    break;
                }
                let (last_status, last_body) = http_get_blocking(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'groups' data");
                    // failed to get the data
                    <BcastStreamGroups as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    // deal with null value
                    let last_body = last_body.replace(": null,", ": \"\",");
                    let mut groups_list: Vec<Groups> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => groups_list = obj,
                        Err(_) => () 
                    }
                    
                    log::debug!("'groups' json:\n{groups_list:#?}");

                    // all good, we got some data
                    <BcastStreamGroups as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                    {
                        let mut groups_locked = groups.lock().unwrap();
                        *groups_locked = groups_list;
                    }
                    thread::sleep(time::Duration::from_millis(1000));
                }           
            }
            log::info!("Worker thread for 'groups' stopped");
            <BcastStreamGroups as BcastStreamBase>::set_started_t(&source, false);
            <BcastStreamGroups as BcastStreamBase>::update_state_t(&source, BcastStatus::Unknown);
        });
    }
}

pub struct BcastStreamResultsIndvWorker {
    pub stream: BcastStreamResultsIndv,
    pub url: String,
}

impl BcastStreamResultsIndvWorker {
    pub fn new() -> BcastStreamResultsIndvWorker {
        BcastStreamResultsIndvWorker {
            stream: BcastStreamResultsIndv::new(),
            url: String::from("http://localhost:8080/bcast/resultsIndv"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamResultsIndvWorker::start");
            self.url = url;
            self.url.push_str("/bcast/resultsIndv");
            self.collect();
        }
    }

    pub fn stop(&self) {
        if !self.stream.stopped() && self.stream.started() {
            log::info!("BcastStreamResultsIndvWorker::stop");
            self.stream.set_started(false);
        }
    }

    pub fn running(&self) -> bool {
        self.stream.started()
    }

    fn collect(&self) {
        let source = Arc::clone(&self.stream.state);
        let results_indv = Arc::clone(&self.stream.data);
        let url = self.url.clone();

        thread::spawn(move || {
            <BcastStreamResultsIndv as BcastStreamBase>::set_started_t(&source, true);

            log::info!("Worker thread for 'results_indv' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !<BcastStreamResultsIndv as BcastStreamBase>::started_t(&source) {
                    break;
                }
                let (last_status, last_body) = http_get_blocking(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'results_indv' data");
                    // failed to get the data
                    <BcastStreamResultsIndv as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    // deal with null value
                    let last_body = last_body.replace(": null,", ": \"\",");
                    let mut results_indv_list: Vec<ResultsIndv> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => results_indv_list = obj,
                        Err(_) => () 
                    }
                    
                    log::debug!("'results_indv' json:\n{results_indv_list:#?}");

                    // all good, we got some data
                    <BcastStreamResultsIndv as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                    {
                        let mut results_indv_locked = results_indv.lock().unwrap();
                        *results_indv_locked = results_indv_list;
                    }
                    thread::sleep(time::Duration::from_millis(1000));
                }           
            }
            log::info!("Worker thread for 'results_indv' stopped");
            <BcastStreamResultsIndv as BcastStreamBase>::set_started_t(&source, false);
            <BcastStreamResultsIndv as BcastStreamBase>::update_state_t(&source, BcastStatus::Unknown);
        });
    }
}

pub struct BcastStreamResultsTeamWorker {
    pub stream: BcastStreamResultsTeam,
    pub url: String,
}

impl BcastStreamResultsTeamWorker {
    pub fn new() -> BcastStreamResultsTeamWorker {
        BcastStreamResultsTeamWorker {
            stream: BcastStreamResultsTeam::new(),
            url: String::from("http://localhost:8080/bcast/resultsTeam"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamResultsTeamWorker::start");
            self.url = url;
            self.url.push_str("/bcast/resultsTeam");
            self.collect();
        }
    }

    pub fn stop(&self) {
        if !self.stream.stopped() && self.stream.started() {
            log::info!("BcastStreamResultsTeamWorker::stop");
            self.stream.set_started(false);
        }
    }

    pub fn running(&self) -> bool {
        self.stream.started()
    }

    fn collect(&self) {
        let source = Arc::clone(&self.stream.state);
        let results_team = Arc::clone(&self.stream.data);
        let url = self.url.clone();

        thread::spawn(move || {
            <BcastStreamResultsTeam as BcastStreamBase>::set_started_t(&source, true);

            log::info!("Worker thread for 'results_team' started");

            let status:Arc<Mutex<u16>>  = Arc::new(Mutex::new(0));
            let body:Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));

            loop {
                if !<BcastStreamResultsTeam as BcastStreamBase>::started_t(&source) {
                    break;
                }
                let (last_status, last_body) = http_get_blocking(&status, &body, url.as_str());

                if last_status != 200 {
                    log::warn!("Failed to retrive 'results_team' data");
                    // failed to get the data
                    <BcastStreamResultsTeam as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    // deal with null value
                    let last_body = last_body.replace(": null,", ": \"\",");
                    let mut results_team_list: Vec<ResultsTeam> = Vec::new();

                    match serde_json::from_str(&last_body.as_str()) {
                        Ok(obj) => results_team_list = obj,
                        Err(_) => () 
                    }
                    
                    log::debug!("'results_team' json:\n{results_team_list:#?}");

                    // all good, we got some data
                    <BcastStreamResultsTeam as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                    {
                        let mut results_team_locked = results_team.lock().unwrap();
                        *results_team_locked = results_team_list;
                    }
                    thread::sleep(time::Duration::from_millis(1000));
                }           
            }
            log::info!("Worker thread for 'results_team' stopped");
            <BcastStreamResultsTeam as BcastStreamBase>::set_started_t(&source, false);
            <BcastStreamResultsTeam as BcastStreamBase>::update_state_t(&source, BcastStatus::Unknown);
        });
    }
}

pub struct BcastStream {
    pub focus: BcastStreamFocusWorker,
    pub nearest: BcastStreamNearestWorker,
    pub event: BcastStreamEventWorker,
    pub entries: BcastStreamEntriesWorker,
    pub groups: BcastStreamGroupsWorker,
    pub results_indv: BcastStreamResultsIndvWorker,
    pub results_team: BcastStreamResultsTeamWorker,
}

impl BcastStreamIf for BcastStream {
    fn start(&mut self, url: String) {
        log::info!("BcastStream::start");
        self.focus.start(url.clone());
        self.nearest.start(url.clone());
        self.event.start(url.clone());
        self.entries.start(url.clone());
        self.groups.start(url.clone());
        self.results_indv.start(url.clone());
        self.results_team.start(url.clone());
    }

    fn stop(&self) {
        log::info!("BcastStream::stop");
        self.focus.stop();
        self.nearest.stop();
        self.event.stop();
        self.entries.stop();
        self.groups.stop();
        self.results_indv.stop();
        self.results_team.stop();
    }

    fn running(&self) -> bool {
        self.focus.running() & 
        self.nearest.running() & 
        self.event.running() &
        self.entries.running() &
        self.groups.running() &
        self.results_indv.running() &
        self.results_team.running()
    }

    fn focus_data(&self) -> Focus {
        self.focus.stream.data()
    }

    fn focus_state(&self) -> BcastState {
        self.focus.stream.state()
    }

    fn nearest_data(&self) -> Vec<Nearest> {
        self.nearest.stream.data()
    }

    fn nearest_state(&self) -> BcastState {
        self.nearest.stream.state()
    }

    fn event_data(&self) -> Event {
        self.event.stream.data()
    }

    fn event_state(&self) -> BcastState {
        self.event.stream.state()
    }

    fn entries_data(&self) -> Vec<Entries> {
        self.entries.stream.data()
    }

    fn entries_state(&self) -> BcastState {
        self.entries.stream.state()
    }

    fn groups_data(&self) -> Vec<Groups> {
        self.groups.stream.data()
    }

    fn groups_state(&self) -> BcastState {
        self.groups.stream.state()
    }

    fn results_indv_data(&self) -> Vec<ResultsIndv> {
        self.results_indv.stream.data()
    }

    fn results_indv_state(&self) -> BcastState {
        self.results_indv.stream.state()
    }

    fn results_team_data(&self) -> Vec<ResultsTeam> {
        self.results_team.stream.data()
    }

    fn results_team_state(&self) -> BcastState {
        self.results_team.stream.state()
    }

    fn ride(&self) -> crate::data::ride::Ride {
        self.focus.stream.ride()
    }
}

impl BcastStream {
    pub fn new() -> BcastStream {
        BcastStream {
            focus: BcastStreamFocusWorker::new(),
            nearest: BcastStreamNearestWorker::new(),
            event: BcastStreamEventWorker::new(),
            entries: BcastStreamEntriesWorker::new(),
            groups: BcastStreamGroupsWorker::new(),
            results_indv: BcastStreamResultsIndvWorker::new(),
            results_team: BcastStreamResultsTeamWorker::new(),
        }
    }
}