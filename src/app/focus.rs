use crate::data::DataCollector;

use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

#[allow(dead_code)]
impl WidgetBase for Widget {   
    fn get_title(&self) -> &'static str {
        "Focus"
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, dc: &DataCollector) {
        let focus = dc.get_focus();

        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("focus_ride_data_grid").show(ui, |ui| {
                    self.key_value_simple(ui, "Name", format!("{}", focus.name), "");
                    self.key_value_simple(ui, "Country", format!("{}", focus.country), "");
                    self.key_value_simple(ui, "Team", format!("{}", focus.team), "");
                    self.key_value_simple(ui, "Team Code", format!("{}", focus.teamCode), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Speed", format!("{}", focus.speed / 275), "kph");
                    self.key_value_simple(ui, "Distance", format!("{}", focus.distance / 1000), "km");
                    self.key_value_simple(ui, "Time", format!("{}", focus.time), "s");
                    ui.end_row();

                    self.key_value_simple(ui, "HR", format!("{}", focus.heartrate), "bpm");
                    self.key_value_simple(ui, "Avg. HR", format!("{}", focus.avgHeartrate), "bpm");
                    self.key_value_simple(ui, "Max. HR", format!("{}", focus.maxHeartrate), "bpm");
                    ui.end_row();

                    self.key_value_simple(ui, "Cadence", format!("{}", focus.cadence), "rpm");
                    self.key_value_simple(ui, "Avg. Cadence", format!("{}", focus.avgCadence), "rpm");
                    self.key_value_simple(ui, "Max. Cadence", format!("{}", focus.maxCadence), "rpm");
                    ui.end_row();

                    self.key_value_simple(ui, "Power", format!("{}", focus.power), "W");
                    self.key_value_simple(ui, "Avg. Power", format!("{}", focus.avgPower), "W");
                    self.key_value_simple(ui, "Max. Power", format!("{}", focus.maxPower), "W");
                    self.key_value_simple(ui, "Nrm. Power", format!("{}", focus.nrmPower), "W");
                    ui.end_row();

                    self.key_value_simple(ui, "Windspeed", format!("{}", focus.windSpeed / 275), "kph");
                    self.key_value_simple(ui, "Wind angle", format!("{}", focus.windAngle), "deg");
                    self.key_value_simple(ui, "Draft", format!("{}", focus.draft), "%");
                    ui.end_row();

                    self.key_value_simple(ui, "Height", format!("{}", focus.height), "m");
                    self.key_value_simple(ui, "Slope", format!("{}", focus.slope), "%");
                    ui.end_row();

                    self.key_value_simple(ui, "Calories", format!("{}", focus.calories), "kcal");
                    self.key_value_simple(ui, "TSS", format!("{}", focus.tss), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Laps Total", format!("{}", focus.eventLapsTotal), "");
                    self.key_value_simple(ui, "Laps Done", format!("{}", focus.eventLapsDone), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Distance Total", format!("{}", focus.eventDistanceTotal / 1000), "km");
                    self.key_value_simple(ui, "Distance Done", format!("{}", focus.eventDistanceDone / 1000), "km");
                    ui.end_row();

                    self.key_value_simple(ui, "Next Location", format!("{}", focus.eventNextLocation), "");
                    self.key_value_simple(ui, "Position", format!("{}", focus.eventPosition), "");
                    ui.end_row();
                });
            });
        });
    }
}

impl Widget {
    pub fn new() -> Widget {
        Widget {
            visible: false,
        }
    }
}
