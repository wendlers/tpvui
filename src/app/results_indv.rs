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
                        ui.end_row();
                        ui.label("");
                        ui.end_row();
                    }
                });
            });
        });
    }
}
