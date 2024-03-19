/// egui example to compare with miniquad-egui behaivour
pub struct WrapApp {
    show_settings: bool,
}

impl WrapApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let slf = Self {
            show_settings: false,
        };
        slf
    }
}

impl eframe::App for WrapApp {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        visuals.panel_fill.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("tools").show(ctx, |ui| {
            let scroll_bar_visibility = if !cfg!(target_os = "android") {
                egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
            } else {
                egui::scroll_area::ScrollBarVisibility::AlwaysHidden
            };
            if self.show_settings {
                // ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let mut value = 0;
                ui.add(egui::Slider::new(&mut value, 1..=10));
                // });
            }
            egui::ScrollArea::horizontal()
                .drag_to_scroll(true)
                .scroll_bar_visibility(scroll_bar_visibility)
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        if ui
                            .add_enabled(false, egui::Button::new("🖼"))
                            .on_hover_text("Background")
                            .clicked()
                        {
                            self.show_settings = !self.show_settings;
                        }
                        if ui.button("⏰").on_hover_text("Time").clicked() {
                            self.show_settings = !self.show_settings;
                        }
                        if ui.button("🏃").on_hover_text("Activity").clicked() {
                            self.show_settings = !self.show_settings;
                        }
                        if ui.button("📆").on_hover_text("Date").clicked() {
                            self.show_settings = !self.show_settings;
                        }
                        if ui.button("📵").on_hover_text("Status").clicked() {
                            self.show_settings = !self.show_settings;
                        }
                        if ui.button("🔋").on_hover_text("Battery").clicked() {
                            self.show_settings = !self.show_settings;
                        }
                        if ui.button("🎬").on_hover_text("Other").clicked() {
                            self.show_settings = !self.show_settings;
                        }
                    });
                });
        });
    }
}

impl WrapApp {
}

fn clock_button(ui: &mut egui::Ui, seconds_since_midnight: f64) -> egui::Response {
    let time = seconds_since_midnight;
    let time = format!(
        "{:02}:{:02}:{:02}.{:02}",
        (time % (24.0 * 60.0 * 60.0) / 3600.0).floor(),
        (time % (60.0 * 60.0) / 60.0).floor(),
        (time % 60.0).floor(),
        (time % 1.0 * 100.0).floor()
    );

    ui.button(egui::RichText::new(time).monospace())
}

// When compiling natively:
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 300.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "egui demo app",
        options,
        Box::new(|cc| Box::new(WrapApp::new(cc))),
    )
}
