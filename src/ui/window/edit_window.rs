use std::io::{Cursor, Write};

use eframe::App;
use eframe::egui::{self, Button, Image, Rect, Ui, Vec2, Widget, ahash::HashMap};
use eframe::egui::{
    Align2, Color32, ColorImage, Context, FontFamily, Modifiers, Pos2, Response, RichText, Spinner,
};
use egui::Key;
use strum::IntoEnumIterator;
use strum::{EnumIter, IntoStaticStr};

use crate::Arg;
use crate::ui::crop::CropTool;
use crate::ui::shape::circle::{Circle, CircleAttribute};
use crate::ui::shape::line::{Line, LineAttribute};
use crate::ui::shape::number::{Number, NumberAttribute};
use crate::ui::shape::pen::{Pen, PenAttribute};
use crate::ui::shape::rectangle::RectangleAttribute;
use crate::ui::shape::text::{Text, TextAttribute};
use crate::ui::shape::{CreateAt, Shape};
use crate::ui::shape::{ShapeId, rectangle::Rectangle};
use crate::ui::utils::from_ratio_rect;
use crate::ui::window::RenderInfo;

#[derive(PartialEq, Eq, Clone, Copy, Default, EnumIter, IntoStaticStr)]
enum Tool {
    None,
    #[default]
    Crop,
    Rect,
    Circle,
    Line,
    Text,
    Number,
    Pen,
}

pub struct EditWindow<'a> {
    raw_screenshot_texture: egui::load::Bytes,

    /// The currently selected tool in the toolbar
    selected_tool: Tool,

    /// Cropped range
    crop_tool: CropTool,

    /// Shapes
    shapes: HashMap<ShapeId, Box<dyn Shape>>,
    active_shape_id: Option<ShapeId>,

    rect_attributes: RectangleAttribute,
    circle_attributes: CircleAttribute,
    line_attributes: LineAttribute,
    text_attributes: TextAttribute,
    number_attributes: NumberAttribute,
    pen_attributes: PenAttribute,

    error_message: Option<String>,

    want_screenshot: bool,
    want_screenshot_first_signal: bool,
    screenshot_copy: bool,
    screenshot_save: bool,
    screenshot_pin: bool,
    user_font: FontFamily,

    arg: Arg,
    pinned_image: &'a mut Option<ColorImage>,
}

impl<'a> EditWindow<'a> {
    pub fn new(
        raw_screenshot: Vec<u8>,
        user_font: FontFamily,
        arg: Arg,
        pinned_image: &'a mut Option<ColorImage>,
    ) -> Self {
        Self {
            raw_screenshot_texture: egui::load::Bytes::from(raw_screenshot),
            selected_tool: Default::default(),
            crop_tool: Default::default(),
            shapes: Default::default(),
            active_shape_id: None,
            rect_attributes: Default::default(),
            circle_attributes: Default::default(),
            line_attributes: Default::default(),
            text_attributes: Default::default(),
            number_attributes: Default::default(),
            pen_attributes: Default::default(),
            error_message: None,
            user_font,
            want_screenshot: false,
            want_screenshot_first_signal: false,
            screenshot_copy: false,
            screenshot_save: false,
            screenshot_pin: false,
            arg,
            pinned_image,
        }
    }

    pub fn active_shape(&mut self) -> Option<&mut Box<dyn Shape>> {
        self.active_shape_id.and_then(|id| self.shapes.get_mut(&id))
    }
}

impl<'a> App for EditWindow<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut render_info = RenderInfo {
            screenshot_rect: Rect::ZERO,
            pixel_ratio: 1f32,
            user_font: self.user_font.clone(),
            shot_mode: self.want_screenshot,
        };
        egui::CentralPanel::default()
            .frame(egui::containers::Frame::NONE)
            .show(ctx, |ui| {
                let resp = self.ui_background(ui, &mut render_info);

                // render all shapes
                self.ui_shape(ui, &render_info);

                // handle global mouse event
                self.handle_global_response(ui, &resp, &render_info);

                // render crop range
                if !self.want_screenshot {
                    self.crop_tool
                        .ui(ui, &render_info, self.selected_tool == Tool::Crop);
                }

                if self.want_screenshot_first_signal {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    self.want_screenshot_first_signal = false;
                }
                // Check for returned screenshot
                self.handle_screenshot_event(ctx, &render_info);
            });
        if !self.want_screenshot {
            self.ui_toolbar(ctx, &render_info);
        }

        self.ui_error_message(ctx);
    }
}

