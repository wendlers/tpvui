#[derive(Clone, Debug, PartialEq)]
pub struct Zone {
    pub name: String,
    pub from: u32,
    pub to: u32,
}

impl Zone {
    pub fn create(new_name: &str, new_from: u32, new_to: u32) -> Zone {
        Zone {
            name: new_name.to_string(),
            from: new_from,
            to: new_to,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HrZones {
    pub zones: Vec<Zone>,
}

impl HrZones {
    pub fn create_from_threshold(threashold: u32) -> HrZones {
        let mut z: Vec<Zone> = Vec::new();
        let th = threashold as f32;
        // Create 7-zone model from HR threshold
        // Z1	Recovery	          0% -  80%
        // Z2	Aerobic	             81% -  89%
        // Z3	Tempo	             90% -  94%
        // Z4	SubThreshold	     95% -  99%
        // Z5	SuperThreshold	    100% - 102%
        // Z6	Aerobic Capacity	103% - 105%
        // Z7	Anaerobic	        105%+
        let mut from: u32 = 0;
        let mut to: u32 = (th * 0.8) as u32;
        z.push(Zone::create("Recovery", from, to));

        from = to + 1;
        to =  (th * 0.89) as u32;
        z.push(Zone::create("Aerobic", from, to)); 

        from = to + 1;
        to =  (th * 0.94) as u32;
        z.push(Zone::create("Tempo", from, to)); 

        from = to + 1;
        to =  (th * 0.99) as u32;     
        z.push(Zone::create("SubThreshold", from, to));  

        from = to + 1;
        to =  (th * 1.02) as u32;    
        z.push(Zone::create("SuperThreshold", from, to));

        from = to + 1;
        to =  (th * 1.05) as u32;    
        z.push(Zone::create("Aerobic", from, to));

        from = to + 1;
        to =  9999;      
        z.push(Zone::create("Anaerobic", from, to));   

        log::info!("HR Zones:\n{:?}", z);

        HrZones { 
            zones: z,
        }   
    }

    pub fn zone(&self, value: u32) -> u32 {
        let mut n: u32 = 0;

        for z in self.zones.iter() {
            // log::info!("{:?}, n={}", z, n);
            if value >= z.from && value <= z.to {
                return n;
            }
            n += 1;
        }
        self.zones.len() as u32
    }

    pub fn name(&self, value: u32) -> String {
        match self.zones.get(self.zone(value) as usize) {
            Some(s) => s.name.clone(),
            None => String::from("Unknown"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PwrZones {
    pub zones: Vec<Zone>,
}

impl PwrZones {
    pub fn create_from_threshold(threashold: u32) -> PwrZones {
        let mut z: Vec<Zone> = Vec::new();
        let th = threashold as f32;
        // Create 7-zone model from power threshold (FTP)
        // Z1	Active Recovery	  0% -  55%
        // Z2	Endurance	     56% -  75%
        // Z3	Tempo	         76% -  90%
        // Z4	Threshold	     91% - 105%
        // Z5	VO2 Max	        106% - 120%
        // Z6	Anaerobic	    121% - 150%
        // Z7	Neuromuscular	151%+
        let mut from: u32 = 0;
        let mut to: u32 = (th * 0.55) as u32;
        z.push(Zone::create("Recovery", from, to));

        from = to + 1;
        to =  (th * 0.75) as u32;
        z.push(Zone::create("Endurance", from, to)); 

        from = to + 1;
        to =  (th * 0.90) as u32;
        z.push(Zone::create("Tempo", from, to)); 

        from = to + 1;
        to =  (th * 1.05) as u32;     
        z.push(Zone::create("Threshold", from, to));  

        from = to + 1;
        to =  (th * 1.20) as u32;    
        z.push(Zone::create("VO2 Max", from, to));

        from = to + 1;
        to =  (th * 1.50) as u32;    
        z.push(Zone::create("Aerobic", from, to));

        from = to + 1;
        to =  9999;      
        z.push(Zone::create("Neuromuscular", from, to));   

        log::info!("PWR Zones:\n{:?}", z);

        PwrZones { 
            zones: z,
        }
    }

    pub fn zone(&self, value: u32) -> u32 {
        let mut n: u32 = 0;

        for z in self.zones.iter() {
            // log::info!("{:?}, n={}", z, n);
            if value >= z.from && value <= z.to {
                return n;
            }
            n += 1;
        }
        self.zones.len() as u32
    }

    pub fn name(&self, value: u32) -> String {
        match self.zones.get(self.zone(value) as usize) {
            Some(s) => s.name.clone(),
            None => String::from("Unknown"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Athlete {
    pub hr_threshold: u32,
    pub hr_zones: HrZones,
    pub pwr_threshold: u32,
    pub pwr_zones: PwrZones,
    pub weight: f32,
}

impl Athlete {
    pub fn new() -> Athlete {
        Athlete { 
            hr_threshold: 171, 
            hr_zones: HrZones::create_from_threshold(171), 
            pwr_threshold: 200, 
            pwr_zones: PwrZones::create_from_threshold(200),
            weight: 61.0,
        }
    }
}
