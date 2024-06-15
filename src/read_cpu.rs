// SPDX-License-Identifier: MIT
// © 2024 Apostolos Chalis, George Fakidis
use std::fs;

pub fn read_proc_stat() -> Vec<u32> {
    let proc_stat_content = fs::read_to_string("/proc/stat").expect("Error: Can not read /proc/stat");

    let mut proc_stat_first_line: String = Default::default();

    for ch in proc_stat_content.chars() {
        if ch != '\n' {
            proc_stat_first_line.push(ch);
        } else {
            break;
        }
    }

    let cpu_data: Vec<u32> = proc_stat_first_line
        .split(char::is_whitespace)
        .filter_map(|s| s.parse().ok())
        .collect();

    cpu_data 
}
