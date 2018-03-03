extern crate core;
extern crate regex;
extern crate time;
extern crate schedule_recv;

use std::process::Command;
use std::collections::HashSet;
use regex::Regex;

///Return a pid in a string that is the 
///end of line retured by netstat :
/// ```
///...26.23:443       CLOSE_WAIT      8688
///                                   ^^^^
///```
///Regex `(\d+)\r` match the last digits of the line.
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

/// Launch command `taskkill /F /PID <pid>` and
/// kill the proccess id if possible.
fn kill_pid(pid: String) {
    let output = Command::new("taskkill")
        .arg("/F").arg("/PID").arg(pid.clone())
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout);
        print!("{}", out);
    }
    else {
        println!("Failed to exec taskkill with pid {}", pid);
    }
}

/// Launch command `netstat -b -o -n`. For
/// all lines that contain svchost.exe call
/// the function `get_pid()`. Then if the
/// function return some pid add it to 
/// the hashset of string. Finally kill all
/// pids added by call `kill_pid()`.
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
        panic!("failed to execute netstat.");
    }
}

/// Create a loop that execute `kill_svchost()`
/// every 8 seconds.
fn main() {
    let tick = schedule_recv::periodic_ms(8000);
    loop {
        println!(">> Check for svchost instances <<");
        kill_svchost();
        tick.recv().unwrap();
    }
}