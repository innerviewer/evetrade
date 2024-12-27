use serde;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Stargate {
    pub origin: u32,
    pub destination: u32,
    pub weight: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct System {
    pub id: u32,
    pub name: String,
    pub security_status: f32,
    pub stargates: Vec<Stargate>,
    pub position: Vector3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Order {
    pub is_buy_order: bool,
    pub order_type: Type,
    pub price: f32,
    pub station_id: u32,
    pub system_id: u32,
    pub region_id: u32,
    pub volume: f32,
}

pub struct TradePair {
    pub buy_system_id: i32,
    pub sell_system_id: i32,
    pub type_id: i32,
    pub potential_profit: f32,
    pub volume: i32,
    pub buy_price: f32,
    pub sell_price: f32,
    pub jumps: i32,
    pub profit_per_jump: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Type {
    pub type_id: u32,
    pub group_id: u32,
    pub name: String,
    pub volume: f32,
}

#[derive(Clone)]
pub enum Waypoint {
    System(System),
    Order(Order),
}

pub struct State {
    pub priority: f32,
    pub cost: i32,
    pub value: i32,
    pub visited: Vec<bool>,
    pub current_node: i32,
    pub path: Vec<Waypoint>,
}

impl Vector3 {
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn similarity(&self, other: &Vector3, w1: f64, w2: f64) -> f64 {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();

        if mag1 == 0.0 || mag2 == 0.0 {
            return 0.0;
        }

        let cos_theta = self.dot(other) / (mag1 * mag2);
        let direction_similarity = (cos_theta + 1.0) / 2.0;

        let magnitude_similarity = mag1.min(mag2) / mag1.max(mag2);

        w1 * direction_similarity + w2 * magnitude_similarity
    }

    pub fn distance(&self, other: &Vector3) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }
}

pub struct TradeCandidate {
    pub profit_per_jump: f32,
    pub system_id: i32,
    pub capital: f32,
    pub waypoints: Vec<Waypoint>,
    pub visited: Vec<bool>,
}
