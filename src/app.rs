use core::time;
use base::WidgetBase;
use egui::Color32;

use crate::data::{DataCollector, DataSource, DataSourceStatus};

mod base;
mod focus;
mod nearest;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TpvUiApp {
    widged_focus: focus::Widget,
    widget_nearest: nearest::Widget,

    #[serde(skip)]
    dc: DataCollector,
}

impl Default for TpvUiApp {
    fn default() -> Self {
        Self {
            widged_focus: focus::Widget::new(), 
            widget_nearest: nearest::Widget::new(),   
            dc: DataCollector::new(),
        }
    }
}

impl TpvUiApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }        
        Default::default()
    }
    
    fn data_source_status(&self, ui: &mut egui::Ui, ds: &DataSource, label: &str, ) {
        let mut status = String::from(label);
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
    
    fn menu_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Center), |ui| {

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
                });
                
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::widgets::global_theme_preference_buttons(ui);
                        ui.add(egui::Separator::default().vertical());
                });
            });

        });
    }

    fn widget_panel(&mut self, ctx: &egui::Context) {
        // egui::SidePanel::left("side_panel").frame(egui::Frame::none()).show(ctx, |ui| {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    self.widged_focus.show_label(ui);
                    self.widget_nearest.show_label(ui);
                });
            });
         });
    }

    fn status_panel(&mut self, ctx: &egui::Context) {
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
                    self.data_source_status(ui, &&self.dc.get_source_focus(), "focus");
                    // nearest status
                    self.data_source_status(ui, &self.dc.get_source_nearest(), "nearest");
                }
            );
        });
    }
    
    fn widget_windows(&mut self, ctx: &egui::Context) {
        if self.widged_focus.is_visible() {            
            egui::Window::new(self.widged_focus.get_title()).show(ctx, |ui| {
                self.widged_focus.show_window(ui, self.dc.get_focus());
            });
        }
        
        if self.widget_nearest.is_visible() {            
            egui::Window::new("Nearest").show(ctx, |ui| {
                self.widget_nearest.show_window(ui, self.dc.get_nearest());
            });
        }
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

        self.menu_panel(ctx);
        self.widget_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Image::new(egui::include_image!("../assets/ventoux.jpg"))
                .paint_at(ui, ctx.used_rect())
        });

        self.widget_windows(ctx);
        self.status_panel(ctx);
    }
}
