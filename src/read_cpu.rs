// Author: Apostolos Chalis 2024
use std::fs; 

pub fn read_proc_stat(){
    let proc_stat_content = fs::read_to_string("/proc/stat").expect("Error: Can not read /proc/stat"); 
    let proc_stat_content_vec: Vec<&str> = proc_stat_content.lines().collect(); // Get first line
    
    let mut cpu_data: Vec<u32> = Vec::new();

    for ch in proc_stat_content_vec[0].enumerate(){
        if ch.is_numeric(){
            cpu_data.push(ch); 
        }
    }

    println!("{:?}", cpu_data); 
}
