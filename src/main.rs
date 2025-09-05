use std::{io::Read, sync::Arc};

use anyhow::{Context, Result};
use eframe::{
    CreationContext,
    egui::{self, Pos2},
};

use crate::ui::MyApp;

mod ui;

fn main() -> Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_always_on_top()
            .with_position(Pos2::ZERO)
            .with_fullscreen(true),
        ..Default::default()
    };

    let mut image_data = Vec::new();
    let read_size = std::io::stdin()
        .read_to_end(&mut image_data)
        .with_context(|| "Failed to read image data from stdin")?;
    if read_size == 0 {
        return Err(anyhow::anyhow!("No image data provided on stdin"));
    }
    let create_with_context = |ctx: &CreationContext| -> Result<Box<dyn eframe::App>, _> {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "SourceHanSansSC-Regular".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(
                "../font/SourceHanSansSC-Regular.otf"
            ))),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("SourceHanSansSC-Regular".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("SourceHanSansSC-Regular".to_owned());

        ctx.egui_ctx.set_fonts(fonts);
        egui_extras::install_image_loaders(&ctx.egui_ctx);
        Ok(Box::new(MyApp::new(image_data)))
    };
    eframe::run_native("Slappy", options, Box::new(create_with_context))
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    Ok(())
}
