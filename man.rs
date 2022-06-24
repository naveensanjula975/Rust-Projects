#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    
    {
        let area = area_of(width, height);
    }
    println!("Area is {}", area);
