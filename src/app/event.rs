use crate::data::{DataCollector, Facade};

use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn get_title(&self) -> &'static str {
        "Event"
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, dc: &DataCollector) {
        let event = dc.get_event();
        
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("event_data_grid").show(ui, |ui| {
                    self.key_value_simple(ui, "Name", format!("{}", event.name), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Route", format!("{}", event.route), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Type", format!("{}", event.type_), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Laps", format!("{}", event.laps), "");
                    ui.end_row();
                    
                    self.key_value_simple(ui, "Distance", format!("{}", event.distance / 1000), "km");
                    ui.end_row();

                    self.key_value_simple(ui, "Height", format!("{}", event.height), "m");
                    ui.end_row();

                    self.key_value_simple(ui, "Locations", format!("{}", event.locations), "");
                    ui.end_row();
                });
            });
        });
    }

    fn show_window_v2(&self, ui: &mut egui::Ui, df: &Facade) {
        let event = df.tpv_event_data();
        
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("event_data_grid").show(ui, |ui| {
                    self.key_value_simple(ui, "Name", format!("{}", event.name), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Route", format!("{}", event.route), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Type", format!("{}", event.type_), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Laps", format!("{}", event.laps), "");
                    ui.end_row();
                    
                    self.key_value_simple(ui, "Distance", format!("{}", event.distance / 1000), "km");
                    ui.end_row();

                    self.key_value_simple(ui, "Height", format!("{}", event.height), "m");
                    ui.end_row();

                    self.key_value_simple(ui, "Locations", format!("{}", event.locations), "");
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
