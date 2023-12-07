use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;


pub fn resolve() -> Result<(), Box<dyn Error>> {
    let file = match File::open("inputs/day02.txt") {
        Ok(file) => file,
        Err(e) => panic!("A problem occurred while opening file: {e}."),
    };
    let entry = BufReader::new(file);
    let mut total_wrapping_paper = 0;
    let mut total_ribbon = 0;

    for line in entry.lines() {
        let (l, w, h): (u32, u32, u32) = parse_line(line?);
        total_wrapping_paper += calculate_wrapping_paper(&l, &w, &h);
        total_ribbon += calculate_ribbon(&l, &w, &h);
    }

    println!("Total amount of square feet of wrapping paper to order is {total_wrapping_paper}, and the total feet of ribbon is {total_ribbon}.");

    Ok(())
}

fn parse_line(content: String) -> (u32, u32, u32) {
    let v: Vec<&str> = content.split('x').collect();
    (
        v.first().unwrap().parse().unwrap(),
        v.get(1).unwrap().parse().unwrap(),
        v.get(2).unwrap().parse().unwrap()
    )
}

fn calculate_wrapping_paper(length: &u32, width: &u32, height: &u32) -> u32 {
    let areas = [length * width,
                 width * height,
                 height * length];

    let min_area = *areas.iter().min().unwrap();

    2 * length * width + 2 * width * height + 2 * height * length + min_area
}

fn calculate_ribbon(length: &u32, width: &u32, height: &u32) -> u32 {
    let bow = length * width * height;
    let mut sizes = [length, width, height];

    sizes.sort();
    let smallest_perimeter = 2 * sizes[0] + 2 * sizes[1];

    smallest_perimeter + bow
}