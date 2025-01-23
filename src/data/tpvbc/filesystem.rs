use std::{sync::Arc, path::Path, sync::mpsc, thread, time, fs};

use crate::data::ride::Ride;

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
        let ride = Arc::clone(&self.stream.ride);
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

                                let mut ride_locked = ride.lock().unwrap();
                                ride_locked.update(focus_locked.clone());
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

pub struct BcastStreamEventWorker {
    pub stream: BcastStreamEvent,
    pub url: String,
}

impl BcastStreamEventWorker {
    pub fn new() -> BcastStreamEventWorker {
        BcastStreamEventWorker {
            stream: BcastStreamEvent::new(),
            url: String::from("file:///home/stefan/devel/tpvbc2http/http/testing/event.json"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamEventWorker::start");
            self.url = url[7..].to_string();
            self.url.push_str("/event.json");
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
        let event = Arc::clone(&self.stream.data);
        let url = self.url.clone();

        thread::spawn(move || {
            <BcastStreamEvent as BcastStreamBase>::set_started_t(&source, true);

            log::info!("Worker thread for 'event' started");

            let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
    
            log::info!("watching file: {}", url);

            // fixme: error handling
            notify::Watcher::watch(&mut watcher, Path::new(&url), notify::RecursiveMode::NonRecursive).unwrap();
            
            loop {
                if !<BcastStreamEvent as BcastStreamBase>::started_t(&source) {
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

                    log::info!("'event' changed");

                    match fs::read_to_string(&url) {
                        Ok(content) => (|c: &str| {
                            // 'type' is a reserved RUST keyword. Thus we can not have a field named 'type'
                            // in the struct for the 'serde' bindings. So what we do here is to change
                            // 'type' to 'type_' in the received body:
                            let c_tmp = c.to_string().replace("\"type\":", "\"type_\":");
                            let mut event_list: Vec<Event> = Vec::new();

                            match serde_json::from_str(&c_tmp) {
                                Ok(obj) => event_list = obj,
                                Err(err) => (|e| {
                                    log::warn!("Faild to deserialize 'event' data: {}", e);
                                })(err),
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
                        })(&content[3..]), // remove utf-8 BOM
                        Err(err) => (|e| {
                            log::warn!("Failed to read 'event': {}", e);
                            <BcastStreamEvent as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                        })(err),
                    }
                }
                thread::sleep(time::Duration::from_millis(250));
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
            url: String::from("file:///home/stefan/devel/tpvbc2http/http/testing/entries.json"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamEntriesWorker::start");
            self.url = url[7..].to_string();
            self.url.push_str("/entries.json");
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

            let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
    
            log::info!("watching file: {}", url);

            // fixme: error handling
            notify::Watcher::watch(&mut watcher, Path::new(&url), notify::RecursiveMode::NonRecursive).unwrap();
            
            loop {
                if !<BcastStreamEntries as BcastStreamBase>::started_t(&source) {
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

                    log::info!("'entries' changed");

                    match fs::read_to_string(&url) {
                        Ok(content) => (|c: &str| {
                            let mut entries_list: Vec<Entries> = Vec::new();

                            match serde_json::from_str(c) {
                                Ok(obj) => entries_list = obj,
                                Err(err) => (|e| {
                                    log::warn!("Faild to deserialize 'entries' data: {}", e);
                                })(err),
                            }
                            
                            log::debug!("'entries' json:\n{entries_list:#?}");

                            // all good, we got some data
                            <BcastStreamEntries as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                            {
                                let mut entries_locked = entries.lock().unwrap();
                                *entries_locked = entries_list;
                            }
                        })(&content[3..]), // remove utf-8 BOM
                        Err(err) => (|e| {
                            log::warn!("Failed to read 'entries': {}", e);
                            <BcastStreamEntries as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                        })(err),
                    }
                }
                thread::sleep(time::Duration::from_millis(250));
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
            url: String::from("file:///home/stefan/devel/tpvbc2http/http/testing/groups.json"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamGroupsWorker::start");
            self.url = url[7..].to_string();
            self.url.push_str("/groups.json");
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

            let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
    
            log::info!("watching file: {}", url);

            // fixme: error handling
            notify::Watcher::watch(&mut watcher, Path::new(&url), notify::RecursiveMode::NonRecursive).unwrap();
            
            loop {
                if !<BcastStreamGroups as BcastStreamBase>::started_t(&source) {
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

                    log::info!("'groups' changed");

                    match fs::read_to_string(&url) {
                        Ok(content) => (|c: &str| {
                            // deal with null value
                            let c_tmp = c.to_string().replace(": null,", ": \"\",");
                            let mut groups_list: Vec<Groups> = Vec::new();

                            match serde_json::from_str(&c_tmp) {
                                Ok(obj) => groups_list = obj,
                                Err(err) => (|e| {
                                    log::warn!("Faild to deserialize 'groups' data: {}", e);
                                })(err),
                            }
                            
                            log::debug!("'groups' json:\n{groups_list:#?}");

                            // all good, we got some data
                            <BcastStreamGroups as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                            {
                                let mut groups_locked = groups.lock().unwrap();
                                *groups_locked = groups_list;
                            }
                        })(&content[3..]), // remove utf-8 BOM
                        Err(err) => (|e| {
                            log::warn!("Failed to read 'groups': {}", e);
                            <BcastStreamGroups as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                        })(err),
                    }
                }
                thread::sleep(time::Duration::from_millis(250));
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
            url: String::from("file:///home/stefan/devel/tpvbc2http/http/testing/resultsIndv.json"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamResultsIndvWorker::start");
            self.url = url[7..].to_string();
            self.url.push_str("/resultsIndv.json");
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

            let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
    
            log::info!("watching file: {}", url);

            // fixme: error handling
            notify::Watcher::watch(&mut watcher, Path::new(&url), notify::RecursiveMode::NonRecursive).unwrap();
            
            loop {
                if !<BcastStreamResultsIndv as BcastStreamBase>::started_t(&source) {
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

                    log::info!("'results_indv' changed");

                    match fs::read_to_string(&url) {
                        Ok(content) => (|c: &str| {
                            let mut results_indv_list: Vec<ResultsIndv> = Vec::new();

                            match serde_json::from_str(c) {
                                Ok(obj) => results_indv_list = obj,
                                Err(err) => (|e| {
                                    log::warn!("Faild to deserialize 'results_indv' data: {}", e);
                                })(err),
                            }
                            
                            log::debug!("'results_indv' json:\n{results_indv_list:#?}");

                            // all good, we got some data
                            <BcastStreamResultsIndv as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                            {
                                let mut results_indv_locked = results_indv.lock().unwrap();
                                *results_indv_locked = results_indv_list;
                            }
                        })(&content[3..]), // remove utf-8 BOM
                        Err(err) => (|e| {
                            log::warn!("Failed to read 'results_indv': {}", e);
                            <BcastStreamResultsIndv as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                        })(err),
                    }
                }
                thread::sleep(time::Duration::from_millis(250));
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
            url: String::from("file:///home/stefan/devel/tpvbc2http/http/testing/resultsTeam.json"),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamResultsTeamWorker::start");
            self.url = url[7..].to_string();
            self.url.push_str("/resultsTeam.json");
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

            let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
    
            log::info!("watching file: {}", url);

            // fixme: error handling
            notify::Watcher::watch(&mut watcher, Path::new(&url), notify::RecursiveMode::NonRecursive).unwrap();
            
            loop {
                if !<BcastStreamResultsTeam as BcastStreamBase>::started_t(&source) {
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

                    log::info!("'results_team' changed");

                    match fs::read_to_string(&url) {
                        Ok(content) => (|c: &str| {
                            let mut results_team_list: Vec<ResultsTeam> = Vec::new();

                            match serde_json::from_str(c) {
                                Ok(obj) => results_team_list = obj,
                                Err(err) => (|e| {
                                    log::warn!("Faild to deserialize 'results_team' data: {}", e);
                                })(err),
                            }
                            
                            log::debug!("'results_team' json:\n{results_team_list:#?}");

                            // all good, we got some data
                            <BcastStreamResultsTeam as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                            {
                                let mut results_team_locked = results_team.lock().unwrap();
                                *results_team_locked = results_team_list;
                            }
                        })(&content[3..]), // remove utf-8 BOM
                        Err(err) => (|e| {
                            log::warn!("Failed to read 'results_team': {}", e);
                            <BcastStreamResultsTeam as BcastStreamBase>::update_state_t(&source, BcastStatus::NotOk);
                        })(err),
                    }
                }
                thread::sleep(time::Duration::from_millis(250));
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

    fn ride(&self) -> Ride {
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