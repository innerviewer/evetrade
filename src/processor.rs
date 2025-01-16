use log::{debug, info};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::esi::ESI;
use crate::route::Route;
use crate::settings::SETTINGS;
use crate::types::{Order, OrderGroup, System, Type};

#[derive(Debug)]
struct PreprocessStats {
    initial_types: usize,
    removed_empty: usize,
    removed_volume: usize,
    removed_price: usize,
    removed_unprofitable: usize,
    final_types: usize,
}

pub struct OrderProcessor<'a> {
    orders: &'a mut HashMap<u32, OrderGroup>,
    systems: &'a HashMap<u32, System>,
    types: &'a HashMap<u32, Type>,
    mean_jump_distance: f64,
    cargo_volume: f32,
    initial_capital: f32,
    percentage_treshold: f32,
    max_jumps: u16,
}

impl<'a> OrderProcessor<'a> {
    pub fn new(
        orders: &'a mut HashMap<u32, OrderGroup>,
        systems: &'a HashMap<u32, System>,
        types: &'a HashMap<u32, Type>,
        mean_jump_distance: f64,
    ) -> Self {
        let settings = SETTINGS.lock().unwrap();
        let initial_capital = settings.get_initial_capital();
        let cargo_volume = settings.get_ship_cargo_volume();
        let percentage_treshold = settings.get_percentage_treshold();
        let max_jumps = settings.get_max_jumps();

        OrderProcessor {
            orders: orders,
            systems: systems,
            types: types,
            mean_jump_distance: mean_jump_distance,
            cargo_volume: cargo_volume,
            initial_capital: initial_capital,
            percentage_treshold: percentage_treshold,
            max_jumps: max_jumps,
        }
    }

    pub fn compute(&mut self) -> Vec<Route> {
        let mut routes: Vec<Route> = Vec::new();

        info!("Preprocessing orders...");
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let stats = self.preprocess_orders();
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        info!("Processing orders took {:?}ms", (end - start).as_millis());
        debug!("Preprocessing results: {:?}", stats);

        // for order in &self.orders {
        //     let mut route = Route::new();
        //     route.add_order(order.clone());
        //     routes.push(route);
        // }

        routes
    }

    fn preprocess_orders(&mut self) -> PreprocessStats {
        let mut stats = PreprocessStats {
            initial_types: self.orders.len(),
            removed_empty: 0,
            removed_volume: 0,
            removed_price: 0,
            removed_unprofitable: 0,
            final_types: 0,
        };

        // First pass: Remove obviously invalid orders and sort them
        let type_ids: Vec<_> = self.orders.keys().cloned().collect();
        for type_id in type_ids {
            if let Some(order_group) = self.orders.get_mut(&type_id) {
                if order_group.buy.is_empty() || order_group.sell.is_empty() {
                    self.orders.remove(&type_id);
                    stats.removed_empty += 1;
                    continue;
                }

                // Sort orders by price (descending for buy, ascending for sell)
                order_group
                    .buy
                    .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
                order_group
                    .sell
                    .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

                // Check item type constraints
                if let Some(item_type) = self.types.get(&type_id) {
                    if item_type.volume > self.cargo_volume {
                        self.orders.remove(&type_id);
                        stats.removed_volume += 1;
                        continue;
                    }
                } else {
                    self.orders.remove(&type_id);
                    continue;
                }
            }

            if let Some(order_group) = self.orders.get_mut(&type_id) {
                let mut truncate_buy_at = order_group.buy.len();
                let mut truncate_sell_at = order_group.sell.len();

                'outer: for (i, buy_order) in order_group.buy.iter().enumerate() {
                    for (k, sell_order) in order_group.sell.iter().enumerate() {
                        let profit_ratio = (buy_order.price - sell_order.price) / sell_order.price;

                        // Debug print for profit check
                        if i == 0 && k == 0 {
                            println!("Type {} - First profit ratio: {}", type_id, profit_ratio);
                        }

                        if profit_ratio < 0.1 {
                            // Using 0.1 as in your Python code
                            truncate_buy_at = i;
                            truncate_sell_at = k;
                            println!("Type {} - Truncating at buy: {}, sell: {}", type_id, i, k);
                            break 'outer;
                        }
                    }
                }

                order_group.buy.truncate(truncate_buy_at);
                order_group.sell.truncate(truncate_sell_at);

                // Debug print final state
                println!(
                    "Type {} - After truncate: {} buy, {} sell",
                    type_id,
                    order_group.buy.len(),
                    order_group.sell.len()
                );
            }
        }

