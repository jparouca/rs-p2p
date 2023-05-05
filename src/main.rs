use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(450.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "just markdown",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    name: String,
    age: u32,
    input_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            input_text: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui(ui.available_size(), |ui| {
                ui.horizontal(|ui| {
                    ui.text_edit_multiline(&mut self.input_text)
                });
            });
        });
    }
}
