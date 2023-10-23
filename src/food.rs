pub struct Food {
    pub x: usize,
    pub y: usize,
    pub size: f64,
    pub energy: f64,
}

impl Food {
    pub fn new(x: usize, y: usize, size: f64, energy: f64, growth_rate: f64) -> Self {
        Food {
            x,
            y,
            size,
            energy,
            growth_rate,
            energy_density: energy / size,
        }
    }
}
