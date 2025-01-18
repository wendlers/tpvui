use crate::data::DataCollector;

use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

#[allow(dead_code)]
impl WidgetBase for Widget {   
    fn get_title(&self) -> &'static str {
        "Groups"
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, dc: &DataCollector) {
        let groups = dc.get_groups();

        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("groups_data_grid").show(ui, |ui| {
                    for g in groups.iter() {
                        self.key_value_simple(ui, "Group #1", format!("{}", g.groupNum1), "");
                        self.key_value_simple(ui, "Group #2", format!("{}", g.groupNum2), "");
                        ui.end_row();

                        self.key_value_simple(ui, "Time gap #1", format!("{}", g.timeGap1), "s");
                        self.key_value_simple(ui, "Time gap #2", format!("{}", g.timeGap2), "s");
                        ui.end_row();

                        self.key_value_simple(ui, "Size", format!("{}", g.size), "");
                        self.key_value_simple(ui, "Peleton", format!("{}", g.isPeloton), "");
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
