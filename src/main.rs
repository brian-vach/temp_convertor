use eframe::egui;
use temperature::{c_to_f, f_to_c};
mod temperature;

struct TemperatureConvertor {
    temperature: f32,
    unit: char,
}

impl Default for TemperatureConvertor {
    fn default() -> Self {
        Self {
            temperature: 0.0,
            unit: 'C',
        }
    }
}

impl eframe::App for TemperatureConvertor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(3.0);
            ui.heading("Temperature Convertor");

            let (min, max) = match self.unit {
                'C' => (-100.0, 100.0),
                'F' => (-148.0, 212.0),
                _ => (0.0, 0.0),
            };

            ui.add(egui::Slider::new(&mut self.temperature, min..=max).text("Temperature"));
            ui.label(format!("{}°{}", self.temperature, self.unit));

            ui.horizontal(|ui| {
                if ui.button("°C").clicked() {
                    if self.unit == 'F' {
                        self.temperature = f_to_c(self.temperature).round();
                    }
                    self.unit = 'C';
                    self.temperature = self.temperature.clamp(-100.0, 100.0)
                }
                if ui.button("°F").clicked() {
                    if self.unit == 'C' {
                        self.temperature = c_to_f(self.temperature).round();
                    }
                    self.unit = 'F';
                    self.temperature = self.temperature.clamp(-148.0, 212.0)
                }
            });

            ui.separator();

            let converted_temperature = match self.unit {
                'C' => c_to_f(self.temperature),
                'F' => f_to_c(self.temperature),
                _ => self.temperature,
            };

            ui.label(format!(
                "Converted Temperature: {:.1}°{}",
                converted_temperature,
                match self.unit {
                    'C' => 'F',
                    'F' => 'C',
                    _ => '?',
                }
            ));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Temperature Convertor",
        options,
        Box::new(|_| Ok(Box::new(TemperatureConvertor::default()))),
    )
}
