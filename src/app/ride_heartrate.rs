use crate::data::Facade;
use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn title(&self) -> &'static str {
        "Heartrate"
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
        egui::Grid::new("hr_grid_1x1_a")
        .min_col_width(210.0)
        .max_col_width(210.0)
        .min_row_height(100.0)
        .num_columns(1)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            self.field_2x2(ui, String::from("★ bpm"), format!("{:4.1}", ride.total.hr.cur));
            ui.end_row();
        });

        // 1x2 grid
        egui::Grid::new("hr_grid_1x2")
        .min_col_width(210.0)
        .max_col_width(210.0)
        .min_row_height(25.0)
        .num_columns(1)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(format!("Z{:1.0} {}", 
                    ride.athlete.hr_zones.zone(ride.total.hr.cur),
                    ride.athlete.hr_zones.name(ride.total.hr.cur)))
                    .size(21.0).color(egui::Color32::LIGHT_GREEN));
            });
            ui.end_row();
        });

        // 2x2 grid
        egui::Grid::new("hr_grid_1x1_b")
        .min_col_width(210.0)
        .max_col_width(210.0)
        .min_row_height(100.0)
        .num_columns(1)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            self.field_1x1(ui, String::from("ø bpm"), format!("{:4.1}", ride.total.hr.avg));
            ui.end_row();
        });

        // 1x1 grid
        egui::Grid::new("hr_data_grid_2x2")
        .min_col_width(105.0)
        .max_col_width(105.0)
        .min_row_height(50.0)
        .num_columns(2)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            // self.field_1x1(ui, String::from("ø bpm"), format!("{:4.1}", ride.total.hr.avg));
            // self.field_1x1(ui, String::from("★ Zone"), format!("Z{:1.0}", ride.athlete.hr_zones.zone(ride.total.hr.cur)));
            // ui.end_row();
            self.field_1x1(ui, String::from("min bpm"), format!("{:4.1}", ride.total.hr.min));
            self.field_1x1(ui, String::from("max bpm"), format!("{:4.1}", ride.total.hr.max));
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
