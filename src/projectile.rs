/*#[derive(Default)]
pub struct Point{
    x: f64,
    y: f64
}*/

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
    pub fn Fdvxdt(&self) -> f64 {
        -(self.air_ressitance_coefficient / self.mass) * self.x_velocity * (self.x_velocity)
    }
}