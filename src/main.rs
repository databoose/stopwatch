use std::{thread, time};
use std::env;

fn clearscreen() {
    thread::spawn(|| {
    loop {
        thread::sleep(time::Duration::from_millis(700));
        print!("\x1B[2J\x1B[1;1H");
    }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let comment = &args[1];

    let mut second: u16 = 0;
    let mut minute: u16 = 0;
    let mut hour: u16 = 0;
    let mut days: u8 = 0;
    clearscreen();

    loop {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}d:{}h:{}m:{}s ({})", days, hour, minute, second, comment);
        second = second + 1;
        if second > 59 { // if greater than 59
            second = 0;
            minute = minute + 1;
            if minute > 59 {
                minute = 0;
                hour = hour + 1;
                if hour > 24 {
                	hour = 0;
                	days = days + 1;
                }
            }
        }
    }
}
