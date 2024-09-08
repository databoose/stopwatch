use std::{thread, time};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut second: u16 = 0;
    let mut minute: u16 = 0;
    let mut hour: u16 = 0;
    let mut days: u8 = 0;

    loop {
        print!("\x1B[2J\x1B[1;1H"); // clear screen
        println!("{}d:{}h:{}m:{}s ({:?})", days, hour, minute, second, args[1]);
        second = second + 1;
        if second > 59 { // if greater than 59
            second = 0;
            if minute > 59 {
                minute = 0;
                if hour > 23 {
                	hour = 0;
                	days = days + 1;
                }
                else { hour = hour + 1;}
            }
            else { minute = minute + 1; }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
