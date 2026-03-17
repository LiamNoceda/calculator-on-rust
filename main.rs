use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Scientific Rust Calc",
        options,
        Box::new(|_cc| Ok(Box::new(CalculatorApp::default()))),
    )
}

struct CalculatorApp {
    input: String,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self { input: "".to_owned() }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Scientific Calculator");

            ui.text_edit_singleline(&mut self.input);

            ui.add_space(10.0);

            egui::Grid::new("buttons_grid").spacing([10.0, 10.0]).show(ui, |ui| {
                if ui.button("7").clicked() { self.input.push('7'); }
                if ui.button("8").clicked() { self.input.push('8'); }
                if ui.button("9").clicked() { self.input.push('9'); }
                if ui.button("/").clicked() { self.input.push('/'); }
                ui.end_row();

                if ui.button("4").clicked() { self.input.push('4'); }
                if ui.button("5").clicked() { self.input.push('5'); }
                if ui.button("6").clicked() { self.input.push('6'); }
                if ui.button("*").clicked() { self.input.push('*'); }
                ui.end_row();

                if ui.button("1").clicked() { self.input.push('1'); }
                if ui.button("2").clicked() { self.input.push('2'); }
                if ui.button("3").clicked() { self.input.push('3'); }
                if ui.button("-").clicked() { self.input.push('-'); }
                ui.end_row();

                if ui.button("0").clicked() { self.input.push('0'); }
                if ui.button("(").clicked() { self.input.push('('); }
                if ui.button(")").clicked() { self.input.push(')'); }
                if ui.button("+").clicked() { self.input.push('+'); }
                ui.end_row();
                
                if ui.button("sin").clicked() { self.input.push_str("sin("); }
                if ui.button("cos").clicked() { self.input.push_str("cos("); }
                if ui.button("sqrt").clicked() { self.input.push_str("sqrt("); }
                if ui.button("pi").clicked() { self.input.push_str("pi"); }
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("Clear").clicked() {
                    self.input.clear();
                }

                if ui.button("=").clicked() {
                    match meval::eval_str(&self.input) {
                        Ok(res) => self.input = res.to_string(),
                        Err(_) => self.input = "Error".to_owned(),
                    }
                }
            });
        });
    }
}
