use super::base;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
    pub url: String,
}

impl base::WidgetBase for Widget {   
    fn get_title(&self) -> &'static str {
        "Data Source Settings"
    }
}

impl Widget {
    pub fn new() -> Widget {
        Widget {
            visible: false,
            url: String::from("http://localhost:8080"),
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn show_window(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Set base URL of TPV data source.Supported format:\n"));
        ui.label(egui::RichText::new("\t- http://<name_or_ip>:<port>").italics());
        ui.label(egui::RichText::new("\t- file://<tpv_bcast_dir>\n").italics());
        ui.separator();

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("URL:");
            ui.text_edit_singleline(&mut self.url);    
        });

        ui.separator();
        ui.label(egui::RichText::new("To enable settings, you must Stop/Start receiving!").color(ui.visuals().warn_fg_color));
        ui.separator();

        if ui.button("Close").clicked() {
            self.visible = false;
        }
    }
}