impl<'a> EditWindow<'a> {
    fn ui_background(&mut self, ui: &mut Ui, render_info: &mut RenderInfo) -> Response {
        let screenshot_image =
            Image::from_bytes("bytes://picture", self.raw_screenshot_texture.clone())
                .fit_to_original_size(1f32 / ui.ctx().pixels_per_point())
                .show_loading_spinner(false)
                .sense(egui::Sense::click_and_drag());
        let image_size = screenshot_image
            .load_for_size(ui.ctx(), Vec2::INFINITY)
            .ok()
            .and_then(|t| t.size());
        let resp = screenshot_image.ui(ui);

        // render spinner if image is not load
        if image_size.is_none() {
            Spinner::new().paint_at(
                ui,
                Rect::from_center_size(ui.max_rect().center(), Vec2::splat(70f32)),
            );
        }

        let pixel_ratio = image_size.map_or(1f32, |v| resp.rect.width() / v.x);
        let screenshot_rect = resp.rect;
        render_info.screenshot_rect = screenshot_rect;
        render_info.pixel_ratio = pixel_ratio;
        resp
    }

    fn ui_toolbar(&mut self, ctx: &Context, render_info: &RenderInfo) {
        egui::Window::new("Tools")
            .collapsible(false)
            .resizable(true)
            .show(ctx, |ui| {
                // tools
                ui.horizontal(|ui| {
                    for tool in Tool::iter() {
                        if tool == Tool::None {
                            continue;
                        }
                        let btn = Button::new(<Tool as Into<&'static str>>::into(tool))
                            .selected(self.selected_tool == tool);
                        if btn.ui(ui).clicked() {
                            if self.selected_tool == tool {
                                self.selected_tool = Tool::None;
                            } else {
                                self.selected_tool = tool;
                            }
                            self.active_shape_id = None;
                        }
                    }
                });

                ui.separator();

                // save, copy and pin
                let cropped_range =
                    from_ratio_rect(&self.crop_tool.cropped_range, &render_info.screenshot_rect);
                let saveable = ctx.available_rect().contains_rect(cropped_range);
                ui.horizontal(|ui| {
                    let save_shotcut = ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::S));
                    let save_btn_clicked = ui.add_enabled(saveable, Button::new("Save")).clicked();
                    let copy_shotcut = ui
                        .input(|inp| inp.events.iter().any(|ev| matches!(ev, egui::Event::Copy)))
                        && !ctx.wants_keyboard_input();
                    let copy_btn_clicked = ui.add_enabled(saveable, Button::new("Copy")).clicked();
                    let copy_and_save_btn_clicked = ui.add_enabled(saveable, Button::new("Copy and Save")).clicked();
                    let pin_btn_clicked = ui.button("Pin").clicked();

                    if save_shotcut || save_btn_clicked || copy_shotcut || copy_btn_clicked || copy_and_save_btn_clicked || pin_btn_clicked{
                        if !saveable {
                            self.error_message = Some("Save or copy failed. Crop area exceeds screen. Please maximize the window and try again.".to_string());
                        } else {
                            self.want_screenshot = true;
                            self.want_screenshot_first_signal = true;
                            self.screenshot_copy = copy_shotcut || copy_btn_clicked || copy_and_save_btn_clicked;
                            self.screenshot_save = save_shotcut || save_btn_clicked || copy_and_save_btn_clicked;
                            self.screenshot_pin = pin_btn_clicked;
                        }
                    }
                });
                ui.separator();
                egui::Grid::new("attributes").show(ui, |ui| {
                    if let Some(active_shape) = self.active_shape() {
                        active_shape.toolbar_ui(ui, render_info);
                    } else {
                        match self.selected_tool {
                            Tool::None | Tool::Crop => {}
                            Tool::Circle => self.circle_attributes.ui(ui),
                            Tool::Rect => self.rect_attributes.ui(ui),
                            Tool::Line => self.line_attributes.ui(ui),
                            Tool::Text => {
                                self.text_attributes.ui(ui, render_info.user_font.clone())
                            }
                            Tool::Number => self.number_attributes.ui(ui),
                            Tool::Pen => self.pen_attributes.ui(ui),
                        }
                    }
                });
            });
    }

    fn ui_shape(&mut self, ui: &mut Ui, render_info: &RenderInfo) {
        let mut has_active = false;
        for (shape_id, shape) in self.shapes.iter_mut() {
            if shape.ui(
                ui,
                !self.want_screenshot && self.active_shape_id.is_some_and(|x| &x == shape_id),
                render_info,
            ) {
                has_active = true;
                self.active_shape_id = Some(*shape_id);
            }
        }
        if !has_active {
            self.active_shape_id = None;
        }

        if ui.ctx().input(|i| i.key_pressed(Key::Delete))
            && let Some(shape_id) = self.active_shape_id
        {
            self.shapes.remove(&shape_id);
            self.active_shape_id = None;
        }

        if ui.ctx().input(|i| i.key_pressed(Key::Escape)) {
            if self.active_shape_id.is_some() {
                self.active_shape_id = None;
            } else {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }

    fn handle_global_response(
        &mut self,
        ui: &mut Ui,
        resp: &eframe::egui::Response,
        render_info: &RenderInfo,
    ) {
        match self.selected_tool {
            Tool::Crop => {
                self.crop_tool.on_global_response(ui, resp, render_info);
            }
            Tool::Rect => Rectangle::handle_create_response(
                ui,
                resp,
                render_info,
                &self.rect_attributes,
                &mut self.active_shape_id,
                &mut self.shapes,
            ),
            Tool::Circle => Circle::handle_create_response(
                ui,
                resp,
                render_info,
                &self.circle_attributes,
                &mut self.active_shape_id,
                &mut self.shapes,
            ),
            Tool::Line => Line::handle_create_response(
                ui,
                resp,
                render_info,
                &self.line_attributes,
                &mut self.active_shape_id,
                &mut self.shapes,
            ),
            Tool::Text => Text::handle_create_response(
                ui,
                resp,
                render_info,
                &self.text_attributes,
                &mut self.active_shape_id,
                &mut self.shapes,
            ),
            Tool::Number => Number::handle_create_response(
                ui,
                resp,
                render_info,
                &self.number_attributes,
                &mut self.active_shape_id,
                &mut self.shapes,
            ),
            Tool::Pen => Pen::handle_create_response(
                ui,
                resp,
                render_info,
                &self.pen_attributes,
                &mut self.active_shape_id,
                &mut self.shapes,
            ),
            Tool::None => {}
        }
    }

    fn handle_screenshot_event(&mut self, ctx: &Context, render_info: &RenderInfo) {
        let screenshot_image = ctx.input(|i| {
            for event in &i.raw.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    return Some(image.clone());
                }
            }
            None
        });
        let Some(image) = screenshot_image else {
            return;
        };
        let pixels_per_point = ctx.pixels_per_point();
        let cropped_range =
            from_ratio_rect(&self.crop_tool.cropped_range, &render_info.screenshot_rect);
        let image_rect = Rect::from_min_max(
            Pos2::ZERO,
            Pos2 {
                x: image.width() as f32 / pixels_per_point,
                y: image.height() as f32 / pixels_per_point,
            },
        );
        if !image_rect.contains_rect(cropped_range) {
            self.error_message = Some("Save or copy failed. Crop area exceeds screen. Please maximize the window and try again.".to_string());
            return;
        }
        let cropped = image.region(&cropped_range, Some(pixels_per_point));

        if let Err(e) = save_image_as_file(
            cropped.clone(),
            self.screenshot_copy,
            self.screenshot_save.then_some(&self.arg.output),
            ctx,
        ) {
            self.error_message = Some(e.to_string());
            return;
        }
        self.want_screenshot = false;
        if self.arg.exit && !self.screenshot_pin {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if self.screenshot_pin {
            *self.pinned_image = Some(cropped);
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn ui_error_message(&mut self, ctx: &Context) {
        if let Some(error_message) = self.error_message.clone() {
            let mut open = true;
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .anchor(
                    Align2::RIGHT_TOP,
                    Vec2 {
                        x: -30f32,
                        y: 30f32,
                    },
                )
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.label(RichText::new(error_message).color(Color32::RED))
                });
            if !open {
                self.error_message = None;
            }
        }
    }
}

fn save_image_as_file(
    image: ColorImage,
    copy: bool,
    output_path: Option<&str>,
    ctx: &Context,
) -> Result<(), Box<dyn std::error::Error>> {
    if copy {
        ctx.copy_image(image.clone());
    }
    if let Some(output_path) = output_path {
        if output_path == "-" {
            let mut cursor = Cursor::new(Vec::new());
            image::write_buffer_with_format(
                &mut cursor,
                image.as_raw(),
                image.width() as u32,
                image.height() as u32,
                image::ColorType::Rgba8,
                image::ImageFormat::Png,
            )?;
            cursor.flush()?;
            cursor.set_position(0);
            std::io::copy(&mut cursor, &mut std::io::stdout())?;
        } else {
            image::save_buffer(
                output_path,
                image.as_raw(),
                image.width() as u32,
                image.height() as u32,
                image::ColorType::Rgba8,
            )?;
        }
    }
    Ok(())
}
