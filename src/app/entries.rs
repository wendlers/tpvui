use crate::data::Facade;
use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn title(&self) -> &'static str {
        "Entries"
    }
    
    fn visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, df: &Facade) {
        let entries = df.tpv_entries_data();
        
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("entries_data_grid").show(ui, |ui| {
                    for e in entries.iter() {
                        self.key_value_simple(ui, "Name", format!("{} (# {})", e.name, e.bibNum), "");
                        self.key_value_simple(ui, "Country", format!("{}", e.country), "");
                        ui.end_row();
                        
                        self.key_value_simple(ui, "Team", format!("{}", e.team), "");
                        self.key_value_simple(ui, "Team Code", format!("{}", e.teamCode), "");
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
