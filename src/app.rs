mod settings;
mod visualizer;

use eframe::egui::{Button, CentralPanel, PaintCallback, Sense, SidePanel, Ui, Widget, Window};
use eframe::{App, egui};
use eframe::wgpu::{Extent3d, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use crate::app::settings::Settings;
use crate::app::visualizer::Visualizer;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct EguiApp {
    settings: Settings,
    #[serde(skip)]
    settings_pinned: bool,
    #[serde(skip)]
    visualizer: Visualizer,
}

impl EguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.settings_pinned {
            Window::new("Visualizer")
                .vscroll(true)
                .open(&mut self.settings_pinned)
                .show(ctx, |ui| {
                    self.settings.ui(ui);
                });
        } else {
            SidePanel::right("settings_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Visualizer");
                    if Button::new("⏏").frame(false).ui(ui).clicked() {
                        self.settings_pinned = true;
                    }
                });
                ui.separator();
                self.settings.ui(ui);

            });
        }

        CentralPanel::default().show(ctx, |ui| {
            self.visualizer.ui(ui);
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}