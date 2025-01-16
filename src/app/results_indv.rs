use crate::data::TpvResultsIndv;

use super::base::{self, WidgetBase};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl base::WidgetBase for Widget {   
    fn get_title(&self) -> &'static str {
        "Results Indv."
    }
}

impl Widget {
    pub fn new() -> Widget {
        Widget {
            visible: false,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    } 

    pub fn show_window(&self, ui: &mut egui::Ui, results: Vec<TpvResultsIndv>) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("results_indv_data_grid").show(ui, |ui| {
                    for r in results.iter() {
                        self.key_value_simple(ui, "Name", format!("{}", r.name), "");
                        self.key_value_simple(ui, "Country", format!("{}", r.country), "");
                        self.key_value_simple(ui, "Eliminated", format!("{}", r.isEliminated), "");
                        ui.end_row();

                        self.key_value_simple(ui, "Team", format!("{}", r.team), "");
                        self.key_value_simple(ui, "Team Code", format!("{}", r.teamCode), "");
                        ui.end_row();

                        self.key_value_simple(ui, "Location", format!("{}", r.location), "");
                        self.key_value_simple(ui, "Position", format!("{}", r.position), "");
                        ui.end_row();

                        self.key_value_simple(ui, "Points", format!("{}", r.points), "");
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