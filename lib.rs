use eframe;
use std::{fs, path};


pub fn file_select_dialog(title: &str, path: &mut String, ctx: &eframe::egui::Context, open: &mut bool) {
    eframe::egui::Window::new(title).open(open).show(ctx,|ui| {
        ui.add(eframe::egui::TextEdit::singleline(path));
        if ui.button("Return").clicked() {
            *path = path::PathBuf::from(&path).parent().unwrap().to_str().unwrap().to_string();
        }
        let iter_entry = fs::read_dir(path.clone());
        if let Ok(entries) = iter_entry {
            eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                for dir in entries {
                    if let Some(dir) = dir.unwrap().path().into_os_string().to_str() {
                        if ui.button(dir).clicked() {
                            *path = dir.to_string();
                        }
                    }
                }
            });
        }
    });
}