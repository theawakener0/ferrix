use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use eframe::egui;

struct Ferrix {
    path: String,
    filename: String,
    code: String,
    output_text: String,
    is_running: Arc<AtomicBool>,
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
            output_text: String::new(),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl eframe::App for Ferrix {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ferrix");
            ui.horizontal(|ui| {
                ui.label("Filename: ");
                ui.text_edit_singleline(&mut self.filename);
            });
            ui.add_sized(
                ui.available_size() - egui::vec2(0.0, 60.0),
                egui::TextEdit::multiline(&mut self.code),
            );

            if ui.button("Run").clicked() && !self.is_running.load(Ordering::SeqCst) {
                let filename = self.filename.clone();
                let code = self.code.clone();
                let path = self.path.clone();
                let is_running = Arc::clone(&self.is_running);
                let output: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

                is_running.store(true, Ordering::SeqCst);

                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    {
                        let mut out = output.lock().unwrap();
                        if let Err(e) = fs::write(format!("{}/{}", path, filename), &code) {
                            *out = format!("Failed to write file: {}", e);
                            is_running.store(false, Ordering::SeqCst);
                            ctx.request_repaint();
                            return;
                        }
                    }

                    let compile_output = Command::new("rustc")
                        .arg(&filename)
                        .arg("-o")
                        .arg("main")
                        .output()
                        .expect("Failed to run rustc");

                    let result = if compile_output.status.success() {
                        let run_output = Command::new("./main")
                            .output()
                            .expect("Failed to run binary");
                        if run_output.status.success() {
                            String::from_utf8_lossy(&run_output.stdout).to_string()
                        } else {
                            format!(
                                "Runtime error:\n{}",
                                String::from_utf8_lossy(&run_output.stderr)
                            )
                        }
                    } else {
                        format!(
                            "Compilation error:\n{}",
                            String::from_utf8_lossy(&compile_output.stderr)
                        )
                    };

                    *output.lock().unwrap() = result;
                    is_running.store(false, Ordering::SeqCst);
                    ctx.request_repaint();
                });

                self.output_text.clear();
            }

            if self.is_running.load(Ordering::SeqCst) {
                ui.spinner();
            } else if !self.output_text.is_empty() {
                ui.separator();
                ui.label("Output:");
                ui.text_edit_multiline(&mut self.output_text.clone());
            }
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
        Box::new(|_cc| Ok(Box::<Ferrix>::default())),
    )
}
