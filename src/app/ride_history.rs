use egui_plot::{Legend, Line, Plot, PlotPoints};

use crate::data::Facade;
use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn title(&self) -> &'static str {
        "History Graph"
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, df: &Facade) {
        let ride = df.ride();        

        let hr_points: PlotPoints = (1..ride.total.hr.history.len()).map(|i| {
            [
                i as f64,
                *ride.total.hr.history.get(i).unwrap() as f64,
            ]
        }).collect();

        let hr_history = Line::new(hr_points).name("bpm");

        let pwr_points: PlotPoints = (1..ride.total.power.history.len()).map(|i| {
            [
                i as f64,
                *ride.total.power.history.get(i).unwrap() as f64,
            ]
        }).collect();

        let pwr_history = Line::new(pwr_points).name("W");

        let cad_points: PlotPoints = (1..ride.total.cadence.history.len()).map(|i| {
            [
                i as f64,
                *ride.total.cadence.history.get(i).unwrap() as f64,
            ]
        }).collect();

        let cad_history = Line::new(cad_points).name("rpm");

        Plot::new("History Graph")
        .legend(Legend::default())
        .show(ui, |plot_ui| {
            plot_ui.line(hr_history);
            plot_ui.line(pwr_history);
            plot_ui.line(cad_history);
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
