use eframe::egui::{self, Id, Image, Key, LayerId, TextureHandle, Ui, UiBuilder, Vec2, Widget};

pub struct PinWindow {
    texture_handle: TextureHandle,
}

impl eframe::App for PinWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut ui = Ui::new(
            ctx.clone(),
            Id::new("pin"),
            UiBuilder::new()
                .layer_id(LayerId::background())
                .max_rect(ctx.available_rect()),
        );

        let screenshot_image = Image::from_texture(&self.texture_handle)
            .fit_to_fraction(Vec2::splat(1f32))
            .show_loading_spinner(false);
        ui.centered_and_justified(|ui| screenshot_image.ui(ui));
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}

impl PinWindow {
    pub fn new(texture_handle: TextureHandle) -> Self {
        Self { texture_handle }
    }
}
