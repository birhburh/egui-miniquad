use {egui_miniquad as egui_mq, miniquad as mq};

struct Content {}

struct Stage {
    egui_mq: egui_mq::EguiMq,
    mq_ctx: Box<dyn mq::RenderingBackend>,
    content: Content,
    first: bool,
}

impl Stage {
    fn new() -> Self {
        let mut mq_ctx = mq::window::new_rendering_backend();

        let egui_mq = egui_mq::EguiMq::new(&mut *mq_ctx);

        // #[cfg(target_os = "macos")]
        // load_system_font(egui_mq.egui_ctx());

        Self {
            egui_mq,
            mq_ctx,
            content: Content {},
            first: true,
        }
    }
}

impl Content {
    fn content(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let desired_width = ui.available_width();
            let desired_height = ui.available_height();
            let desired_size = egui::vec2(desired_width, desired_height);
            let (_id, rect) = ui.allocate_space(desired_size);

            let painter = ui.painter();
            painter.rect(
                rect.shrink(1.0),
                10.0,
                ui.ctx().style().visuals.window_fill(),
                egui::Stroke::new(0.5, egui::Color32::DARK_GRAY),
            );
            painter.line_segment(
                [
                    rect.left_top() + egui::vec2(2.0, rect.height() * 0.5),
                    rect.right_top() + egui::vec2(-2.0, rect.height() * 0.5),
                ],
                egui::Stroke::new(0.5, egui::Color32::DARK_GREEN),
            );
            let pos_top_left =
                egui::emath::pos2(rect.left() + (rect.width() / 2.0) - 30.0, rect.top() + 20.0);
            let pos_bottom_right = egui::emath::pos2(
                rect.left() + (rect.width() / 2.0) + 30.0,
                rect.top() + rect.height() - 20.0,
            );
            let r = egui::emath::Rect::from_two_pos(pos_top_left, pos_bottom_right);

            painter.rect_filled(r, 10.0, egui::ecolor::Rgba::from_luminance_alpha(0.2, 0.2));
            painter.text(
                egui::pos2(
                    rect.left() + rect.width() * 0.2,
                    rect.top() + rect.height() * 0.7,
                ),
                egui::Align2::LEFT_CENTER,
                "This is some text",
                egui::FontId::new(30.0, egui::FontFamily::Proportional),
                egui::Color32::RED,
            );
        });
    }

    fn expanding_content(ui: &mut egui::Ui) {
        let width = ui.available_width().clamp(20.0, 200.0);
        let height = ui.available_height();
        let (rect, _response) =
            ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
        ui.painter().hline(
            rect.x_range(),
            rect.center().y,
            (1.0, ui.visuals().text_color()),
        );
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
    fn update(&mut self, skiped: bool) -> bool {
        // Run the UI code:
        self.egui_mq
            .run(&mut *self.mq_ctx, skiped, |_mq_ctx, egui_ctx| {
                #[cfg(target_os = "macos")]
                if self.first {
                    self.first = false;
                    egui_ctx.request_repaint();
                }

                egui::TopBottomPanel::bottom("tools").show(egui_ctx, |ui| {
                    ui.style_mut().spacing.button_padding = egui::vec2(16.0, 16.0);
                    // ui.style_mut().spacing.slider_width = 200.0;
                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Button,
                        egui::FontId::new(24.0, egui::epaint::FontFamily::Proportional),
                    );
                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Body,
                        egui::FontId::new(24.0, egui::epaint::FontFamily::Proportional),
                    );

                    let scroll_bar_visibility = if ! cfg!(target_os = "android") {
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
                    } else {
                        egui::scroll_area::ScrollBarVisibility::AlwaysHidden
                    };
                    egui::ScrollArea::horizontal()
                        .drag_to_scroll(true)
                        .scroll_bar_visibility(scroll_bar_visibility)
                        .show(ui, |ui| {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Max), |ui| {
                                if ui.button("🔍").clicked() {
                                    mq::window::set_window_size(100, 100);
                                }
                                if ui.button("🛃").clicked() {
                                    mq::window::set_window_size(300, 300);
                                }
                                if ui.button("💛").clicked() {
                                    mq::window::set_window_size(500, 500);
                                }
                                if ui.button("💚").clicked() {
                                    mq::window::set_window_size(700, 700);
                                }
                            });
                        });
                });

                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    ui.style_mut().spacing.button_padding = egui::vec2(16.0, 16.0);
                    // ui.style_mut().spacing.slider_width = 200.0;
                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Button,
                        egui::FontId::new(24.0, egui::epaint::FontFamily::Proportional),
                    );
                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Body,
                        egui::FontId::new(24.0, egui::epaint::FontFamily::Proportional),
                    );
                    self.content.content(ui);
                });
            });
        self.egui_mq.egui_ctx().has_requested_repaint()
    }

    fn draw(&mut self) {
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

    fn resize_event(&mut self, _width: f32, _height: f32) {
        self.egui_mq.egui_ctx().request_repaint();
    }
}

fn main() {
    let conf = mq::conf::Conf {
        window_title: "egui ❤ miniquad".to_string(),
        high_dpi: true,
        window_width: 500,
        window_height: 300,
        min_width: Some(300),
        ..Default::default()
    };
    mq::start(conf, || Box::new(Stage::new()));
}
