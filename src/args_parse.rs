// SPDX-License-Identifier: MIT
// © 2024 Apostolos Chalis, George Fakidis
use std::env;
use std::collections::HashMap;


 

/// Parses arguments from the command line into a HashMap like --property=value --property2=value2
pub fn parse_args()->HashMap<String,String> {
    println!("Parsing args from command line");
    let args: Vec<_> = env::args().collect();
    if args.len() ==  1 {
        println!("We do not have arguments so run with default options");
    }
    let mut program_args:HashMap<String,String> = HashMap::new();
    let mut skipped_iter = args.iter();
    skipped_iter.next();
    //skip the directory from which it was executed
    for arg in skipped_iter {
        //i probably do too many copies generally, cos String is owned where i could just use &str
        let mut split_vals = arg.split("=").take(2);
        let argname = split_vals.next();
        let argval = split_vals.next();

        if let Some(name) = argname {
            let parsed_name = name.to_string().replace("--","");
            //if there is no value i still want the key to be there for flags
            if let Some(val) = argval {
                program_args.insert(parsed_name ,val.to_string());
             } else {
                program_args.insert(parsed_name,"".to_string());
             }
            
        }
        
        println!("Arg passed: {:#?} --> {:#?}",argname,argval);
    }
    println!("Parsed args from program {:?}",program_args);
    return program_args;
}
