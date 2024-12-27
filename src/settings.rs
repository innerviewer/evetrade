use std::sync::Mutex;

pub struct Settings {
    log_level: log::Level,
    update_universe_data: bool,
    percentage_treshold: u8,
    ship_cargo_volume: u32,
    max_jumps: u16,
    initial_capital: u64,
    security_treshold: f32,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            log_level: log::Level::Debug,
            update_universe_data: false,
            percentage_treshold: 20,
            ship_cargo_volume: 6300,
            max_jumps: 100,
            initial_capital: 50000000,
            security_treshold: -1.0,
        }
    }

    pub fn set_level(&mut self, value: log::Level) {
        self.log_level = value;
    }

    pub fn get_level(&self) -> log::Level {
        self.log_level
    }

    pub fn get_update_universe_data(&self) -> bool {
        self.update_universe_data
    }
}

lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::new());
}
