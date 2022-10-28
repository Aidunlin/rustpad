use std::{env::current_exe, process::Command};

use eframe::{
    egui::{
        Align, CentralPanel, Context, Layout, ScrollArea, TextEdit, TextStyle::*, TopBottomPanel,
    },
    epaint::{Color32, Vec2},
    App,
};

#[derive(Default)]
pub struct RustpadApp {
    text: String,
    font_size: f32,
    zoom: usize,
    cursor_line: usize,
    cursor_col: usize,
    show_status_bar: bool,
    show_settings: bool,
}

impl RustpadApp {
    pub fn new() -> Self {
        Self {
            font_size: 16.0,
            zoom: 100,
            show_status_bar: true,
            cursor_line: 1,
            cursor_col: 1,
            ..Default::default()
        }
    }
}

impl App for RustpadApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        for (text_style, font) in style.text_styles.iter_mut() {
            match text_style {
                Small => {}
                Body => font.size = 16.0,
                Monospace => font.size = 16.0,
                Button => font.size = 16.0,
                Heading => font.size = 32.0,
                Name(_) => {}
            }
        }
        style.spacing.button_padding = Vec2::new(8.0, 8.0);
        ctx.set_style(style);

        if !self.show_settings {
            TopBottomPanel::top("top").show(ctx, |ui| {
                ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                ui.horizontal(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {}
                        if ui.button("New window").clicked() {
                            Command::new(current_exe().expect("Failed to get current exe path"))
                                .spawn()
                                .expect("Failed to start new app");
                        }
                        if ui.button("Open").clicked() {}
                        if ui.button("Save").clicked() {}
                        if ui.button("Save as").clicked() {}
                        if ui.separator().clicked() {}
                        if ui.button("Page setup").clicked() {}
                        if ui.button("Print").clicked() {}
                        if ui.separator().clicked() {}
                        if ui.button("Exit").clicked() {
                            frame.close();
                        }
                    });
                    ui.menu_button("Edit", |ui| {
                        if ui.button("Undo").clicked() {}
                        ui.separator();
                        if ui.button("Cut").clicked() {}
                        if ui.button("Copy").clicked() {}
                        if ui.button("Paste").clicked() {}
                        if ui.button("Delete").clicked() {}
                        ui.separator();
                        if ui.button("Find").clicked() {}
                        if ui.button("Find next").clicked() {}
                        if ui.button("Find previous").clicked() {}
                        if ui.button("Replace").clicked() {}
                        if ui.button("Go to").clicked() {}
                        ui.separator();
                        if ui.button("Select all").clicked() {}
                        if ui.button("Time/Date").clicked() {}
                        ui.separator();
                        if ui.button("Font").clicked() {}
                    });
                    ui.menu_button("View", |ui| {
                        ui.menu_button("Zoom", |ui| {
                            if ui.button("Zoom in").clicked() {
                                if self.zoom < 500 {
                                    self.zoom += 10;
                                }
                                ui.close_menu();
                            }
                            if ui.button("Zoom out").clicked() {
                                if self.zoom > 10 {
                                    self.zoom -= 10;
                                }
                                ui.close_menu();
                            }
                            if ui.button("Restore default zoom").clicked() {
                                self.zoom = 100;
                                ui.close_menu();
                            }
                        });
                        ui.checkbox(&mut self.show_status_bar, "Status bar");
                        ui.checkbox(&mut false, "Word wrap");
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("⚙").clicked() {
                            self.show_settings = true;
                        }
                    });
                })
            });

            if self.show_status_bar {
                TopBottomPanel::bottom("bottom").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Ln {}, Col {}", self.cursor_line, self.cursor_col));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label("UTF-8");
                            ui.separator();
                            ui.label("Windows (CRLF)");
                            ui.separator();
                            ui.label(format!("{}%", self.zoom));
                            ui.separator();
                        });
                    });
                });
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                if self.show_settings {
                    if ui.button("Back").clicked() {
                        self.show_settings = false;
                    }
                    ui.heading("Settings");
                    ui.heading("About this app");
                    ui.label(format!("Rustpad {}", env!("CARGO_PKG_VERSION")));
                    ui.label("© 2022 Aidunlin");
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.style_mut()
                            .text_styles
                            .entry(Monospace)
                            .and_modify(|e| e.size = self.font_size * (self.zoom as f32 / 100.0));

                        let output = TextEdit::multiline(&mut self.text)
                            .frame(false)
                            .code_editor()
                            .show(ui);

                        output.response.context_menu(|ui| {
                            if ui.button("Undo").clicked() {}
                            ui.separator();
                            if ui.button("Cut").clicked() {}
                            if ui.button("Copy").clicked() {}
                            if ui.button("Paste").clicked() {}
                            if ui.button("Delete").clicked() {}
                            ui.separator();
                            if ui.button("Select all").clicked() {}
                        });

                        if let Some(cursor_range) = output.cursor_range {
                            let cursor = cursor_range.primary.pcursor;
                            self.cursor_line = cursor.paragraph + 1;
                            self.cursor_col = cursor.offset + 1;
                        }
                    });
                }
            });
        });
    }
}
