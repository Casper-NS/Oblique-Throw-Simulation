#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{iter::{MapWhile, Map}, f64::consts::PI};

// hide console window on Windows in release
use eframe::{egui};
use egui::plot::{Line, Plot, PlotPoints };
use projectile::Point;

mod projectile;
pub use crate::projectile::ProjectileData;

fn main() {
    let _projectile = ProjectileData {
        mass: 0.005,
        air_ressitance_coefficient: 0.0011,
        initial_velocity: 10.0,
        gravity: 9.82,
        throwing_angle:  45.0,
        step_size: 0.005,
        num_of_steps: 10000,
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
    projectile: ProjectileData,
}

impl Default for App {
    fn default() -> Self {
        Self {
            projectile: ProjectileData::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("data_panal").show(ctx, |ui|{
            ui.heading("Throw Variables");
            ui.add(egui::Slider::new(&mut self.projectile.mass, 0.0..=10.0)
                .smart_aim(false)
                .fixed_decimals(5)
                .step_by(0.00001)
                .text("Mass"));
                
            ui.add(egui::Slider::new(&mut self.projectile.air_ressitance_coefficient, 0.0..=1.0)
            .smart_aim(false)
            .fixed_decimals(5)
            .step_by(0.00001)
            .text("Ressistance-Coefficient"));
            
            ui.add(egui::Slider::new(&mut self.projectile.gravity, 0.0..=20.0)
            .smart_aim(false)
            .fixed_decimals(5)
            .step_by(0.00001)
            .text("Mass"));

            ui.add(egui::Slider::new(&mut self.projectile.throwing_angle, 0.0..=89.9)
            .smart_aim(false)
            .fixed_decimals(5)
            .step_by(0.00001)
            .text("Throwing Angle"));

            ui.add(egui::Slider::new(&mut self.projectile.initial_velocity, 0.0..=100.0)
            .smart_aim(false)
            .fixed_decimals(5)
            .step_by(0.00001)
            .text("Velocity"));

            ui.add(egui::Slider::new(&mut self.projectile.num_of_steps, 500..=100000)
            .fixed_decimals(1)
            .step_by(100.0)
            .text("Number of steps"));

            ui.add(egui::Slider::new(&mut self.projectile.step_size, 0.0..=1.0)
            .smart_aim(false)
            .fixed_decimals(5)
            .step_by(0.00001)
            .text("Step Size"));
        });

        egui::CentralPanel::default().show(ctx, |ui|
            {
                self.projectile.x_position = 0.0;
                self.projectile.y_position = 0.0;
                self.projectile.x_velocity = self.projectile.initial_velocity * (self.projectile.throwing_angle * PI / 180.0).cos();
                self.projectile.y_velocity = self.projectile.initial_velocity * (self.projectile.throwing_angle * PI / 180.0).sin();

                let mut i = 0;
                let mut throw = Vec::<Point>::new();

                while i < self.projectile.num_of_steps {
                    throw.push(self.projectile.euler_method());
                    i += 1;
                    if throw.last().unwrap().x <= 0.0 {
                        break;
                    }
                }

                let sin: PlotPoints = throw.iter().map(|point| {[point.x, point.y]}).collect();
                
                let line = Line::new(sin);
            
                let plot = Plot::new("Throw");

                plot.show(ui, |plotui|{
                    plotui.line(line)});

                throw.clear();
            });
        }
}