        self.orders
            .retain(|_, group| !group.buy.is_empty() && !group.sell.is_empty());
        //*self.orders = profitable_orders;
        stats.final_types = self.orders.len();

        stats
    }

    // Helper method to calculate maximum possible units for a trade
    fn calculate_max_units(&self, type_id: u32, buy_order: &Order, sell_order: &Order) -> i32 {
        let type_volume = self
            .types
            .get(&type_id)
            .map(|t| t.volume)
            .unwrap_or(f32::MAX);
        let volume_limit = (self.cargo_volume / type_volume);
        let capital_limit = (self.initial_capital / sell_order.price);

        volume_limit
            .min(capital_limit)
            .min(buy_order.volume)
            .min(sell_order.volume)
            .floor() as i32
    }
}

/*
def process_orders(self) -> list[Route]:
    """
    Find profitable trade routes considering:
    - Ship cargo volume
    - Initial capital (which can increase during trading)
    - Maximum number of jumps
    - Security threshold
    - Profit percentage threshold

    Returns a list of Route objects representing profitable trading paths
    """
    routes = []
    visited_systems = set()

    # Priority queue for route candidates
    # Priority is -profit_per_jump to use min-heap as max-heap
    candidates = []

    # Initialize with potential starting points
    # Look for sell orders that we can afford
    for type_id in self.orders:
        type_volume = self.types[type_id].volume
        if type_volume > self.max_cargo:
            continue

        for system_id in self.graph:
            sell_orders = self.graph[system_id][type_id]['sell_orders']
            if not sell_orders:
                continue

            best_sell = sell_orders[0]  # Already sorted by price ascending
            if best_sell.price * (self.max_cargo // type_volume) <= self.initial_capital:
                heappush(candidates, (0, system_id, self.initial_capital, [], set()))

    while candidates and len(routes) < 10:  # Limit to top 10 routes
        _, current_system, current_capital, path, visited = heappop(candidates)

        if len(path) // 2 >= self.max_jumps:  # Check if we've exceeded max jumps
            continue

        # Try to find profitable trades from current system
        for type_id in self.orders:
            type_volume = self.types[type_id].volume
            max_units = min(self.max_cargo // type_volume,
                          current_capital // min(o.price for o in self.graph[current_system][type_id]['sell_orders'] if o))

            if max_units == 0:
                continue

            # Find best sell order in current system
            sell_orders = [o for o in self.graph[current_system][type_id]['sell_orders']
                         if o.volume >= max_units and o.price * max_units <= current_capital]

            if not sell_orders:
                continue

            sell_order = sell_orders[0]  # Lowest price

            # Look for profitable buy orders in other systems
            for dest_system in self.graph:
                if dest_system == current_system or dest_system in visited:
                    continue

                if self.systems[dest_system].security_status < Settings.security_treshold:
                    continue

                buy_orders = [o for o in self.graph[dest_system][type_id]['buy_orders']
                            if o.volume >= max_units]

                if not buy_orders:
                    continue

                buy_order = buy_orders[0]  # Highest price (sorted descending)

                # Calculate potential profit
                purchase_cost = sell_order.price * max_units
                sell_revenue = buy_order.price * max_units
                potential_profit = sell_revenue - purchase_cost

                # Check profit percentage threshold
                if Utils.get_profit_percentage(sell_order.price, buy_order.price) < Settings.percentage_treshold:
                    continue

                # Get path between systems
                system_path = self.pathfinder.compute_path(current_system, dest_system)
                if not system_path:
                    continue

                jumps = len(system_path) - 1
                if len(path) // 2 + jumps > self.max_jumps:
                    continue

                profit_per_jump = potential_profit / jumps if jumps > 0 else 0

                # Create new route candidate
                new_path = list(path)
                new_path.extend([sell_order, *system_path])
                new_visited = set(visited)
                new_visited.add(dest_system)

                heappush(candidates,
                        (-profit_per_jump,
                         dest_system,
                         current_capital - purchase_cost + sell_revenue,
                         new_path,
                         new_visited))

        # If we have a valid path, create a route
        if path:
            route = Route()
            for item in path:
                if isinstance(item, Order):
                    route.add_order(item)
                else:
                    route.add_systems(item)
            routes.append(route)

    return routes
*/
