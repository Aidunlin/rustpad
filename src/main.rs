use std::{env::current_exe, process::Command};

use eframe::{
    egui::{Align, CentralPanel, Context, Layout, ScrollArea, TextEdit, TopBottomPanel},
    run_native, App, Frame, NativeOptions,
};

fn main() {
    run_native(
        "Rustpad",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(RustpadApp::default())),
    );
}

#[derive(Default)]
struct RustpadApp {
    text: String,
}

impl App for RustpadApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top").show(ctx, |ui| {
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
                        if ui.button("Zoom in").clicked() {}
                        if ui.button("Zoom out").clicked() {}
                        if ui.button("Restore default zoom").clicked() {}
                    });
                    ui.checkbox(&mut false, "Status bar");
                    ui.checkbox(&mut false, "Word wrap");
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Settings").clicked() {}
                });
            })
        });

        TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Ln {}, Col {}", 1, 1));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("UTF-8");
                    ui.separator();
                    ui.label("Windows (CRLF)");
                    ui.separator();
                    ui.label(format!("{}%", 100));
                    ui.separator();
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(&mut self.text)
                        .frame(false)
                        .code_editor(),
                )
                .context_menu(|ui| {
                    if ui.button("Undo").clicked() {}
                    ui.separator();
                    if ui.button("Cut").clicked() {}
                    if ui.button("Copy").clicked() {}
                    if ui.button("Paste").clicked() {}
                    if ui.button("Delete").clicked() {}
                    ui.separator();
                    if ui.button("Select all").clicked() {}
                    ui.separator();
                    ui.checkbox(&mut false, "Right-to-left reading order");
                    ui.checkbox(&mut false, "Show Unicode control characters");
                    // ui.menu_button("Insert Unicode control characters", |ui| {});
                    ui.separator();
                    if ui.button("Reconversion").clicked() {}
                });
            });
        });
    }
}
