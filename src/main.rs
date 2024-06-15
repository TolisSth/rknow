// SPDX-License-Identifier: MIT
// © 2024 Apostolos Chalis, George Fakidis
use std::collections::HashMap;
mod args_parse;
mod read_cpu;
fn main() {
    println!("rknow ~ A Rust system profiler v0.1.0\nApostolos Chalis, George Fakidis 2024");
    read_cpu::read_proc_stat();
    let profiler_options = args_parse::parse_args();
    execute_tool(profiler_options);
}


fn execute_tool(options:HashMap<String,String>) { 
    //they will be both calling the same function with just some preprocessing before they do so
    if(options.is_empty()) {
        println!("Executing with default options");
        return ;
    }
    println!("Executing with non-default params");

    return ;

}

