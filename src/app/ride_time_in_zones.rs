use std::ops::RangeInclusive;

use egui_plot::{AxisHints, Bar, BarChart, GridMark, Legend, Plot};

use crate::data::Facade;
use super::base::WidgetBase;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Widget {
    pub visible: bool,
}

impl WidgetBase for Widget {   
    fn title(&self) -> &'static str {
        "Time in Zones"
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn show_label(&mut self, ui: &mut egui::Ui) {
        self.visible = self.show_label_base(ui, self.visible);
    }

    fn show_window(&self, ui: &mut egui::Ui, df: &Facade) {
        let _ride = df.ride();        
        let w = 25.0;

        let zone_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| -> String {
            let v = mark.value as i32;

            if v == 12 {
                return String::from("HR");
            } else if v == 43 {
                return String::from("PWR");
            }

            String::new()
        };

        let x_axes = vec![
            AxisHints::new_x()
                .label("t% in Zones")
                .formatter(zone_formatter)
                .placement(egui_plot::VPlacement::Top)];
     
        let data_z1 = BarChart::new(vec![
            Bar::new(w * 0.5, 5.0).name("HR Z1"),
            Bar::new(w * 1.75, 10.0).name("PWR Z1"),
        ])
        .width(w)
        .name("Z1");

        let data_z2 = BarChart::new(vec![
            Bar::new(w * 0.5, 10.0).name("HR Z2"),
            Bar::new(w * 1.75, 5.0).name("PWR Z2"),
        ])
        .width(w)
        .name("Z2")
        .stack_on(&[&data_z1]);

        let data_z3 = BarChart::new(vec![
            Bar::new(w * 0.5, 45.0).name("HR Z3"),
            Bar::new(w * 1.75, 45.0).name("PWR Z3"),
        ])
        .width(w)
        .name("Z3")
        .stack_on(&[&data_z2]);

        let data_z4 = BarChart::new(vec![
            Bar::new(w * 0.5, 20.0).name("HR Z4"),
            Bar::new(w * 1.75, 20.0).name("PWR Z4"),
        ])
        .width(w)
        .name("Z4")
        .stack_on(&[&data_z3]);

        let data_z5 = BarChart::new(vec![
            Bar::new(w * 0.5, 10.0).name("HR Z5"),
            Bar::new(w * 1.75, 10.0).name("PWR Z5"),
        ])
        .width(w)
        .name("Z5")
        .stack_on(&[&data_z4]);

        let data_z6 = BarChart::new(vec![
            Bar::new(w * 0.5, 7.0).name("HR Z6"),
            Bar::new(w * 1.75, 7.0).name("PWR Z6"),
        ])
        .width(w)
        .name("Z6")
        .stack_on(&[&data_z5]);

        let data_z7 = BarChart::new(vec![
            Bar::new(w * 0.5, 3.0).name("HR Z7"),
            Bar::new(w * 1.75, 3.0).name("PWR Z7"),
        ])
        .width(w)
        .name("Z7")
        .stack_on(&[&data_z6]);
            
        Plot::new("Time in Zones")
        .legend(Legend::default())
        .x_grid_spacer(|_input | {
            vec![
                GridMark { value: w * 0.5, step_size: w}, 
                GridMark { value: w * 1.75, step_size: w}]})
        .custom_x_axes(x_axes)
        .data_aspect(1.0)
        .allow_drag(false)
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(data_z1);
            plot_ui.bar_chart(data_z2);
            plot_ui.bar_chart(data_z3);
            plot_ui.bar_chart(data_z4);
            plot_ui.bar_chart(data_z5);
            plot_ui.bar_chart(data_z6);
            plot_ui.bar_chart(data_z7);
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
