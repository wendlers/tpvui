use crate::data::{DataCollector, Facade};

use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn get_title(&self) -> &'static str {
        "Results Team"
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, dc: &DataCollector) {
        let results = dc.get_results_team();
        
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("results_team_data_grid").show(ui, |ui| {
                    for r in results.iter() {
                        self.key_value_simple(ui, "Team", format!("{}", r.team), "");
                        self.key_value_simple(ui, "Team Code", format!("{}", r.teamCode), "");
                        ui.end_row();

                        self.key_value_simple(ui, "Location", format!("{}", r.location), "");
                        self.key_value_simple(ui, "Position", format!("{}", r.position), "");
                        self.key_value_simple(ui, "Points Total", format!("{}", r.pointsTotal), "");
                        ui.end_row();

                        self.key_value_simple(ui, "Time", format!("{}", r.time), "s");
                        self.key_value_simple(ui, "Delta Time", format!("{}", r.deltaTime), "s");
                        ui.end_row();

                        ui.label("");
                        ui.end_row();
                    }
                });
            });
        });
    }

    fn show_window_v2(&self, ui: &mut egui::Ui, df: &Facade) {
        let results = df.tpv_results_team_data();
        
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("results_team_data_grid").show(ui, |ui| {
                    for r in results.iter() {
                        self.key_value_simple(ui, "Team", format!("{}", r.team), "");
                        self.key_value_simple(ui, "Team Code", format!("{}", r.teamCode), "");
                        ui.end_row();

                        self.key_value_simple(ui, "Location", format!("{}", r.location), "");
                        self.key_value_simple(ui, "Position", format!("{}", r.position), "");
                        self.key_value_simple(ui, "Points Total", format!("{}", r.pointsTotal), "");
                        ui.end_row();

                        self.key_value_simple(ui, "Time", format!("{}", r.time), "s");
                        self.key_value_simple(ui, "Delta Time", format!("{}", r.deltaTime), "s");
                        ui.end_row();

                        ui.label("");
                        ui.end_row();
                    }
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
