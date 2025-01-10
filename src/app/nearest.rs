use crate::data::TpvNearest;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
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
        if ui.selectable_label(self.visible, "Nearest").clicked() {
            self.visible = !self.visible;
        }
    } 

    pub fn show_window(&self, ui: &mut egui::Ui, nearest: TpvNearest) {
        egui::Grid::new("nearest_data_grid").show(ui, |ui| {
            ui.label("TbD").highlight();
        });
    }
}
