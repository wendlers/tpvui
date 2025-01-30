use crate::data::Facade;
use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn title(&self) -> &'static str {
        "Wind"
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
        egui::Grid::new("wind_grid_1x1")
        .min_col_width(210.0)
        .max_col_width(210.0)
        .min_row_height(100.0)
        .num_columns(1)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            self.field_2x2(ui, String::from("★ kmh"), format!("{:3.1}", ride.total.wind.speed));
            ui.end_row();
            self.field_2x2(ui, String::from("★ deg"), format!("{:3.0}", ride.total.wind.angle));
            ui.end_row();
        });

        // 1x1 grid
        egui::Grid::new("wind_data_grid_2x2")
        .min_col_width(210.0)
        .max_col_width(210.0)
        .min_row_height(50.0)
        .num_columns(1)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            self.field_1x1(ui, String::from("★ Draft %"), format!("{:3.0}", ride.total.wind.draft));
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
