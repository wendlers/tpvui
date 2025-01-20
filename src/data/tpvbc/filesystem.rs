use std::{sync::Arc, path::Path, sync::mpsc, thread, time, fs};

use super::{
    interface::BcastStreamIf, 
    BcastState, 
    BcastStatus, 
    BcastStreamBase, 
    // BcastStreamEntries, 
    // BcastStreamEvent, 
    BcastStreamFocus, 
    // BcastStreamGroups, 
    BcastStreamNearest, 
    // BcastStreamResultsIndv, 
    // BcastStreamResultsTeam, 
    Entries, 
    Event, 
    Focus, 
    Groups, 
    Nearest, 
    ResultsIndv, 
    ResultsTeam
};

pub struct BcastStreamFocusWorker {
    pub stream: BcastStreamFocus,
    pub url: String,
}

impl BcastStreamFocusWorker {
    pub fn new() -> BcastStreamFocusWorker {
        BcastStreamFocusWorker {
            stream: BcastStreamFocus::new(),
            url: String::from("file:///home/stefan/devel/tpvbc2http/http/testing/focus.json"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamFocusWorker::start");
            self.url = url[7..].to_string();
            self.url.push_str("/focus.json");
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
        let url = self.url.clone();

        thread::spawn(move || {
            <BcastStreamFocus as BcastStreamBase>::set_started_t(&source, true);

            log::info!("Worker thread for 'focus' started");

            let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
    
            log::info!("watching file: {}", url);

            // fixme: error handling
            notify::Watcher::watch(&mut watcher, Path::new(&url), notify::RecursiveMode::NonRecursive).unwrap();
            
            loop {
                if !<BcastStreamFocus as BcastStreamBase>::started_t(&source) {
                    break;
                }

                let changed: bool;

                match rx.recv_timeout(time::Duration::from_millis(1000)) {
                    Ok(_) => changed = true,
                    Err(_) => changed = false,
                }
        
                if changed {
                    // flush all the follow up events ...
                    loop {
                        match rx.recv_timeout(time::Duration::from_millis(100)) {
                            Ok(_) => (),
                            Err(_) => break,
                        }
                    }

                    log::info!("'focus' changed");

                    match fs::read_to_string(&url) {
                        Ok(content) => (|c: &str| {
                            let mut focus_list: Vec<Focus> = Vec::new();

                            match serde_json::from_str(c) {
                                Ok(obj) => focus_list = obj,
                                Err(err) => (|e| {
                                    log::warn!("Faild to deserialize 'focus' data: {}", e);
                                })(err),
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
                            }
                        })(&content[3..]), // remove utf-8 BOM
                        Err(err) => (|e| {
                            log::warn!("Failed to read 'focus': {}", e);
                            <BcastStreamFocus as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                        })(err),
                    }
                }
                thread::sleep(time::Duration::from_millis(250));
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
            url: String::from("file:///home/stefan/devel/tpvbc2http/http/testing/nearest.json"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamNearestWorker::start");
            self.url = url[7..].to_string();
            self.url.push_str("/nearest.json");
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

            let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
    
            log::info!("watching file: {}", url);

            // fixme: error handling
            notify::Watcher::watch(&mut watcher, Path::new(&url), notify::RecursiveMode::NonRecursive).unwrap();
            
            loop {
                if !<BcastStreamNearest as BcastStreamBase>::started_t(&source) {
                    break;
                }

                let changed: bool;

                match rx.recv_timeout(time::Duration::from_millis(1000)) {
                    Ok(_) => changed = true,
                    Err(_) => changed = false,
                }
        
                if changed {
                    // flush all the follow up events ...
                    loop {
                        match rx.recv_timeout(time::Duration::from_millis(100)) {
                            Ok(_) => (),
                            Err(_) => break,
                        }
                    }

                    log::info!("'nearest' changed");

                    match fs::read_to_string(&url) {
                        Ok(content) => (|c: &str| {
                            let mut nearest_list: Vec<Nearest> = Vec::new();

                            match serde_json::from_str(c) {
                                Ok(obj) => nearest_list = obj,
                                Err(err) => (|e| {
                                    log::warn!("Faild to deserialize 'nearest' data: {}", e);
                                })(err),
                            }
                            
                            log::debug!("'nearest' json:\n{nearest_list:#?}");

                            // all good, we got some data
                            <BcastStreamNearest as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                            {
                                let mut nearest_locked = nearest.lock().unwrap();
                                *nearest_locked = nearest_list;
                            }
                        })(&content[3..]), // remove utf-8 BOM
                        Err(err) => (|e| {
                            log::warn!("Failed to read 'nearest': {}", e);
                            <BcastStreamNearest as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                        })(err),
                    }
                }
                thread::sleep(time::Duration::from_millis(250));
            }
            log::info!("Worker thread for 'nearest' stopped");
            <BcastStreamNearest as BcastStreamBase>::set_started_t(&source, false);
            <BcastStreamNearest as BcastStreamBase>::update_state_t(&source, BcastStatus::Unknown);
        });
    }
}

pub struct BcastStream {
    pub focus: BcastStreamFocusWorker,
    pub nearest: BcastStreamNearestWorker,
    // pub event: BcastStreamEventWorker,
    // pub entries: BcastStreamEntriesWorker,
    // pub groups: BcastStreamGroupsWorker,
    // pub results_indv: BcastStreamResultsIndvWorker,
    // pub results_team: BcastStreamResultsTeamWorker,
}

impl BcastStreamIf for BcastStream {
    fn start(&mut self, url: String) {
        log::info!("BcastStream::start");
        self.focus.start(url.clone());
        self.nearest.start(url.clone());
        // self.event.start(url.clone());
        // self.entries.start(url.clone());
        // self.groups.start(url.clone());
        // self.results_indv.start(url.clone());
        // self.results_team.start(url.clone());
    }

    fn stop(&self) {
        log::info!("BcastStream::stop");
        self.focus.stop();
        self.nearest.stop();
        // self.event.stop();
        // self.entries.stop();
        // self.groups.stop();
        // self.results_indv.stop();
        // self.results_team.stop();
    }

    fn running(&self) -> bool {
        self.focus.running() &
        self.nearest.running() 
        // self.event.running() &
        // self.entries.running() &
        // self.groups.running() &
        // self.results_indv.running() &
        // self.results_team.running()
    }

    fn focus_data(&self) -> Focus {
        self.focus.stream.data().clone()
    }

    fn focus_state(&self) -> BcastState {
        self.focus.stream.state().clone()
    }

    fn nearest_data(&self) -> Vec<Nearest> {
        self.nearest.stream.data().clone()
    }

    fn nearest_state(&self) -> BcastState {
        self.nearest.stream.state().clone()
    }

    fn event_data(&self) -> Event {
        // self.event.stream.data().clone()
        Event::new()
    }

    fn event_state(&self) -> BcastState {
        // self.event.stream.state().clone()
        BcastState::new()
    }

    fn entries_data(&self) -> Vec<Entries> {
        // self.entries.stream.data().clone()
        vec![Entries::new()]
    }

    fn entries_state(&self) -> BcastState {
        // self.entries.stream.state().clone()
        BcastState::new()
    }

    fn groups_data(&self) -> Vec<Groups> {
        // self.groups.stream.data().clone()
        vec![Groups::new()]
    }

    fn groups_state(&self) -> BcastState {
        // self.groups.stream.state().clone()
        BcastState::new()
    }

    fn results_indv_data(&self) -> Vec<ResultsIndv> {
        // self.results_indv.stream.data().clone()
        vec![ResultsIndv::new()]
    }

    fn results_indv_state(&self) -> BcastState {
        // self.results_indv.stream.state().clone()
        BcastState::new()
    }

    fn results_team_data(&self) -> Vec<ResultsTeam> {
        // self.results_team.stream.data().clone()
        vec![ResultsTeam::new()]
    }

    fn results_team_state(&self) -> BcastState {
        // self.results_team.stream.state().clone()
        BcastState::new()
    }
}

impl BcastStream {
    pub fn new() -> BcastStream {
        BcastStream {
          focus: BcastStreamFocusWorker::new(),
          nearest: BcastStreamNearestWorker::new(),
          //  event: BcastStreamEventWorker::new(),
          //  entries: BcastStreamEntriesWorker::new(),
          //  groups: BcastStreamGroupsWorker::new(),
          //  results_indv: BcastStreamResultsIndvWorker::new(),
          //  results_team: BcastStreamResultsTeamWorker::new(),
        }
    }
}