use core::time;
use egui::Color32;

use crate::data::{DataCollector, DataSourceStatus};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TpvUiApp {
    // Example stuff:
    // label: String,

    #[serde(skip)]
    dc: DataCollector,
}

impl Default for TpvUiApp {
    fn default() -> Self {
        Self {
            dc: DataCollector::new(),
        }
    }
}

impl TpvUiApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }        
        Default::default()
    }
}

impl eframe::App for TpvUiApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.dc.start();

        ctx.request_repaint_after(time::Duration::from_millis(250));
        ctx.set_pixels_per_point(1.0);

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            let focus = self.dc.get_focus();

            ui.heading(focus.name);
            ui.separator();

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.label("Speed").highlight();
                ui.label(format!("{} kph", focus.speed / 275));
                ui.label("Distance").highlight();
                ui.label(format!("{} km", focus.distance / 1000));
                ui.label("Time").highlight();
                ui.label(format!("{} s", focus.time));
                ui.end_row();
                ui.end_row();

                ui.label("Power").highlight();
                ui.label(format!("{} W", focus.power));
                ui.label("Avg. Power").highlight();
                ui.label(format!("{} W", focus.avgPower));
                ui.label("Max. Power").highlight();
                ui.label(format!("{} W", focus.maxPower));
                ui.label("Nrm. Power").highlight();
                ui.label(format!("{} W", focus.nrmPower));
                ui.end_row();
                ui.end_row();

                ui.label("HR").highlight();
                ui.label(format!("{} bpm", focus.heartrate));
                ui.label("Avg. HR").highlight();
                ui.label(format!("{} bpm", focus.avgHeartrate));
                ui.label("Max. HR").highlight();
                ui.label(format!("{} bpm", focus.maxHeartrate));
                ui.end_row();
                ui.end_row();

                ui.label("Cadence").highlight();
                ui.label(format!("{} rpm", focus.cadence));
                ui.label("Avg. Cadence").highlight();
                ui.label(format!("{} rpm", focus.avgCadence));
                ui.label("Max. Cadence").highlight();
                ui.label(format!("{} rpm", focus.maxCadence));
                ui.end_row();
                ui.end_row();

                ui.label("Windspeed").highlight();
                ui.label(format!("{} kph", focus.windSpeed / 100));
                ui.label("Wind angle").highlight();
                ui.label(format!("{} deg", focus.windAngle));
                ui.label("Slope").highlight();
                ui.label(format!("{} %", focus.slope));
                ui.end_row();
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {    
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if cfg!(debug_assertions) {
                        ui.label(
                            egui::RichText::new("☢ Debug build ☢")
                                    .color(ui.visuals().warn_fg_color),
                        )
                        .on_hover_text("tpvui was compiled with debug enabled");
                        ui.add(egui::Separator::default().vertical());
                    }
                    // focus status
                    {
                        let ds = self.dc.get_source_focus();
                        let mut status = String::from("focus");
                        let mut color = Color32::GREEN;
                
                        if ds.status == DataSourceStatus::NotOk {
                            status.push_str(" ⚠ ");
                            color = Color32::RED;
                        } else {
                            status.push_str(" ☭ ");
                        }

                        status.push_str(&format!("{:08}", ds.frame));
                        ui.label(
                            egui::RichText::new(status)
                                .color(color)
                        );
                        ui.add(egui::Separator::default().vertical());
                    }
                    // nearest status
                    {
                        let ds = self.dc.get_source_nearest();
                        let mut status = String::from("nearest");
                        let mut color = Color32::GREEN;
                
                        if ds.status == DataSourceStatus::NotOk {
                            status.push_str(" ⚠ ");
                            color = Color32::RED;
                        } else {
                            status.push_str(" ☭ ");
                        }

                        status.push_str(&format!("{:08}", ds.frame));
                        ui.label(
                            egui::RichText::new(status)
                                .color(color)
                        );
                        ui.add(egui::Separator::default().vertical());
                    }

                }
            );
        });
    }
}