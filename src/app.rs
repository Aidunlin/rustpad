use std::{env::current_exe, fs, process::Command};

use eframe::{
    egui::{
        widgets, Align, CentralPanel, Context, Key, Layout, Modifiers, ScrollArea, TextEdit,
        TextStyle::*, TopBottomPanel, Ui, Window,
    },
    epaint::{Color32, Vec2},
    App,
};

#[derive(Default)]
pub struct DialogState {
    text_edit: String,
    show: bool,
}

#[derive(Default)]
pub struct RustpadApp {
    text: String,
    font_size: f32,
    zoom: usize,
    cursor_line: usize,
    cursor_col: usize,
    show_status_bar: bool,
    show_settings: bool,
    open_dialog_state: DialogState,
    save_dialog_state: DialogState,
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

    fn central_scroll(ctx: &Context, contents: impl FnOnce(&mut Ui)) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, contents);
        });
    }

    fn any_dialog_shown(&mut self) -> bool {
        self.open_dialog_state.show || self.save_dialog_state.show
    }

    fn zoom_in(&mut self) {
        if self.zoom < 500 {
            self.zoom += 10;
        }
    }

    fn zoom_out(&mut self) {
        if self.zoom > 10 {
            self.zoom -= 10;
        }
    }

    fn zoom_reset(&mut self) {
        self.zoom = 100;
    }

    fn handle_zoom_inputs(&mut self, ctx: &Context) {
        ctx.input_mut(|input| {
            match input.zoom_delta() {
                delta if delta > 1.0 => self.zoom_in(),
                delta if delta < 1.0 => self.zoom_out(),
                _ => {}
            }

            if input.consume_key(Modifiers::ALT, Key::Num0) {
                self.zoom_reset();
            }
            if input.consume_key(Modifiers::ALT, Key::ArrowUp) {
                self.zoom_in();
            }
            if input.consume_key(Modifiers::ALT, Key::ArrowDown) {
                self.zoom_out();
            }
        });
    }

    fn menu_bar(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.set_enabled(!self.any_dialog_shown());
            ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;

            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| self.file_menu(ui, frame));
                ui.menu_button("Edit", |ui| self.edit_menu(ui));
                ui.menu_button("View", |ui| self.view_menu(ui));

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("⚙").clicked() {
                        self.show_settings = true;
                    }
                });
            })
        });
    }

    fn file_menu(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        if ui.button("New").clicked() {}
        if ui.button("New window").clicked() {
            ui.close_menu();
            Command::new(current_exe().expect("Failed to get current exe path"))
                .spawn()
                .expect("Failed to start new app");
        }
        if ui.button("Open").clicked() {
            self.open_dialog_state.show = true;
            ui.close_menu();
        }
        if ui.button("Save").clicked() {
            self.save_dialog_state.show = true;
            ui.close_menu();
        }
        if ui.button("Save as").clicked() {
            self.save_dialog_state.show = true;
            ui.close_menu();
        }
        if ui.separator().clicked() {}
        if ui.button("Page setup").clicked() {}
        if ui.button("Print").clicked() {}
        if ui.separator().clicked() {}
        if ui.button("Exit").clicked() {
            frame.close();
        }
    }

    fn edit_menu(&mut self, ui: &mut Ui) {
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
    }

    fn view_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Zoom", |ui| self.zoom_menu(ui));
        ui.checkbox(&mut self.show_status_bar, "Status bar");
        ui.checkbox(&mut false, "Word wrap");
    }

    fn zoom_menu(&mut self, ui: &mut Ui) {
        if ui.button("Zoom in").clicked() {
            self.zoom_in();
            ui.close_menu();
        }
        if ui.button("Zoom out").clicked() {
            self.zoom_out();
            ui.close_menu();
        }
        if ui.button("Restore default zoom").clicked() {
            self.zoom_reset();
            ui.close_menu();
        }
    }

    fn status_bar(&self, ctx: &Context) {
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
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

    fn settings_panel(&mut self, ctx: &Context) {
        Self::central_scroll(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 8.0;
            if ui.button("Back").clicked() {
                self.show_settings = false;
            }
            ui.add_space(8.0);
            ui.heading("Settings");
            ui.label("App theme");
            widgets::global_dark_light_mode_buttons(ui);
            ui.add_space(8.0);
            ui.heading("About this app");
            ui.label(format!("Rustpad {}", env!("CARGO_PKG_VERSION")));
            ui.label("© 2022 Aidunlin");
        });
    }

    fn main_panel(&mut self, ctx: &Context) {
        Self::central_scroll(ctx, |ui| {
            ui.set_enabled(!self.any_dialog_shown());
            ui.centered_and_justified(|ui| {
                ui.style_mut()
                    .text_styles
                    .entry(Monospace)
                    .and_modify(|e| e.size = self.font_size * (self.zoom as f32 / 100.0));

                let output = TextEdit::multiline(&mut self.text)
                    .frame(false)
                    .code_editor()
                    .show(ui);

                output.response.context_menu(|ui| self.right_click_menu(ui));

                if let Some(cursor_range) = output.cursor_range {
                    let cursor = cursor_range.primary.pcursor;
                    self.cursor_line = cursor.paragraph + 1;
                    self.cursor_col = cursor.offset + 1;
                }
            });
        });
    }

    fn right_click_menu(&mut self, ui: &mut Ui) {
        if ui.button("Undo").clicked() {}
        ui.separator();
        if ui.button("Cut").clicked() {}
        if ui.button("Copy").clicked() {}
        if ui.button("Paste").clicked() {}
        if ui.button("Delete").clicked() {}
        ui.separator();
        if ui.button("Select all").clicked() {}
    }

    fn open_dialog(&mut self, ctx: &Context) {
        Window::new("Open")
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;
                ui.label("Open file");
                ui.text_edit_singleline(&mut self.open_dialog_state.text_edit);
                ui.horizontal(|ui| {
                    if ui.button("Open").clicked() {
                        match fs::read_to_string(&self.open_dialog_state.text_edit) {
                            Ok(string) => {
                                self.text = string;
                                self.open_dialog_state.show = false;
                            }
                            Err(e) => println!("Failed to open file: {}", e),
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.open_dialog_state.show = false;
                    }
                });
            });
    }

    fn save_dialog(&mut self, ctx: &Context) {
        Window::new("Save")
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;
                ui.label("Save file");
                ui.text_edit_singleline(&mut self.save_dialog_state.text_edit);
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        match fs::write(&self.save_dialog_state.text_edit, &self.text) {
                            Ok(_) => self.save_dialog_state.show = false,
                            Err(e) => println!("Failed to save file: {}", e),
                        };
                    }
                    if ui.button("Cancel").clicked() {
                        self.save_dialog_state.show = false;
                    }
                });
            });
    }
}

impl App for RustpadApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        for (text_style, font) in style.text_styles.iter_mut() {
            match text_style {
                Body | Monospace | Button => font.size = 16.0,
                Heading => font.size = 32.0,
                _ => {}
            }
        }
        style.spacing.button_padding = Vec2::new(8.0, 8.0);
        ctx.set_style(style);

        self.handle_zoom_inputs(ctx);

        if self.open_dialog_state.show {
            self.open_dialog(ctx);
        }
        if self.save_dialog_state.show {
            self.save_dialog(ctx);
        }

        if self.show_settings {
            self.settings_panel(ctx);
        } else {
            self.menu_bar(ctx, frame);
            if self.show_status_bar {
                self.status_bar(ctx);
            }
            self.main_panel(ctx);
        }
    }
}
