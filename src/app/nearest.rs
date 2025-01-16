use crate::data::TpvNearest;

use super::base::{self, WidgetBase};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl base::WidgetBase for Widget {   
    fn get_title(&self) -> &'static str {
        "Nearest"
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

    pub fn show_window(&self, ui: &mut egui::Ui, nearest: Vec<TpvNearest>) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("nearest_data_grid").show(ui, |ui| {
                    for n in nearest.iter() {
                        self.key_value_simple(ui, "Name", format!("{}", n.name), "");
                        self.key_value_simple(ui, "Country", format!("{}", n.country), "");
                        ui.end_row();
                        
                        self.key_value_simple(ui, "Team", format!("{}", n.team), "");
                        self.key_value_simple(ui, "Team Code", format!("{}", n.teamCode), "");
                        ui.end_row();

                        self.key_value_simple(ui, "Speed", format!("{}", n.speed / 275), "kph");
                        self.key_value_simple(ui, "Distance", format!("{}", n.distance / 1000), "km");
                        ui.end_row();

                        self.key_value_simple(ui, "Position", format!("{}", n.position), "");
                        self.key_value_simple(ui, "Eliminated", format!("{}", n.isEliminated), "");
                        ui.end_row();
                        
                        ui.label("");
                        ui.end_row();
                    }
                
                });
            });
        });
    }
}
