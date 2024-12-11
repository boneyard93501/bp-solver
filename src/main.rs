#[derive(Debug)]
struct Server {
    core_capacity: u32,
    disk_capacity: u32,
    orders: Vec<(u32, u32)>, // Each order is (cores, disk space)
}

impl Server {
    // this info needs to come from the indexer or chain
    fn new(core_capacity: u32, disk_capacity: u32) -> Self {
        Server {
            core_capacity,
            disk_capacity,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, cores: u32, disk: u32) -> bool {
        if self.remaining_cores() >= cores && self.remaining_disk_space() >= disk {
            self.orders.push((cores, disk));
            true
        } else {
            false
        }
    }
    fn remaining_cores(&self) -> u32 {
        self.core_capacity - self.orders.iter().map(|(c, _)| c).sum::<u32>()
    }

    /// Get the remaining disk capacity of the server.
    fn remaining_disk_space(&self) -> u32 {
        self.disk_capacity - self.orders.iter().map(|(_, d)| d).sum::<u32>()
    }
}

fn bin_packing_ffd_multi(
    orders: Vec<(u32, u32)>,
    core_capacity: u32,
    disk_capacity: u32,
) -> Vec<Server> {
    
    let mut sorted_orders = orders.clone();
    sorted_orders.sort_unstable_by(|a, b| b.cmp(a));
    
    let mut servers: Vec<Server> = Vec::new();

    for (cores, disk) in sorted_orders {
        let mut placed = false;
        for server in &mut servers {
            if server.add_order(cores, disk) {
                placed = true;
                break;
            }
        }
        if !placed {
            // Create a new server if the order didn't fit in any existing server.
            let mut new_server = Server::new(core_capacity, disk_capacity);
            new_server.add_order(cores, disk);
            servers.push(new_server);
        }
    }

    servers
}

fn main() {
    // contrived example to actually make anything happen
    let orders = vec![
        (4, 100), 
        (2, 50),
        (6, 150),
        (1, 30),
        (3, 80),
        (5, 120),
        (2, 60),
        (4, 90),
    ];
    let core_capacity = 10; // Each server has 10 cores.
    let disk_capacity = 200; // Each server has 200 GB of disk space.


    let servers = bin_packing_ffd_multi(orders, core_capacity, disk_capacity);

    for (i, server) in servers.iter().enumerate() {
        println!(
            "Server {}: {:?}, Remaining Cores: {}, Remaining Disk: {}",
            i + 1,
            server.orders,
            server.remaining_cores(),
            server.remaining_disk_space()
        );
    }
}
