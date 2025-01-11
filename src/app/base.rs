#[allow(dead_code)]
pub trait WidgetBase {
    fn get_default_text_size(&self) -> f32 {
        25.0
    }

    fn show_label_base(&mut self, ui: &mut egui::Ui, visible: bool) -> bool {
        if ui.selectable_label(visible, egui::RichText::new(self.get_title()).size(20.0)).clicked() {
            return !visible;
        }
        visible
    } 

    fn key_value_simple(&self, ui: &mut egui::Ui, k: &str, v: String, u: &str) {
        let fsize = self.get_default_text_size();

        ui.label(egui::RichText::new(k).size(fsize));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
            ui.label(egui::RichText::new(v).size(fsize).color(egui::Color32::LIGHT_BLUE));
        });
        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label(egui::RichText::new(u).size(fsize * 0.5).color(egui::Color32::DARK_GRAY));
            ui.add_space(fsize);
        });
    }  

    fn value_simple(&self, ui: &mut egui::Ui, v: String) {
        let fsize = self.get_default_text_size();
        ui.label(egui::RichText::new(v).size(fsize).color(egui::Color32::LIGHT_BLUE));
    }  

    fn get_title(&self) -> &'static str;
}