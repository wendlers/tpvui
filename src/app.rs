use core::time;
use base::WidgetBase;
use egui::Color32;

use crate::data::{Facade, tpvbc::BcastState, tpvbc::BcastStatus};

mod base;
mod focus;
mod nearest;
mod event;
mod entries;
mod groups;
mod results_indv;
mod results_team;
mod setings_source;

const APP_KEY: &str = "tpvui";

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TpvUiApp {
    widged_focus: focus::Widget,
    widget_nearest: nearest::Widget,
    widget_event: event::Widget,
    widget_entries: entries::Widget,
    widget_groups: groups::Widget,
    widget_results_indv: results_indv::Widget,
    widget_results_team: results_team::Widget,
    widget_settings_source: setings_source::Widget,

    #[serde(skip)]
    df: Facade,
}

impl Default for TpvUiApp {
    fn default() -> Self {
        Self {
            widged_focus: focus::Widget::new(), 
            widget_nearest: nearest::Widget::new(),   
            widget_event: event::Widget::new(),
            widget_entries: entries::Widget::new(),
            widget_groups: groups::Widget::new(),
            widget_results_indv: results_indv::Widget::new(),
            widget_results_team: results_team::Widget::new(),
            widget_settings_source: setings_source::Widget::new(),
            df: Facade::new(),
        }
    }
}

impl TpvUiApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
    
    fn data_source_status(&self, ui: &mut egui::Ui, ds: &BcastState, label: &str, ) {
        let mut status = String::from(label);
        let mut color = Color32::GREEN;

        if ds.status == BcastStatus::Unknown {
            status.push_str(" X ");
            color = Color32::GRAY;
        } else if ds.status == BcastStatus::NotOk {
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
                    }
                    ui.add_space(16.0);
                    ui.menu_button("Data Source", |ui| {
                        if ui.button("Settings").clicked() {
                            self.widget_settings_source.visible = !self.widget_settings_source.visible; 
                        }
                        ui.separator();
                        if self.df.running() {
                            if ui.button("Stop receiving").clicked() {
                                self.df.stop();
                            }                            
                        } else {
                            if ui.button("Start receiving").clicked() {
                                self.df.start(self.widget_settings_source.url.clone());
                            }    
                        }
                    });
                    ui.add_space(16.0);                        
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
        egui::SidePanel::left("widget_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    self.widged_focus.show_label(ui);
                    self.widget_nearest.show_label(ui);
                    self.widget_event.show_label(ui);
                    self.widget_entries.show_label(ui);
                    self.widget_groups.show_label(ui);
                    self.widget_results_indv.show_label(ui);
                    self.widget_results_team.show_label(ui);
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
                                    .color(ui.visuals().warn_fg_color)
                        )
                        .on_hover_text("tpvui was compiled with debug enabled");
                        ui.add(egui::Separator::default().vertical());
                    }

                    if self.df.running() {
                        ui.label(
                            egui::RichText::new("★ Started")
                                    .color(Color32::GREEN)
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("★ Started")
                                    .color(Color32::GRAY)
                        );
                    }
                    ui.add(egui::Separator::default().vertical());

                    self.data_source_status(ui, &self.df.tpv_focus_state(), "focus");
                    self.data_source_status(ui, &self.df.tpv_nearest_state(), "nearest");
                    self.data_source_status(ui, &self.df.tpv_event_state(), "event");
                    self.data_source_status(ui, &self.df.tpv_entries_state(), "entries");
                    self.data_source_status(ui, &self.df.tpv_groups_state(), "groups");
                    self.data_source_status(ui, &self.df.tpv_results_indv_state(), "resultsIndv");
                    self.data_source_status(ui, &&self.df.tpv_results_team_state(), "resultsTeam");
                }
            );
        });
    }

    fn window_show_hide(ctx: &egui::Context, wdg: &impl WidgetBase, df: &Facade) {
        if wdg.visible() {            
            egui::Window::new(wdg.title()).show(ctx, |ui| {
                wdg.show_window(ui, df);
            });
        }          
    }

    fn widget_windows(&mut self, ctx: &egui::Context) {
        TpvUiApp::window_show_hide(ctx, &self.widged_focus, &self.df); 
        TpvUiApp::window_show_hide(ctx, &self.widget_nearest, &self.df);
        TpvUiApp::window_show_hide(ctx, &self.widget_event, &self.df); 
        TpvUiApp::window_show_hide(ctx, &self.widget_entries, &self.df);
        TpvUiApp::window_show_hide(ctx, &self.widget_groups, &self.df);
        TpvUiApp::window_show_hide(ctx, &self.widget_results_indv, &self.df);
        TpvUiApp::window_show_hide(ctx, &self.widget_results_team, &self.df);

        if self.widget_settings_source.is_visible() {            
            egui::Window::new(self.widget_settings_source.get_title()).show(ctx, |ui| {
                self.widget_settings_source.show_window(ui);
            });
        }
    }
}

impl eframe::App for TpvUiApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.0);
        ctx.request_repaint_after(time::Duration::from_millis(500));

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
