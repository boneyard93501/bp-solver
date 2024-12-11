use std::collections::VecDeque;

// Weighted First-Fit Decreasing (FFD) heuristic
// https://en.wikipedia.org/wiki/First-fit_bin_packing




#[derive(Debug)]
struct Bin {
    core_capacity: u32,
    disk_capacity: u32,
    items: Vec<(u32, u32)>, // Each item is (cores, disk space)
}

impl Bin {
    fn new(core_capacity: u32, disk_capacity: u32) -> Self {
        Bin {
            core_capacity,
            disk_capacity,
            items: Vec::new(),
        }
    }

    fn add_item(&mut self, cores: u32, disk: u32, core_weight: f32, disk_weight: f32) -> bool {
        let core_remaining = self.remaining_core_capacity() as f32;
        let disk_remaining = self.remaining_disk_capacity() as f32;
        let core_needed = cores as f32;
        let disk_needed = disk as f32;

        // Check if adding the item fits within weighted capacities.
        if core_remaining >= core_needed * core_weight && disk_remaining >= disk_needed * disk_weight {
            self.items.push((cores, disk));
            true
        } else {
            false
        }
    }

    fn remaining_core_capacity(&self) -> u32 {
        self.core_capacity.saturating_sub(self.items.iter().map(|(c, _)| c).sum::<u32>())
    }

    fn remaining_disk_capacity(&self) -> u32 {
        self.disk_capacity.saturating_sub(self.items.iter().map(|(_, d)| d).sum::<u32>())
    }
}


fn bin_packing_weighted_ffd(
    items: Vec<(u32, u32)>,
    core_capacity: u32,
    disk_capacity: u32,
    core_weight: f32,
    disk_weight: f32,
) -> Vec<Bin> {
    assert!((core_weight + disk_weight - 1.0).abs() < 1e-6, "Weights must sum to 1.0");

    let mut sorted_items = items.clone();
    sorted_items.sort_unstable_by(|a, b| {
        let a_value = (a.0 as f32) * core_weight + (a.1 as f32) * disk_weight;
        let b_value = (b.0 as f32) * core_weight + (b.1 as f32) * disk_weight;
        b_value.partial_cmp(&a_value).unwrap()
    });

    let mut bins: Vec<Bin> = Vec::new();

    for (cores, disk) in sorted_items {
        let mut placed = false;
        for bin in &mut bins {
            if bin.add_item(cores, disk, core_weight, disk_weight) {
                placed = true;
                break;
            }
        }
        if !placed {
            // Create a new bin if the item didn't fit in any existing bin.
            let mut new_bin = Bin::new(core_capacity, disk_capacity);
            new_bin.add_item(cores, disk, core_weight, disk_weight);
            bins.push(new_bin);
        }
    }

    bins
}

fn main() {
    // Example items and bin capacities.
    let items = vec![
        (4, 100), // 4 cores, 100 GB
        (2, 50),
        (6, 150),
        (1, 30),
        (3, 80),
        (5, 120),
        (2, 60),
        (4, 90),
    ];
    let core_capacity = 10; // Each server/bin has 10 cores.
    let disk_capacity = 200; // Each server/bin has 200 GB of disk space.


    let core_weight = 0.6; // 60% priority to core usage.
    let disk_weight = 0.4; // 40% priority to disk usage.

    println!("\ncore_weight: {}\ndisk_weight: {}\n", core_weight, disk_weight);
    let bins = bin_packing_weighted_ffd(items.clone(), core_capacity, disk_capacity, core_weight, disk_weight);
    for (i, bin) in bins.iter().enumerate() {
        println!(
            "Bin {}: {:?}, Remaining Cores: {}, Remaining Disk: {}",
            i + 1,
            bin.items,
            bin.remaining_core_capacity(),
            bin.remaining_disk_capacity()
        );
    }

    let core_weight = 0.2; // 60% priority to core usage.
    let disk_weight = 0.8; // 40% priority to disk usage.
    println!("\ncore_weight: {}\ndisk_weight: {}\n", core_weight, disk_weight);
    let bins = bin_packing_weighted_ffd(items, core_capacity, disk_capacity, core_weight, disk_weight);
    for (i, bin) in bins.iter().enumerate() {
        println!(
            "Bin {}: {:?}, Remaining Cores: {}, Remaining Disk: {}",
            i + 1,
            bin.items,
            bin.remaining_core_capacity(),
            bin.remaining_disk_capacity()
        );
    }
}
