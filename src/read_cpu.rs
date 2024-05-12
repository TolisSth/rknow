// Author: Apostolos Chalis 2024
use std::fs; 

pub fn read_proc_stat(){
    let proc_stat_content = fs::read_to_string("/proc/stat").expect("Error: Can not read /proc/stat"); 
    let mut proc_stat_first_line: String = Default::default(); 

    for ch in proc_stat_content.chars(){ 
        if ch != '\n'{
            proc_stat_first_line.push(ch);  
        }
        else{
            break; 
        }
    }
    
    println!("{}", proc_stat_first_line); 
}
