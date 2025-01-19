use std::sync::{Arc, Mutex};
use serde::Deserialize;

pub mod http;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct Focus {
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

impl Focus {
    pub(crate) fn new() -> Focus {
        Focus {
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
pub struct Nearest {
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

impl Nearest {
    pub(crate) fn new() -> Nearest {
        Nearest {
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
pub struct Event{
    pub name: String,
    pub route: String,
    pub laps: u32,
    pub distance: u32,
    pub height: u32,
    pub locations: u32,
    pub type_: String, 
}

impl Event {
    pub(crate) fn new() -> Event {
        Event {
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
pub struct Entries {
    pub bibNum: u32,
    pub name: String,
    pub country: String,
    pub team: String,
    pub teamCode: String,    
}

impl Entries {
    pub(crate) fn new() -> Entries {
        Entries {
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
pub struct Groups {
    pub groupNum1: u32,
    pub groupNum2: u32,
    pub leader: String,
    pub size: u32,
    pub timeGap1: i32,
    pub timeGap2: i32,
    pub isPeloton: bool,
}

impl Groups {
    pub(crate) fn new() -> Groups {
        Groups {
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
pub struct ResultsIndv {
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

impl ResultsIndv {
    pub(crate) fn new() -> ResultsIndv {
        ResultsIndv {
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
pub struct ResultsTeam {
    pub location: u32,
    pub position: u32,
    pub team: String,
    pub teamCode: String,
    pub pointsTotal: u32,
    pub time: f32,
    pub deltaTime: f32,
}

impl ResultsTeam {
    pub(crate) fn new() -> ResultsTeam {
        ResultsTeam {
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
pub enum BcastStatus {
    Unknown,
    Ok,
    NotOk,
}

#[derive(Clone)]
pub struct BcastState {
    pub started: bool,
    pub stopped: bool,
    pub status: BcastStatus,
    pub frame: u64,
}

impl BcastState {
    pub fn new() -> BcastState {
        BcastState {
            started: false,
            stopped: true,
            status: BcastStatus::Unknown,
            frame: 0,
        }
    }
}

pub trait BcastStreamBase {
    fn set_started_t(state: &Arc<Mutex<BcastState>>, started: bool) {
        let mut state_locked = state.lock().unwrap();

        state_locked.started = started;
        state_locked.stopped = !started;
    }


    fn started_t(state: &Arc<Mutex<BcastState>>) -> bool {
        let state_locked = state.lock().unwrap();
        state_locked.started 
    }

    fn update_state_t(state: &Arc<Mutex<BcastState>>, new_state: BcastStatus) {
        let mut state_locked = state.lock().unwrap();
       
        if new_state == BcastStatus::Ok {
            state_locked.frame += 1;
        }
       
        state_locked.status = new_state;
    }


    fn set_started(&self, started: bool);

    fn started(&self) -> bool;

    fn stopped(&self) -> bool;
}

pub struct BcastStreamFocus {
    state: Arc<Mutex<BcastState>>,
    data: Arc<Mutex<Focus>>,
}

impl BcastStreamBase for BcastStreamFocus {
    fn set_started(&self, started: bool) {
        <BcastStreamFocus as BcastStreamBase>::set_started_t(&self.state, started);
    }

    fn started(&self) -> bool {
        <BcastStreamFocus as BcastStreamBase>::started_t(&self.state)
    }

    fn stopped(&self) -> bool {
        let state_locked = self.state.lock().unwrap();
        state_locked.stopped 
    }
}

impl BcastStreamFocus {
    pub fn new() -> BcastStreamFocus {
        BcastStreamFocus {
            state: Arc::new(Mutex::new(BcastState::new())),
            data: Arc::new(Mutex::new(Focus::new())),
        }
    }

    pub fn data(&self) -> Focus {
        let data_locked = self.data.lock().unwrap();
        data_locked.clone()
    }

    pub fn state(&self) -> BcastState {
        let state_locked = self.state.lock().unwrap();
        state_locked.clone()
    }
}

pub struct BcastStreamNearest {
    state: Arc<Mutex<BcastState>>,
    data: Arc<Mutex<Vec<Nearest>>>,
}

impl BcastStreamBase for BcastStreamNearest {
    fn set_started(&self, started: bool) {
        <BcastStreamNearest as BcastStreamBase>::set_started_t(&self.state, started);
    }

    fn started(&self) -> bool {
        <BcastStreamNearest as BcastStreamBase>::started_t(&self.state)
    }

    fn stopped(&self) -> bool {
        let state_locked = self.state.lock().unwrap();
        state_locked.stopped 
    }
}

impl BcastStreamNearest {
    pub fn new() -> BcastStreamNearest {
        BcastStreamNearest {
            state: Arc::new(Mutex::new(BcastState::new())),
            data: Arc::new(Mutex::new(vec![Nearest::new()])),
        }
    }

    pub fn data(&self) -> Vec<Nearest> {
        let data_locked = self.data.lock().unwrap();
        data_locked.clone()
    }

    pub fn state(&self) -> BcastState {
        let state_locked = self.state.lock().unwrap();
        state_locked.clone()
    }
}

pub struct BcastStreamEvent {
    state: Arc<Mutex<BcastState>>,
    data: Arc<Mutex<Event>>,
}

impl BcastStreamBase for BcastStreamEvent {
    fn set_started(&self, started: bool) {
        <BcastStreamEvent as BcastStreamBase>::set_started_t(&self.state, started);
    }

    fn started(&self) -> bool {
        <BcastStreamEvent as BcastStreamBase>::started_t(&self.state)
    }

    fn stopped(&self) -> bool {
        let state_locked = self.state.lock().unwrap();
        state_locked.stopped 
    }
}

impl BcastStreamEvent {
    pub fn new() -> BcastStreamEvent {
        BcastStreamEvent {
            state: Arc::new(Mutex::new(BcastState::new())),
            data: Arc::new(Mutex::new(Event::new())),
        }
    }

    pub fn data(&self) -> Event {
        let data_locked = self.data.lock().unwrap();
        data_locked.clone()
    }

    pub fn state(&self) -> BcastState {
        let state_locked = self.state.lock().unwrap();
        state_locked.clone()
    }
}

pub struct BcastStreamEntries {
    state: Arc<Mutex<BcastState>>,
    data: Arc<Mutex<Vec<Entries>>>,
}

impl BcastStreamBase for BcastStreamEntries {
    fn set_started(&self, started: bool) {
        <BcastStreamEntries as BcastStreamBase>::set_started_t(&self.state, started);
    }

    fn started(&self) -> bool {
        <BcastStreamEntries as BcastStreamBase>::started_t(&self.state)
    }

    fn stopped(&self) -> bool {
        let state_locked = self.state.lock().unwrap();
        state_locked.stopped 
    }
}

impl BcastStreamEntries {
    pub fn new() -> BcastStreamEntries {
        BcastStreamEntries {
            state: Arc::new(Mutex::new(BcastState::new())),
            data: Arc::new(Mutex::new(vec![Entries::new()])),
        }
    }

    pub fn data(&self) -> Vec<Entries> {
        let data_locked = self.data.lock().unwrap();
        data_locked.clone()
    }

    pub fn state(&self) -> BcastState {
        let state_locked = self.state.lock().unwrap();
        state_locked.clone()
    }
}

pub struct BcastStreamGroups {
    state: Arc<Mutex<BcastState>>,
    data: Arc<Mutex<Vec<Groups>>>,
}

impl BcastStreamBase for BcastStreamGroups {
    fn set_started(&self, started: bool) {
        <BcastStreamGroups as BcastStreamBase>::set_started_t(&self.state, started);
    }

    fn started(&self) -> bool {
        <BcastStreamGroups as BcastStreamBase>::started_t(&self.state)
    }

    fn stopped(&self) -> bool {
        let state_locked = self.state.lock().unwrap();
        state_locked.stopped 
    }
}

impl BcastStreamGroups {
    pub fn new() -> BcastStreamGroups {
        BcastStreamGroups {
            state: Arc::new(Mutex::new(BcastState::new())),
            data: Arc::new(Mutex::new(vec![Groups::new()])),
        }
    }

    pub fn data(&self) -> Vec<Groups> {
        let data_locked = self.data.lock().unwrap();
        data_locked.clone()
    }

    pub fn state(&self) -> BcastState {
        let state_locked = self.state.lock().unwrap();
        state_locked.clone()
    }
}

pub struct BcastStreamResultsIndv {
    state: Arc<Mutex<BcastState>>,
    data: Arc<Mutex<Vec<ResultsIndv>>>,
}

impl BcastStreamBase for BcastStreamResultsIndv {
    fn set_started(&self, started: bool) {
        <BcastStreamResultsIndv as BcastStreamBase>::set_started_t(&self.state, started);
    }

    fn started(&self) -> bool {
        <BcastStreamResultsIndv as BcastStreamBase>::started_t(&self.state)
    }

    fn stopped(&self) -> bool {
        let state_locked = self.state.lock().unwrap();
        state_locked.stopped 
    }
}

impl BcastStreamResultsIndv {
    pub fn new() -> BcastStreamResultsIndv {
        BcastStreamResultsIndv {
            state: Arc::new(Mutex::new(BcastState::new())),
            data: Arc::new(Mutex::new(vec![ResultsIndv::new()])),
        }
    }

    pub fn data(&self) -> Vec<ResultsIndv> {
        let data_locked = self.data.lock().unwrap();
        data_locked.clone()
    }

    pub fn state(&self) -> BcastState {
        let state_locked = self.state.lock().unwrap();
        state_locked.clone()
    }
}

pub struct BcastStreamResultsTeam {
    state: Arc<Mutex<BcastState>>,
    data: Arc<Mutex<Vec<ResultsTeam>>>,
}

impl BcastStreamBase for BcastStreamResultsTeam {
    fn set_started(&self, started: bool) {
        <BcastStreamResultsTeam as BcastStreamBase>::set_started_t(&self.state, started);
    }

    fn started(&self) -> bool {
        <BcastStreamResultsTeam as BcastStreamBase>::started_t(&self.state)
    }

    fn stopped(&self) -> bool {
        let state_locked = self.state.lock().unwrap();
        state_locked.stopped 
    }
}

impl BcastStreamResultsTeam {
    pub fn new() -> BcastStreamResultsTeam {
        BcastStreamResultsTeam {
            state: Arc::new(Mutex::new(BcastState::new())),
            data: Arc::new(Mutex::new(vec![ResultsTeam::new()])),
        }
    }

    pub fn data(&self) -> Vec<ResultsTeam> {
        let data_locked = self.data.lock().unwrap();
        data_locked.clone()
    }

    pub fn state(&self) -> BcastState {
        let state_locked = self.state.lock().unwrap();
        state_locked.clone()
    }
}
