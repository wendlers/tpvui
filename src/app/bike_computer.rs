use crate::data::Facade;
use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn title(&self) -> &'static str {
        "Bike Computer"
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, df: &Facade) {
        let ride = df.ride();
        
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("event_data_grid").show(ui, |ui| {
                    self.key_value_simple(ui, "Time", format!("{}", ride.total.time_hms()), "hms");
                    self.key_value_simple(ui, "Dist.", format!("{:4.1}", ride.total.distance), "km");
                    ui.end_row();

                    self.key_value_simple(ui, "TSS", format!("{}", ride.total.tss), "");
                    self.key_value_simple(ui, "Cal.", format!("{}", ride.total.calories), "kcal");
                    self.key_value_simple(ui, "Lap", format!("{}", ride.total.lap), "");
                    ui.end_row();

                    self.key_value_simple(ui, "Speed", format!("{:2.1}", ride.total.speed.cur), "kph");
                    self.key_value_simple(ui, "Avg.", format!("{:2.1}", ride.total.speed.avg), "kph");
                    self.key_value_simple(ui, "Max.", format!("{:2.1}", ride.total.speed.max), "kph");
                    ui.end_row();

                    self.key_value_simple(ui, "HR", format!("{:3.0}", ride.total.hr.cur), "bpm");
                    self.key_value_simple(ui, "Avg.", format!("{:3.0}", ride.total.hr.avg), "bpm");
                    self.key_value_simple(ui, "Min.", format!("{:3.0}", ride.total.hr.min), "bpm");
                    self.key_value_simple(ui, "Max.", format!("{:3.0}", ride.total.hr.max), "bpm");
                    ui.end_row();

                    self.key_value_simple(ui, "PWR", format!("{}", ride.total.power.cur), "W");
                    self.key_value_simple(ui, "Max.", format!("{}", ride.total.power.max), "W");
                    self.key_value_simple(ui, "Nrm.", format!("{}", ride.total.power.nrm), "W");
                    self.key_value_simple(ui, "W/kg.", format!("{}", ride.total.power.wpk), "W/kg");
                    ui.end_row();

                    self.key_value_simple(ui, "Alt.", format!("{}", ride.total.height.cur), "m");
                    self.key_value_simple(ui, "Min.", format!("{}", ride.total.height.min), "m");
                    self.key_value_simple(ui, "Max.", format!("{}", ride.total.height.max), "m");
                    self.key_value_simple(ui, "Asc.", format!("{}", ride.total.height.ascend), "m");
                    self.key_value_simple(ui, "Desc.", format!("{}", ride.total.height.descend), "m");
                    self.key_value_simple(ui, "Slope", format!("{}", ride.total.height.slope), "%");
                    ui.end_row();

                    self.key_value_simple(ui, "Wind", format!("{:3.1}", ride.total.wind.speed), "kph");
                    self.key_value_simple(ui, "Angle", format!("{}", ride.total.wind.angle), "deg");
                    self.key_value_simple(ui, "Draft", format!("{}", ride.total.wind.draft), "%");
                    ui.end_row();

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
