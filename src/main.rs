// Author: Apostolos Chalis,George Fakidis 2024
use std::{collections::HashMap};
mod args_parse;
mod read_cpu;
fn main() {
    println!("rknow ~ A Rust system profiler v0.1.0\nApostolos Chalis 2024");
    read_cpu::read_proc_stat();
    let profiler_options = args_parse::parse_args();
    execute_tool(profiler_options);
}


fn should_daemonize(options:&HashMap<String,String> )->bool {
    return options.contains_key("d") || options.contains_key("D") || options.contains_key("daemon") || options.contains_key("daemonize");
}

fn show_cpu(options:&HashMap<String,String>) -> bool {
    return options.contains_key("c") || options.contains_key("C") || options.contains_key("cpu") || options.contains_key("CPU");
}

fn show_ram(options:&HashMap<String,String>) -> bool {
    return options.contains_key("r") || options.contains_key("R") || options.contains_key("ram") || options.contains_key("RAM");
}


fn show_help(options:&HashMap<String,String>) -> bool {
    return options.contains_key("h") || options.contains_key("H") || options.contains_key("help") || options.contains_key("HELP");
}

fn show_disk(options:&HashMap<String,String>) -> bool {
    return options.contains_key("d") || options.contains_key("D") || options.contains_key("disk") || options.contains_key("DISK");
}

fn help_message() {
    println!("
Welcome to rknow, a rust profiler written with 0 dependencies, specifically for linux systems, to be used in HPC Research   
Flags:
--cpu,--c,--C,--CPU ===> Prints information for the CPU
--ram,--r,--R,--RAM ===> Prints information for the ram
--help,--h,--H,--HELP ===> Prints help information
--disk,--d,--D,--DISK ===> Prints disk information [Not Implemented Yet]
    ");
}

fn run_as_daemon(cpu:bool,ram:bool,disk:bool) {
    
    println!("Running as daemon in the background");
    loop {
        if cpu {
            read_cpu::read_proc_stat();
        }

        if ram {
            println!("reading from ram");
        }

        if disk {
            println!("reading from disk");
        }
    }
    todo!("This is not yet properly written, only the boilerplate");
}

fn run_normally(cpu:bool,ram:bool,disk:bool){
    loop {
        if cpu {
            println!("{:?}",read_cpu::read_proc_stat());
        }

        if ram {
            println!("reading from ram");
        }

        if disk {
            println!("reading from disk");
        }
    }
}

fn execute_tool(options:HashMap<String,String>) { 
    //they will be both calling the same function with just some preprocessing before they do so
    if options.is_empty() {
        println!("Executing with default options");
        return ;
    }
    if show_help(&options) {
        help_message();
        return ;
    }
    let cpu = show_cpu(&options);
    let ram = show_ram(&options);
    let disk = show_disk(&options);
    if should_daemonize(&options) {
        run_as_daemon(cpu, ram, disk);
    }else {
        run_normally(cpu, ram, disk);
    }
   
    println!("Executing with non-default params");

    return ;

}

