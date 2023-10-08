use std::io;

fn second_convert(start: u64, end: u64) -> (Time<u32>) {}
fn main() {
    println!("Welcome to the Second System Conversion tool.\nPlease enter a starting date and an ending date in the following format:\n mm/dd/yyyy/hh/mm/ss");
    let mut start = String::new();
    let mut end = String::new();
    println!("Start:");
    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read entry.");

    let mut start_data: Vec<u8> = Vec::new();
    let chars = start.chars();
    for item in chars {
        match item {
            '/' => continue,
            _ => match item.to_digit(10) {
                Some(num) => start_data.push(num as u8),
                None => println!("That is not a valid starting date."),
            },
        }
    }

    println!("End:");
    io::stdin()
        .read_line(&mut end)
        .expect("Failed to read entry.");

    let mut end_data: Vec<u8> = Vec::new();
    let chars = end.chars();
    for item in chars {
        match item {
            '/' => continue,
            _ => match item.to_digit(10) {
                Some(num) => end_data.push(num as u8),
                None => println!("That is not a valid ending date."),
            },
        }
    }

    let start_id: u64 = start_data.concat();
    let end_id: u64 = end_data.concat();

    println!(
        "The total elapsed time is as follows: {}",
        second_convert(start_id, end_id)
    );
}
