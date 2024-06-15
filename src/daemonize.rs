// SPDX-License-Identifier: MIT
// © 2024 Apostolos Chalis, George Fakidis
// libc stuff
use libc::umask; 
use libc::dup2; 
use libc::chdir; 
use libc::open; 
use libc::close; 
use libc::fork; 
use libc::signal; 
use libc::setsid; 
use libc::O_RDWR; 

// STD (C) 
use libc::STDERR_FILENO;
use libc::STDOUT_FILENO;
use libc::STDIN_FILENO;

// STD (Rust)
use std::process::exit; 
use std::ffi::CString; 

// Signals
use libc::SIG_IGN; 
use libc::SIGCHLD; 
use libc::SIGHUP; 


pub fn daemonize(){
    unsafe{
        let mut pid = fork(); 

        if pid < 0 {
            println!("ERROR: Fork failed "); 
            exit(1); 
        }
        
        // Terminate parent process
        if pid > 0 {
            exit(0);
        }
        
        // On success child is now session leader
        if setsid() < 0{
            exit(1); 
        }

        signal(SIGCHLD, SIG_IGN); 
        signal(SIGHUP, SIG_IGN); 

        pid = fork(); 

        if pid < 0 {
            println!("ERROR: Fork failed "); 
            exit(1); 
        }
        
        // Terminate parent process
        if pid > 0 {
            exit(0);
        }

        umask(0);

        if chdir(CString::new("/").unwrap().as_ptr()) < 0 {
            exit(1);
        }

        let dev_null = CString::new("/dev/null").unwrap();
        let fd = open(dev_null.as_ptr(), O_RDWR);
        if fd < 0 {
            exit(1);
        }
         if dup2(fd, STDIN_FILENO) < 0 {
            exit(1);
        }

        if dup2(fd, STDOUT_FILENO) < 0 {
            exit(1);
        }

        if dup2(fd, STDERR_FILENO) < 0 {
            exit(1);
        }

        if fd > STDERR_FILENO {
            close(fd);
        }
    } 
}
