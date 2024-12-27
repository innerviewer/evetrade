use std::collections::HashMap;

use crate::esi::ESI;
use crate::route::Route;
use crate::types::{Order, System, Type};

pub struct OrderProcessor {
    orders: Vec<Order>,
    systems: HashMap<u32, System>,
    types: HashMap<u32, Type>,
    mean_jump_distance: f64,
}

impl OrderProcessor {
    pub fn new(
        orders: Vec<Order>,
        systems: HashMap<u32, System>,
        types: HashMap<u32, Type>,
        mean_jump_distance: f64,
    ) -> Self {
        OrderProcessor {
            orders: orders,
            systems: systems,
            types: types,
            mean_jump_distance: mean_jump_distance,
        }
    }

    pub fn compute(&mut self) -> Vec<Route> {
        let mut routes: Vec<Route> = Vec::new();

        for order in &self.orders {
            let mut route = Route::new();
            route.add_order(order.clone());
            routes.push(route);
        }

        routes
    }
}
