// SPDX-License-Identifier: MIT
// © 2024 Apostolos Chalis, George Fakidis
use std::fs::File;
use std::io::{self, BufRead};
use std::thread::sleep;
use std::time::Duration;

pub fn get_cpu_util() -> io::Result<()> {
    let mut last_idle = 0.0;
    let mut last_total = 0.0;

    loop {
        let file = File::open("/proc/stat")?;
        let reader = io::BufReader::new(file);
        let line = reader.lines().next().unwrap()?;
        let fields: Vec<f64> = line.split_whitespace()
                                   .skip(1)
                                   .map(|s| s.parse().unwrap())
                                   .collect();

        let idle = fields[3];
        let total: f64 = fields.iter().sum();

        let idle_delta = idle - last_idle;
        let total_delta = total - last_total;

        last_idle = idle;
        last_total = total;

        let utilisation = 100.0 * (1.0 - idle_delta / total_delta);
        print!("{:5.1}%", utilisation);
        print!("\r");

        sleep(Duration::from_secs(5));
    }
}

