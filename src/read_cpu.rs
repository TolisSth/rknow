// SPDX-License-Identifier: MIT
// © 2024 Apostolos Chalis, George Fakidis
use std::fs::read_to_string;

pub fn get_cpu_util() {
    // TODO: reimplement this whole reading mechanism
    // It has large performance cost
    let mut first_line_proc_stat: Vec<String> = Vec::new();

    for line in read_to_string("/proc/stat").unwrap().lines() {
        first_line_proc_stat.push(line.to_string());
        break; // Breaking the loop after 1 iteration because we
               // only need the first line
    }
}

