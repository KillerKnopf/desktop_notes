use eframe::{
    egui::{self, style, Button, Frame, Key, ScrollArea, SidePanel, TopBottomPanel},
    epaint::{Color32, FontFamily, FontId, Rounding, Stroke, Vec2},
};

#[cfg(debug_assertions)]
use eframe::epaint::Pos2;
#[cfg(debug_assertions)]
use tracing::trace;

#[derive(Default)]
pub struct NotesApp {
    note: String,
    is_on_top: bool,
    #[cfg(debug_assertions)]
    tracked_position: Pos2,
}

impl NotesApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut notes_app = Self::default();
        notes_app.is_on_top = true;
        notes_app
    }

    #[cfg(debug_assertions)]
    fn custom_update_logs(&mut self, frame: &mut eframe::Frame) {
        let pos = frame.info().window_info.position.unwrap_or(Pos2::ZERO);
        if pos != self.tracked_position {
            trace!("Position -> x: {}, y: {}", pos.x, pos.y);
            self.tracked_position = pos;
        }
    }

    fn change_always_on_top(&mut self, frame: &mut eframe::Frame) {
        self.is_on_top = !self.is_on_top;
        frame.set_always_on_top(self.is_on_top);
    }
}

impl eframe::App for NotesApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // This window won't have decoration but it should still be dragable
        frame.drag_window();

        let btn_frame = Frame {
            inner_margin: style::Margin {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            },
            outer_margin: style::Margin {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            },
            rounding: Rounding::ZERO,
            shadow: eframe::epaint::Shadow::NONE,
            fill: Color32::GRAY,
            stroke: Stroke::NONE,
        };

        let txt_frame = Frame {
            inner_margin: style::Margin {
                left: 4.0,
                right: 0.0,
                top: 4.0,
                bottom: 4.0,
            },
            outer_margin: style::Margin::ZERO,
            rounding: Rounding::ZERO,
            shadow: eframe::epaint::Shadow::NONE,
            fill: Color32::GOLD,
            stroke: Stroke::NONE,
        };

        TopBottomPanel::bottom("note area")
            .exact_height(ctx.available_rect().height() * 0.9)
            .frame(txt_frame)
            .show(ctx, |ui| {
                ScrollArea::new([false, true])
                    .max_width(f32::INFINITY)
                    .show(ui, |ui| {
                        ui.add_sized(
                            Vec2 { x: 320.0, y: 200.0 },
                            egui::TextEdit::multiline(&mut self.note)
                                .text_color(Color32::BLACK)
                                .font(FontId::new(20.0, FontFamily::Proportional))
                                .margin(Vec2::new(3.0, 0.0))
                                .frame(false),
                        );
                    });
            });

        SidePanel::left("settings button")
            .exact_width(ctx.available_rect().width() / 2.0)
            .frame(btn_frame)
            .show(ctx, |ui| {
                let btn_settings = Button::new("Settings").rounding(Rounding::ZERO);

                ui.add_sized(ui.available_size(), btn_settings);
            });

        SidePanel::left("hide button")
            .exact_width(ctx.available_rect().width())
            .frame(btn_frame)
            .show(ctx, |ui| {
                let btn_hide = Button::new("Hide").rounding(Rounding::ZERO);

                // Putting an if before a ui.add function creates a Response that can handle events
                if ui.add_sized(ui.available_size(), btn_hide).clicked() {
                    self.change_always_on_top(frame);
                }
            });

        if ctx.input(|input| input.key_pressed(Key::Escape) && !input.modifiers.shift) {
            frame.close();
        }

        if ctx.input(|input| input.key_pressed(Key::Escape) && input.modifiers.shift) {
            self.change_always_on_top(frame);
        }

        #[cfg(debug_assertions)]
        self.custom_update_logs(frame);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(5)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Color32::to_normalized_gamma_f32(Color32::TRANSPARENT)
    }
}
