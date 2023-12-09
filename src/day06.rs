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
    let temp = line.unwrap();
    let entry: Vec<&str> = temp.split_whitespace().collect();

    let operation: Operation = match entry.get(1) {
      Some(&"on") => Operation::TurnOn,
      Some(&"off") => Operation::TurnOff,
      _ => Operation::Toggle,
    };

    let start = operation.get_coords(&entry, Coords::Start);
    let end = operation.get_coords(&entry, Coords::End);

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
  }

  for (_key, brightness) in lamps { counter += brightness };
  println!("The total brightness of all lights combined is {counter}.");
}

enum Operation {
  Toggle,
  TurnOn,
  TurnOff
}

enum Coords {
  Start,
  End,
}

impl Operation {
  fn get_coords(&self, entry: &[&str], location: Coords) -> (u32, u32) {
    let index = match (location, &self) {
      (Coords::Start, Operation::Toggle) => 1,
      (Coords::Start, _) => 2,
      (Coords::End, Operation::Toggle) => 3,
      (Coords::End, _) => 4,
    };

    match entry.get(index) {
      Some(coords) => match coords.split_once(',').map(|(x,y)| (x.parse(), y.parse())) {
        Some(value) => match value {
          (Ok(value1), Ok(value2)) => (value1, value2),
          _ => panic!("my error 2"),
        },
        None => panic!("my error"),
      },
      None => panic!("Problem while parsing."),
    }
  }
}

