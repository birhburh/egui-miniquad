use {egui_miniquad as egui_mq, miniquad as mq};

struct Content {
    scroll_to_row_slider: usize,
    scroll_to_row: Option<usize>,
    selection: std::collections::HashSet<usize>,
}

struct Stage {
    egui_mq: egui_mq::EguiMq,
    prev_egui_zoom_factor: f32,
    zoom_factor: f32,
    mq_ctx: Box<dyn mq::RenderingBackend>,
    content: Content,
}

fn expanding_content(ui: &mut egui::Ui) {
    let width = ui.available_width().clamp(20.0, 200.0);
    let height = ui.available_height();
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.center().y,
        (1.0, ui.visuals().text_color()),
    );
}

fn long_text(row_index: usize) -> String {
    format!("Row {row_index} has some long text that you may want to clip, or it will take up too much horizontal space!")
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
            content: Content {
                scroll_to_row_slider: 0,
                scroll_to_row: None,
                selection: Default::default(),
            },
        }
    }
}

impl Content {
    fn table_ui(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let text_height = (egui::TextStyle::Button.resolve(ui.style()).size + 32.0)
            .max(ui.spacing().interact_size.y);

        let mut table = TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .auto_shrink(false);

        // table = table.sense(egui::Sense::click());

        // if let Some(row_index) = self.scroll_to_row.take() {
        //     table = table.scroll_to_row(row_index, None);
        // }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Content");
                });
            })
            .body(|body| {
                body.rows(text_height, 200, |mut row| {
                    let row_index = row.index();
                    // row.set_selected(self.selection.contains(&row_index));

                    row.col(|ui| {
                        ui.horizontal_centered(|ui| {
                            ui.add(
                                egui::Label::new("Thousands of rows of even height").wrap(false),
                            );
                            if ui.button("Quit").clicked() {}
                        });
                    });

                    // self.toggle_row_selection(row_index, &row.response());
                });
            });
    }

    fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selection.contains(&row_index) {
                self.selection.remove(&row_index);
            } else {
                self.selection.insert(row_index);
            }
        }
    }

    fn content(&mut self, ui: &mut egui::Ui, dpi_scale: f32, zoom_factor: &mut f32) {
        // egui::widgets::global_dark_light_mode_buttons(ui);

        // ui.group(|ui| {
        //     ui.label("Physical pixels per each logical 'point':");
        //     ui.label(format!("native: {:.2}", dpi_scale));
        //     ui.label(format!("egui:   {:.2}", ui.ctx().pixels_per_point()));
        //     ui.label("Current zoom factor:");
        //     ui.add(egui::Slider::new(zoom_factor, 0.75..=3.0).logarithmic(true))
        //         .on_hover_text(
        //             "Override egui zoom factor manually (changes effective pixels per point)",
        //         );
        //     if ui.button("Reset").clicked() {
        //         *zoom_factor = 1.0;
        //     }
        // });

        // #[cfg(not(target_arch = "wasm32"))]
        // {
        //     if ui.button("Quit").clicked() {
        //         std::process::exit(0);
        //     }
        // }

        self.table_ui(ui);
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
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(24.0, egui::epaint::FontFamily::Proportional),
                );
                ui.vertical_centered_justified(|ui| {
                    self.content.content(ui, dpi_scale, zoom_factor);
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
        ..Default::default()
    };
    mq::start(conf, || Box::new(Stage::new()));
}
