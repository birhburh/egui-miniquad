use {egui_miniquad as egui_mq, miniquad as mq};

struct Stage {
    egui_mq: egui_mq::EguiMq,
    prev_egui_zoom_factor: f32,
    zoom_factor: f32,
    mq_ctx: Box<dyn mq::RenderingBackend>,
}

impl Stage {
    fn new() -> Self {
        let mut mq_ctx = mq::window::new_rendering_backend();

        let egui_mq = egui_mq::EguiMq::new(&mut *mq_ctx);

        #[cfg(target_os = "macos")]
        load_system_font(egui_mq.egui_ctx());

        Self {
            egui_mq,
            prev_egui_zoom_factor: 1.0,
            zoom_factor: 1.0,
            mq_ctx,
        }
    }

    fn content(ui: &mut egui::Ui, egui_ctx: &egui::Context, dpi_scale: f32, zoom_factor: &mut f32) {
        egui::widgets::global_dark_light_mode_buttons(ui);

        ui.group(|ui| {
            ui.label("Physical pixels per each logical 'point':");
            ui.label(format!("native: {:.2}", dpi_scale));
            ui.label(format!("egui:   {:.2}", ui.ctx().pixels_per_point()));
            ui.label("Current zoom factor:");
            ui.add(egui::Slider::new(zoom_factor, 0.75..=3.0).logarithmic(true))
                .on_hover_text(
                    "Override egui zoom factor manually (changes effective pixels per point)",
                );
            if ui.button("Reset").clicked() {
                *zoom_factor = 1.0;
            }

            ui.label("By default, egui allows zooming with\nCtrl/Cmd and +/-/0");
            // Creating a checkbox that directly mutates the egui context's options causes a
            // freeze so we copy the state out, possibly mutate it with the checkbox, and
            // then copy it back in.
            let mut zoom_with_keyboard = egui_ctx.options(|o| o.zoom_with_keyboard);
            ui.checkbox(&mut zoom_with_keyboard, "Allow egui zoom with keyboard");
            egui_ctx.options_mut(|o| o.zoom_with_keyboard = zoom_with_keyboard);
        });

        #[cfg(not(target_arch = "wasm32"))]
        {
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
        }
        if ui.button("bottom button").clicked() {
            println!("BOTTOM BUTTON!");
        }
    }
}

#[cfg(target_os = "macos")]
use std::fs::read;

#[cfg(target_os = "macos")]
use egui::epaint::FontFamily;
#[cfg(target_os = "macos")]
use egui::{FontData, FontDefinitions};
#[cfg(target_os = "macos")]
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};

#[cfg(target_os = "macos")]
const FONT_SYSTEM_SANS_SERIF: &'static str = "System Sans Serif";

#[cfg(target_os = "macos")]
fn load_system_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    let handle = SystemSource::new()
        .select_best_match(
            &[FamilyName::Title("Helvetica".into()), FamilyName::SansSerif],
            &Properties::new(),
        )
        .unwrap();

    let buf: Vec<u8> = match handle {
        Handle::Memory { bytes, .. } => bytes.to_vec(),
        Handle::Path { path, .. } => read(path).unwrap(),
    };

    fonts
        .font_data
        .insert(FONT_SYSTEM_SANS_SERIF.to_owned(), FontData::from_owned(buf));

    fonts.families.insert(
        FontFamily::Proportional,
        vec![FONT_SYSTEM_SANS_SERIF.to_owned()],
    );

    ctx.set_fonts(fonts);
}

impl mq::EventHandler for Stage {
    fn update(&mut self) -> bool {
        println!("UPDATE");
        let dpi_scale = mq::window::dpi_scale();

        let zoom_factor = &mut self.zoom_factor;
        // Run the UI code:
        self.egui_mq.run(&mut *self.mq_ctx, |_mq_ctx, egui_ctx| {
            // zoom factor could have been changed by the user in egui using Ctrl/Cmd and -/+/0,
            // but it could also be in the middle of being changed by us using the slider. So we
            // only allow egui's zoom to override our zoom if the egui zoom is different from what
            // we saw last time (meaning the user has changed it).
            let curr_egui_zoom = egui_ctx.zoom_factor();
            if self.prev_egui_zoom_factor != curr_egui_zoom {
                *zoom_factor = curr_egui_zoom;
            }
            self.prev_egui_zoom_factor = curr_egui_zoom;

            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.style_mut().spacing.button_padding = egui::vec2(16.0, 16.0);
                // ui.style_mut().spacing.slider_width = 200.0;
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(24.0, egui::epaint::FontFamily::Proportional),
                );
                ui.vertical_centered_justified(|ui| {
                    Stage::content(ui, egui_ctx, dpi_scale, zoom_factor);
                });
            });

            // Don't change zoom while dragging the slider
            if !egui_ctx.is_using_pointer() {
                egui_ctx.set_zoom_factor(*zoom_factor);
            }
        });
        self.egui_mq.egui_ctx().has_requested_repaint()
    }

    fn draw(&mut self) {
        println!("DRAW");
        // Draw things behind egui here
        self.mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        self.mq_ctx
            .begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.mq_ctx.end_render_pass();

        self.egui_mq.draw(&mut *self.mq_ctx);

        // Draw things in front of egui here

        self.mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

fn main() {
    let conf = mq::conf::Conf {
        window_title: "egui ❤ miniquad".to_string(),
        high_dpi: true,
        window_width: 500,
        window_height: 300,
        ..Default::default()
    };
    mq::start(conf, || Box::new(Stage::new()));
}
