use std::sync::Mutex;

pub struct Settings {
    log_level: log::Level,
    update_universe_data: bool,
    percentage_treshold: f32,
    ship_cargo_volume: f32,
    max_jumps: u16,
    initial_capital: f32,
    security_treshold: f32,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            log_level: log::Level::Debug,
            update_universe_data: false,
            percentage_treshold: 10.0,
            ship_cargo_volume: 6300.0,
            max_jumps: 100,
            initial_capital: 50000000.0,
            security_treshold: -1.0,
        }
    }

    pub fn get_level(&self) -> log::Level {
        self.log_level
    }

    pub fn get_update_universe_data(&self) -> bool {
        self.update_universe_data
    }

    pub fn get_percentage_treshold(&self) -> f32 {
        self.percentage_treshold
    }

    pub fn get_max_jumps(&self) -> u16 {
        self.max_jumps
    }

    pub fn get_ship_cargo_volume(&self) -> f32 {
        self.ship_cargo_volume
    }

    pub fn get_initial_capital(&self) -> f32 {
        self.initial_capital
    }

    pub fn set_level(&mut self, value: log::Level) {
        self.log_level = value;
    }
}

lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::new());
}
