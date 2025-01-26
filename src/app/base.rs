#[allow(dead_code)]
pub trait WidgetBase {
    fn default_text_size(&self) -> f32 {
        25.0
    }

    fn show_label_base(&mut self, ui: &mut egui::Ui, visible: bool) -> bool {
        if ui.selectable_label(visible, egui::RichText::new(self.title()).size(16.0)).clicked() {
            return !visible;
        }
        visible
    } 
    
    fn show_label(&mut self, ui: &mut egui::Ui);

    fn key_value_simple(&self, ui: &mut egui::Ui, k: &str, v: String, u: &str) {
        let fsize = self.default_text_size();

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
        let fsize = self.default_text_size();
        ui.label(egui::RichText::new(v).size(fsize).color(egui::Color32::LIGHT_BLUE));
    }  

    fn field_1x1(&self, ui: &mut egui::Ui, t: String, v: String) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.separator();
            ui.label(egui::RichText::new(t).size(14.0).color(egui::Color32::DARK_GRAY));
            ui.separator();
            ui.label(egui::RichText::new(v).size(32.0).color(egui::Color32::LIGHT_BLUE));
        });
    }

    fn field_2x2(&self, ui: &mut egui::Ui, t: String, v: String) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.separator();
            ui.label(egui::RichText::new(t).size(14.0).color(egui::Color32::DARK_GRAY));
            ui.separator();
            ui.label(egui::RichText::new(v).size(48.0).color(egui::Color32::LIGHT_BLUE));
        });
    }

    fn title(&self) -> &'static str;

    fn visible(&self) -> bool;

    fn show_window(&self, _ui: &mut egui::Ui, _df: &crate::data::Facade);
}