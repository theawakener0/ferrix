use std::fs;
use std::process::Command;

use eframe::egui::{self, TextBuffer};

struct Ferrix {
    path: String,
    filename: String,
    code: String,
}

impl Default for Ferrix {
    fn default() -> Self {
        let code: &str = r#"
        fn main() {
            println!("Hello, world!");
        }
        "#;

        Self {
            path: String::from("."),
            filename: String::from("main.rs"),
            code: String::from(code),
        }
    }
}

impl eframe::App for Ferrix {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        fs::write(&self.path, &self.code).unwrap();

        if ui.button("Run").clicked() {
            let output = Command::new("rustc")
                .arg(&self.filename)
                .arg("-o")
                .arg("main")
                .output()
                .expect("Failed to run rustc");

            if output.status.success() {
                let run_output = Command::new("./main")
                    .output()
                    .expect("Failed to run binary");
                ui.label(String::from_utf8_lossy(&run_output.stdout));
            } else {
                ui.label(String::from_utf8_lossy(&output.stderr));
            }
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Ferrix");
            ui.horizontal(|ui| {
                ui.label("Filename: ");
                ui.text_edit_singleline(&mut self.filename);
            });
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.code),
            );
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Ok(Box::<Ferrix>::default())),
    )
}
