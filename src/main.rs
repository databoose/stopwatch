use std::{thread, time};

fn clearscreen() {
    thread::spawn(|| {
    loop {
        thread::sleep(time::Duration::from_millis(700));
        print!("\x1B[2J\x1B[1;1H");
    }
    });
}

fn main() {
    let mut second: i16 = 0;
    let mut minute: i16 = 0;
    let mut hour: i16 = 0;
    clearscreen();

    loop {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}h:{}m:{}s", hour, minute, second);
        second = second + 1;
        if second > 59 { // if greater than 59
            second = 0;
            minute = minute + 1;
            if minute > 59 {
                minute = 0;
                hour = hour + 1;
            }
        }
    }
}
