#[derive(Clone, PartialEq)]
pub struct Speed {
    pub cur: f32,
    pub max: f32,
    pub avg: f32,
    first: bool,
}

impl Speed {
    pub fn new() -> Speed {
        Speed { 
            cur: 0.0,  
            max: 0.0, 
            avg: 0.0, 
            first: true,
        }
    }

    fn update(&mut self, focus: &super::tpvbc::Focus) {
        self.cur = focus.speed as f32  / 275.0;

        if self.first || self.cur > self.max {
            self.max = self.cur;
        }

        self.avg = ((focus.distance as f32) / (focus.time as f32)) * 3.6; 

        self.first = false;
    }
}

#[derive(Clone, PartialEq)]
pub struct HearRate {
    pub cur: u32,
    pub min: u32,
    pub max: u32,
    pub avg: u32,
    first: bool,
}

impl HearRate {
    pub fn new() -> HearRate {
        HearRate { 
            cur:  0,
            min:  0,  
            max:  0, 
            avg:  0,
            first: true, 
        }
    }

    fn update(&mut self, focus: &super::tpvbc::Focus) {
        self.cur = focus.heartrate;

        if self.first || self.cur < self.min {
            self.min = self.cur;
        }

        if self.first || self.cur > self.max {
            self.max = self.cur;
        }

        self.avg = focus.avgHeartrate;

        self.first = false;
    }
}

#[derive(Clone, PartialEq)]
pub struct Cadence {
    pub cur: u32,
    pub max: u32,
    pub avg: u32,
    first: bool,
}

impl Cadence {
    pub fn new() -> Cadence {
        Cadence { 
            cur: 0,  
            max: 0, 
            avg: 0, 
            first: true,
        }
    }

    fn update(&mut self, focus: &super::tpvbc::Focus) {
        self.cur = focus.cadence;

        if self.first || self.cur > self.max {
            self.max = self.cur;
        }

        self.avg = focus.avgCadence; 

        self.first = false;
    }
}

#[derive(Clone, PartialEq)]
pub struct Power {
    pub cur: u32,
    pub max: u32,
    pub nrm: u32,
    pub wpk: f32,
    first: bool,
}

impl Power {
    pub fn new() -> Power {
        Power { 
            cur: 0,  
            max: 0, 
            nrm: 0, 
            wpk: 0.0,
            first: true,
        }
    }

    fn update(&mut self, focus: &super::tpvbc::Focus) {
        self.cur = focus.power;

        if self.first || self.cur > self.max {
            self.max = self.cur;
        }

        self.nrm = focus.nrmPower; 
        self.wpk = self.cur as f32 / 61.0;

        self.first = false;
    }
}

#[derive(Clone, PartialEq)]
pub struct Height {
    pub cur: u32,
    pub min: u32,
    pub max: u32,
    pub ascend: u32,
    pub descend: u32,
    pub slope: i32,
    prev: u32,
    first: bool,
}

impl Height {
    pub fn new() -> Height {
        Height { 
            cur: 0, 
            min: 0, 
            max: 0, 
            ascend: 0,
            descend: 0, 
            slope: 0,
            prev: 0,
            first: true,
        }
    }

    fn update(&mut self, focus: &super::tpvbc::Focus) {
        self.cur = focus.height;

        if self.first || self.cur > self.max {
            self.max = self.cur;
        }

        if self.first || self.cur < self.min {
            self.min = self.cur;
        }

        if !self.first {
            if self.cur > self.prev {
                self.ascend += self.cur - self.prev;
            } else {
                self.descend += self.prev - self.cur;
            }
        }

        self.slope = focus.slope;
        self.prev = self.cur;
        self.first = false;
    }
}

#[derive(Clone, PartialEq)]
pub struct Wind {
    pub speed: f32,
    pub angle: u32,
    pub draft: u32,
}

impl Wind {
    pub fn new() -> Wind {
        Wind { 
            speed: 0.0,  
            angle: 0, 
            draft: 0,
        }
    }

    fn update(&mut self, focus: &super::tpvbc::Focus) {
        self.speed = focus.windSpeed as f32  / 275.0;
        self.angle = focus.windAngle;
        self.draft = focus.draft;
    }
}

#[derive(Clone, PartialEq)]
pub struct Metrics {
    pub time: u32,
    pub distance: f32,
    pub tss: u32,
    pub calories: u32,
    pub speed: Speed,
    pub hr: HearRate,
    pub cadence: Cadence,
    pub power: Power,
    pub height: Height,
    pub lap: u32,
    pub wind: Wind,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            time: 0,
            distance: 0.0,
            tss: 0,
            calories: 0,
            speed: Speed::new(),
            hr: HearRate::new(),
            cadence: Cadence::new(),
            power: Power::new(),
            height: Height::new(),
            lap: 0,
            wind: Wind::new(),
        }
    }

    pub fn time_hms(&self) -> String {
        let h: u32 = self.time / 60 / 60;
        let m: u32 = (self.time / 60) % 60;
        let s: u32 = self.time % 60;

        format!("{:02}:{:02}:{:02}", h, m, s)
    }
}

#[derive(Clone, PartialEq)]
pub struct Ride {
    pub total: Metrics,
    pub current_lap: Metrics,
    pub past_laps: Vec<Metrics>,
}

impl Ride {
    pub fn new() -> Ride {
        Ride {
            total: Metrics::new(),
            current_lap: Metrics::new(),
            past_laps: Vec::new(),
        }
    }

    pub fn update(&mut self, focus: super::tpvbc::Focus) {
        // make sure we have not seen this data before
        if self.total.time < focus.time {
            self.total.time = focus.time;
            self.total.distance = (focus.distance as f32) / 1000.0;
            self.total.tss = focus.tss;
            self.total.calories = focus.calories;

            self.total.speed.update(&focus);
            self.total.hr.update(&focus); 
            self.total.cadence.update(&focus);
            self.total.power.update(&focus);
            self.total.height.update(&focus);

            if focus.eventLapsDone >= 0 {
                self.total.lap = focus.eventLapsDone as u32 + 1;
            }

            self.total.wind.update(&focus);
        }
    }
}
