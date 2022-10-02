#[derive(Default)]
pub struct Point{
    pub x: f64,
    pub y: f64
}

use egui::plot::PlotPoints;

#[derive(Default)]
pub struct ProjectileData{
    pub mass: f64,
    pub air_ressitance_coefficient: f64,
    pub initial_velocity: f64,
    pub gravity: f64,
    pub throwing_angle: f64,
    pub step_size: f64, //step size is related to how often the calculations are run. (The lower the more accurate approximation, but also slower)
    pub num_of_steps: u32, //Determines how long the simulation should run
    pub time: f64,
    pub x_velocity: f64,
    pub y_velocity: f64,
    pub x_position: f64,
    pub y_position: f64,
}

impl ProjectileData{
    //Function describing changes in x's velocity over time.
    pub fn f_dvx_dt(&self) -> f64 {
        -(self.air_ressitance_coefficient / self.mass) * self.x_velocity * (self.x_velocity.powi(2) + self.y_velocity.powi(2))
    }

    pub fn f_dvy_dt(&self) -> f64 {
        -(self.air_ressitance_coefficient / self.mass) * self.x_velocity * (self.x_velocity.powi(2) + self.y_velocity.powi(2)) -self.gravity
    }

    pub fn euler_method(&mut self) -> Point {
        self.x_position = self.x_position + self.step_size * self.x_velocity;
		self.y_position = self.y_position + self.step_size * self.y_velocity;

		let x_temp_velocity = self.x_velocity + self.f_dvx_dt() * self.step_size;
		let y_temp_velocity = self.y_velocity + self.f_dvy_dt() * self.step_size;

        self.x_velocity = x_temp_velocity;
        self.y_velocity = y_temp_velocity;

        Point {x: self.x_position, y: self.y_position}
    }
}