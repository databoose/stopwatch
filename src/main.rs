use std::{thread, time};
use std::env;
use std::process;

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
    let mut is_there_a_comment;
    let mut comment;
    match args.len() {
        1 => is_there_a_comment = false,
        2 => {
            comment = &args[1];
            is_there_a_comment = true;
        },
        _ => {
            println!("Error : passed too many arguments");
            println!("Use the entire comment encased in quotation marks to set a comment");
            process::exit(0);
        }
    }

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
