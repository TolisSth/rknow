// Author: Apostolos Chalis 2024
mod read_cpu; 
mod daemonize; 

fn main() {
    println!("rknow ~ A Rust system profiler v0.1.0\nApostolos Chalis 2024");

    println!("Starting daemon..."); 
    daemonize::daemonize();

    read_cpu::read_proc_stat(); 
}
