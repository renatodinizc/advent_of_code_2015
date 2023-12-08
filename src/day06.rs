use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn resolve() {
  let file = match File::open("inputs/day06.txt") {
    Ok(file) => file,
    Err(e) => panic!("A problem occurred while opening file: {e}."),
  };

  let entry = BufReader::new(file);

  let mut lamps: HashMap<(u32, u32), u32> = HashMap::new();
  let mut counter = 0;

  for line in entry.lines() {
    let entry = line.unwrap();
    let entry_v: Vec<&str> = entry.split(' ').collect();

    let is_toggle = entry_v.first().unwrap() == &"toggle";

    let operation: Operation = if is_toggle {
      Operation::Toggle
    } else {
      match *entry_v.get(1).unwrap() {
        "on" => Operation::TurnOn,
        "off" => Operation::TurnOff,
        _ => panic!("problem while reading file"),
      }
    };

    let start: Option<(u32, u32)> = if is_toggle {
      entry_v.get(1).unwrap().split_once(',').map(|(x,y)| (x.parse().unwrap(), y.parse().unwrap()))
    } else {
        entry_v.get(2).unwrap().split_once(',').map(|(x,y)| (x.parse().unwrap(), y.parse().unwrap()))
    };

    let end: Option<(u32, u32)> = if is_toggle {
      entry_v.get(3).unwrap().split_once(',').map(|(x,y)| (x.parse().unwrap(), y.parse().unwrap()))
    } else {
      entry_v.get(4).unwrap().split_once(',').map(|(x,y)| (x.parse().unwrap(), y.parse().unwrap()))
    };

    match (start, end) {
      (Some(start), Some(end)) => {

        for x in start.0..=end.0 {
          for y in start.1..=end.1 {
            match &operation {
              Operation::TurnOn => {
                let brightness = lamps.entry((x, y)).or_insert(0);
                *brightness += 1;
              },
              Operation::TurnOff => {
                let brightness = lamps.entry((x, y)).or_insert(1);
                if *brightness > 0 { *brightness -= 1 };
              },
              Operation::Toggle => {
                let brightness = lamps.entry((x, y)).or_insert(0);
                *brightness += 2;
              },
            };
          }
        }
      },
      _ => panic!("Problem here"),
    };
  }



  for (_key, brightness) in lamps {
    counter += brightness;
  };
  println!("The total brightness of all lights combined is {counter}.");
}

enum Operation {
  Toggle,
  TurnOn,
  TurnOff
}

