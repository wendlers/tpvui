use crate::data::ride::Ride;

use super::{
    BcastState,
    Focus,
    Nearest,
    Event,
    Entries,
    Groups,
    ResultsIndv,
    ResultsTeam,
};

pub trait BcastStreamIf {
    fn start(&mut self, url: String);

    fn stop(&self);

    fn running(&self) -> bool;

    fn focus_data(&self) -> Focus;

    fn focus_state(&self) -> BcastState;

    fn nearest_data(&self) -> Vec<Nearest>;

    fn nearest_state(&self) -> BcastState;

    fn event_data(&self) -> Event;

    fn event_state(&self) -> BcastState;

    fn entries_data(&self) -> Vec<Entries>;

    fn entries_state(&self) -> BcastState;

    fn groups_data(&self) -> Vec<Groups>;

    fn groups_state(&self) -> BcastState;

    fn results_indv_data(&self) -> Vec<ResultsIndv>;

    fn results_indv_state(&self) -> BcastState;

    fn results_team_data(&self) -> Vec<ResultsTeam>;

    fn results_team_state(&self) -> BcastState;

    fn ride(&self) -> Ride;
}
