pub mod tpvbc;

pub struct Facade {
    tpv: tpvbc::http::BcastStream,
}

impl Facade {
    pub fn new() -> Facade {
        Facade {
            tpv: tpvbc::http::BcastStream::new(),
        }
    }

    pub fn start(&mut self, url: String) {
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

    pub fn tpv_entries_data(&self) -> Vec<tpvbc::Entries> {
        self.tpv.entries.stream.data().clone()
    }

    pub fn tpv_entries_state(&self) -> tpvbc::BcastState {
        self.tpv.entries.stream.state().clone()
    }

    pub fn tpv_groups_data(&self) -> Vec<tpvbc::Groups> {
        self.tpv.groups.stream.data().clone()
    }

    pub fn tpv_groups_state(&self) -> tpvbc::BcastState {
        self.tpv.groups.stream.state().clone()
    }

    pub fn tpv_results_indv_data(&self) -> Vec<tpvbc::ResultsIndv> {
        self.tpv.results_indv.stream.data().clone()
    }

    pub fn tpv_results_indv_state(&self) -> tpvbc::BcastState {
        self.tpv.results_indv.stream.state().clone()
    }

    pub fn tpv_results_team_data(&self) -> Vec<tpvbc::ResultsTeam> {
        self.tpv.results_team.stream.data().clone()
    }

    pub fn tpv_results_team_state(&self) -> tpvbc::BcastState {
        self.tpv.results_team.stream.state().clone()
    }
}
