use tpvbc::interface::BcastStreamIf;

pub mod ride;
pub mod tpvbc;

#[derive(Clone, PartialEq)]
pub enum BcastMethod {
    HttpClient,
    File,
}

pub struct Facade {
    bcast_emthod: BcastMethod,
    tpv: Box<dyn BcastStreamIf>,
}

impl Facade {
    pub fn new() -> Facade {
        Facade {
            bcast_emthod: BcastMethod::HttpClient,
            tpv: Box::new(tpvbc::httpclient::BcastStream::new()),
        }
    }

    pub fn start(&mut self, url: String) {
        if self.running() {
            log::warn!("Steaming is already running! Stop first!");
            return;
        }

        let bcast_method;

        if url.starts_with("file://") {
            bcast_method = BcastMethod::File;
        } else {
            bcast_method = BcastMethod::HttpClient;
        }

        if self.bcast_emthod != bcast_method {
            if bcast_method == BcastMethod::File {
                log::info!("TPV bcast data is read from FS");
                self.tpv = Box::new(tpvbc::filesystem::BcastStream::new());
            } else {
                log::info!("TPV bcast data is read from HTTP (client)");
                self.tpv = Box::new(tpvbc::httpclient::BcastStream::new());
            }
            self.bcast_emthod = bcast_method;
        }

        log::info!("Facade::start");
        self.tpv.start(url);
    }

    pub fn stop(&self) {
        log::info!("Facade::stop");
        self.tpv.stop();
    }

    pub fn running(&self) -> bool {
        self.tpv.running()
    }

    pub fn tpv_focus_data(&self) -> tpvbc::Focus {
        self.tpv.focus_data()
    }

    pub fn tpv_focus_state(&self) -> tpvbc::BcastState {
        self.tpv.focus_state()
    }

    pub fn tpv_nearest_data(&self) -> Vec<tpvbc::Nearest> {
        self.tpv.nearest_data()
    }

    pub fn tpv_nearest_state(&self) -> tpvbc::BcastState {
        self.tpv.nearest_state()
    }

    pub fn tpv_event_data(&self) -> tpvbc::Event {
        self.tpv.event_data()
    }

    pub fn tpv_event_state(&self) -> tpvbc::BcastState {
        self.tpv.event_state()
    }

    pub fn tpv_entries_data(&self) -> Vec<tpvbc::Entries> {
        self.tpv.entries_data()
    }

    pub fn tpv_entries_state(&self) -> tpvbc::BcastState {
        self.tpv.entries_state()
    }

    pub fn tpv_groups_data(&self) -> Vec<tpvbc::Groups> {
        self.tpv.groups_data()
    }

    pub fn tpv_groups_state(&self) -> tpvbc::BcastState {
        self.tpv.groups_state()
    }

    pub fn tpv_results_indv_data(&self) -> Vec<tpvbc::ResultsIndv> {
        self.tpv.results_indv_data()
    }

    pub fn tpv_results_indv_state(&self) -> tpvbc::BcastState {
        self.tpv.results_indv_state()
    }

    pub fn tpv_results_team_data(&self) -> Vec<tpvbc::ResultsTeam> {
        self.tpv.results_team_data()
    }

    pub fn tpv_results_team_state(&self) -> tpvbc::BcastState {
        self.tpv.results_team_state()
    }

    pub fn ride(&self) -> ride::Ride {
        self.tpv.ride()
    }
}
