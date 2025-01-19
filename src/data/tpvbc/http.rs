use std::{sync::{Arc, Mutex}, thread, time};
use super::{BcastStatus, BcastStreamBase, BcastStreamEvent, BcastStreamFocus, BcastStreamNearest, Focus, Nearest, Event};

fn http_get_blocking(status: &Arc<Mutex<u16>>, body: &Arc<Mutex<String>>, url: &str) -> (u16, String) {
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

    pub fn start(&self) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamFocusWorker::start");
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
                    
                    log::info!("'focus' json:\n{focus_list:#?}");

                    // all good, we got some data
                    <BcastStreamFocus as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
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

    pub fn start(&self) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamNearestWorker::start");
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
                    
                    log::info!("'nearest' json:\n{nearest_list:#?}");

                    // all good, we got some data
                    <BcastStreamNearest as BcastStreamBase>::update_state_t(&source, BcastStatus::Ok);
                    {
                        let mut nearest_locked = nearest.lock().unwrap();
                        *nearest_locked = nearest_list;
                    }
                    thread::sleep(time::Duration::from_millis(250));
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

    pub fn start(&self) {
        if self.stream.stopped() && !self.stream.started() {
            log::info!("BcastStreamEventWorker::start");
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
                    
                    log::info!("'event' json:\n{event_list:#?}");

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

pub struct BcastStream {
    pub focus: BcastStreamFocusWorker,
    pub nearest: BcastStreamNearestWorker,
    pub event: BcastStreamEventWorker,
}

impl BcastStream {
    pub fn new() -> BcastStream {
        BcastStream {
            focus: BcastStreamFocusWorker::new(),
            nearest: BcastStreamNearestWorker::new(),
            event: BcastStreamEventWorker::new(),
        }
    }

    pub fn start(&self) {
        log::info!("BcastStream::start");
        self.focus.start();
        self.nearest.start();
        self.event.start();
    }

    pub fn stop(&self) {
        log::info!("BcastStream::stop");
        self.focus.stop();
        self.nearest.stop();
        self.event.stop();
    }

    pub fn running(&self) -> bool {
        self.focus.running() & self.nearest.running() & self.event.running()
    }
}
