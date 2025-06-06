use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust GUI Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    num1: String,
    num2: String,
    result: String,
    operator: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust GUI Calculator");

            ui.horizontal(|ui| {
                ui.label("Number 1:");
                ui.text_edit_singleline(&mut self.num1);
            });

            ui.horizontal(|ui| {
                ui.label("Operator (+ - * /):");
                ui.text_edit_singleline(&mut self.operator);
            });

            ui.horizontal(|ui| {
                ui.label("Number 2:");
                ui.text_edit_singleline(&mut self.num2);
            });

            if ui.button("Calculate").clicked() {
                let n1 = self.num1.trim().parse::<f64>();
                let n2 = self.num2.trim().parse::<f64>();

                match (n1, n2, self.operator.trim()) {
                    (Ok(a), Ok(b), "+") => self.result = (a + b).to_string(),
                    (Ok(a), Ok(b), "-") => self.result = (a - b).to_string(),
                    (Ok(a), Ok(b), "*") => self.result = (a * b).to_string(),
                    (Ok(a), Ok(b), "/") => {
                        if b == 0.0 {
                            self.result = "Error: Division by zero".to_string()
                        } else {
                            self.result = (a / b).to_string()
                        }
                    }
                    _ => self.result = "Invalid input".to_string(),
                }
            }

            ui.separator();
            ui.label(format!("Result: {}", self.result));
        });
    }
}
