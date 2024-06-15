use std::{collections::HashMap, fs};


//no need to copy probably just dont know how to do in rust
#[derive(Debug)]
struct MemInfoLine {
    stat_name:String,
    stat_kb:u64
}

fn read_mem_info_line_amount(line: &str)->MemInfoLine{
    let mut line_iter = line.split(":");
    let name_of_memory_info = line_iter.next().expect("first element of line was not found while splitting by :");
    println!("Memory stat: {}",name_of_memory_info);
    let amount_part = line_iter.next().expect("second element of line was not found while splitting by :");
    let amount_trimmed = amount_part.trim_start();
    let mut amount_split = amount_trimmed.split(" ");
    let amount = amount_split.next().unwrap();
    let amount_u64:u64 = amount.parse().expect("Result could not be parsed");

    MemInfoLine {
        stat_kb:amount_u64,
        stat_name:name_of_memory_info.to_owned()
    }
}

///reads the /proc/meminfo file into a hashmap for later use
pub fn read_ram_of_system()->HashMap<String,u64> {
    let proc_meminfo_file_contents = fs::read_to_string("/proc/meminfo")
    .expect("/proc/meminfo could not be read, if you are on windows this tool does not work. If on linux, might have something to do with privileges, try sudo");
    let lines_of_file = proc_meminfo_file_contents.split("\n");
    println!("/proc/meminfo contents: {:?}",lines_of_file);
    let mut memory_info = HashMap::new();
    lines_of_file.filter(|l| !l.is_empty()).map(|l| read_mem_info_line_amount(l)).for_each(|m| {memory_info.insert(m.stat_name, m.stat_kb);});
    println!("Memory stats {:#?}",memory_info);

    return memory_info;
}

