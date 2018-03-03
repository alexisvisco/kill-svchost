extern crate core;
extern crate regex;
extern crate time;
extern crate schedule_recv;

use std::process::Command;
use std::collections::HashSet;
use regex::Regex;

fn get_pid(vec: Vec<&str>, i: usize) -> Option<String> {
    let mut index = i - 1;
    loop {
        let line = vec[index];
        if line.contains("TCP") {
            let re = Regex::new(r"(\d+)\r").unwrap();
            for cap in re.captures_iter(line) {
                return Some(String::from(&cap[1]))
            }
        }
        if index > 0 {
            index -= 1;
        }
        else {
            break;
        }
    }
    None
}

fn kill_pid(pid: String) {
    let output = Command::new("taskkill")
        .arg("/F").arg("/PID").arg(pid.clone())
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout);
        println!("{}", out);
    }
    else {
        println!("Failed to exec taskkill with pid {}", pid);
    }
}

fn kill_svchost()
{
    let output = Command::new("netstat")
        .arg("-b").arg("-o").arg("-n")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    
    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout);
        let splitted: Vec<&str> = out.split("\n").collect();
        let mut new_set : HashSet<String> = HashSet::new();
        for i in 0 .. splitted.len() - 1 {
            let s : &str = splitted[i];
            if s.contains("svchost.exe") {
                match get_pid(splitted.clone(), i)  {
                    Some(pid) => {
                        new_set.insert(pid);
                    },
                    None => {},
                }
            }
        }
        for pid in new_set.iter() {
            kill_pid(pid.clone());
        }
    } else {
        println!("netstat failed and stderr was:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn main() {
    let tick = schedule_recv::periodic_ms(8000);
    loop {
        println!("Check for svchost instances..");
        kill_svchost();
        tick.recv().unwrap();
    }
}