use std::{collections::HashMap, io::Read, sync::Arc};

use crate::ui::MyApp;
use anyhow::{Context, Result};
use clap::Parser;
use eframe::{
    CreationContext,
    egui::{FontData, FontDefinitions, FontFamily, Pos2, ViewportBuilder},
};
use log::warn;

mod ui;

#[derive(Parser)]
pub struct Arg {
    /// Input file, '-' means stdin.
    #[arg(short, long, default_value = "-")]
    pub input: String,

    /// Output file, '-' means stdout.
    #[arg(short, long, default_value = "-")]
    pub output: String,

    /// Exit after saving or copying.
    #[arg(short, long, default_value_t = false)]
    pub exit: bool,

    /// The fonts list which uesed to render text.
    #[arg(short, long)]
    pub fonts: Vec<String>,
}

fn main() -> Result<()> {
    env_logger::init();
    let arg = Arg::parse();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_always_on_top()
            .with_position(Pos2::ZERO)
            .with_fullscreen(true),
        ..Default::default()
    };

    let image_data = if arg.input == "-" {
        let mut image_data = Vec::new();
        std::io::stdin()
            .read_to_end(&mut image_data)
            .with_context(|| "Failed to read image data from stdin")?;
        image_data
    } else {
        std::fs::read(&arg.input).with_context(|| "Failed to read image data from stdin")?
    };
    let fonts_data = load_font(&arg.fonts)?;
    let create_with_context = |ctx: &CreationContext| -> Result<Box<dyn eframe::App>, _> {
        let mut fonts = FontDefinitions::default();
        let font_family = if fonts_data.is_empty() {
            FontFamily::Proportional
        } else {
            fonts
                .families
                .entry(FontFamily::Name("User".into()))
                .or_default()
                .extend(fonts_data.keys().cloned());

            fonts.font_data.extend(fonts_data);
            FontFamily::Name("User".into())
        };

        ctx.egui_ctx.set_fonts(fonts);
        egui_extras::install_image_loaders(&ctx.egui_ctx);
        Ok(Box::new(MyApp::new(image_data, font_family, arg)))
    };
    eframe::run_native("Slappy", options, Box::new(create_with_context))
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    Ok(())
}

fn load_font(fonts: &Vec<String>) -> Result<HashMap<String, Arc<FontData>>> {
    let mut res: HashMap<String, Arc<FontData>> = HashMap::new();
    if fonts.is_empty() {
        return Ok(res);
    }

    let Some(fc) = fontconfig::Fontconfig::new() else {
        anyhow::bail!("Load fontconfig failed.");
    };
    for font in fonts {
        let Some(f) = fc.find(font, None) else {
            warn!("Can not find font {:?}", font);
            continue;
        };
        match std::fs::read(&f.path) {
            Ok(font_data) => {
                res.insert(f.name, Arc::new(FontData::from_owned(font_data)));
            }
            Err(e) => {
                warn!(
                    "Read file {:?} failed when load font {:?}. Reason: {}",
                    f.path, font, e
                );
            }
        }
    }
    Ok(res)
}
