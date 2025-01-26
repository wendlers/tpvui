use crate::data::Facade;
use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn title(&self) -> &'static str {
        "Nearest"
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, df: &Facade) {
        let nearest = df.tpv_nearest_data();

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

                        self.key_value_simple(ui, "Speed", format!("{}", n.speed), "kph/275");
                        self.key_value_simple(ui, "Distance", format!("{}", n.distance), "m");
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

impl Widget {
    pub fn new() -> Widget {
        Widget {
            visible: false,
        }
    }

    pub fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    } 
}
