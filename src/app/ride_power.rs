use crate::data::Facade;
use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn title(&self) -> &'static str {
        "Power"
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, df: &Facade) {
        let ride = df.ride();
        
        // 2x2 grid
        egui::Grid::new("power_grid_1x1_a")
        .min_col_width(200.0)
        .max_col_width(200.0)
        .min_row_height(100.0)
        .num_columns(1)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            self.field_2x2(ui, String::from("★ W"), format!("{:4.0}", ride.total.power.cur));
            ui.end_row();
        });

        // 1x2 grid
        egui::Grid::new("power_grid_1x2")
        .min_col_width(200.0)
        .max_col_width(200.0)
        .min_row_height(25.0)
        .num_columns(1)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(format!("Z{:1.0} {}", 
                    ride.athlete.pwr_zones.zone(ride.total.power.cur),
                    ride.athlete.pwr_zones.name(ride.total.power.cur)))
                    .size(21.0).color(egui::Color32::LIGHT_GREEN));
            });
            ui.end_row();
        });
        
        // 2x2 grid
        egui::Grid::new("power_grid_1x1_b")
        .min_col_width(200.0)
        .max_col_width(200.0)
        .min_row_height(100.0)
        .num_columns(1)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            self.field_1x1(ui, String::from("nrm W"), format!("{:4.0}", ride.total.power.nrm));
            ui.end_row();
        });

        // 1x1 grid
        egui::Grid::new("power_data_grid_2x2")
        .min_col_width(100.0)
        .max_col_width(100.0)
        .min_row_height(50.0)
        .num_columns(2)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            self.field_1x1(ui, String::from("★ W/kg"), format!("{:2.1}", ride.total.power.wpk));
            self.field_1x1(ui, String::from("max W"), format!("{:4.0}", ride.total.power.max));
            ui.end_row();
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
