use std::io::{ self, Read };

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ip = input.parse::<u32>().unwrap();
    println!("{}.{}.{}.{}", (ip >> 24) as u8, (ip >> 16) as u8, (ip >> 8) as u8, ip as u8);
}