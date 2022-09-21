#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// hide console window on Windows in release
use eframe::{egui};
use egui::plot::{Line, Plot, PlotPoints };

mod projectile;
pub use crate::projectile::ProjectileData;

fn main() {
    let _projectile = ProjectileData {
        mass: 0.005,
        air_ressitance_coefficient: 0.0011,
        initial_velocity: 0.0,
        gravity: 9.82,
        throwing_angle:  45.0,
        step_size: 0.05,
        num_of_steps: 100,
        time: 0.0,
        x_velocity: 0.0,
        y_velocity: 0.0,
        x_position: 0.0,
        y_position: 0.0,
    };
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Air Ressitance Simulator",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}

struct App{
    name: String,
    age: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: "Name".to_owned(),
            age: 40,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("data_panal").show(ctx, |ui|{
            ui.heading("Throw Variables");
            ui.add(egui::Slider::new(&mut self.age, 0..=110).text("age"));
            if ui.button("Increment Year").clicked(){
                self.age += 1;
            }
        });
        egui::CentralPanel::default().show(ctx, |ui|
            {
                let sin: PlotPoints = (0..1000).map(|i| {
                    let x = i as f64 * 0.01;
                    [x, x.sin()]
                }).collect();
                
                let line = Line::new(sin);
            
                let plot = Plot::new("Throw");

                plot.show(ui, |plotui|{
                    plotui.line(line)});
            });
        }
}

