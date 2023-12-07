use std::fs;
use std::collections::HashMap;

pub fn resolve() {
  let directions = match fs::read_to_string("inputs/day03.txt") {
    Ok(content) => content,
    Err(_e) => panic!("A problem occurred while opening file."),
  };

  let mut santa: (i32, i32) = (0,0);
  let mut robot: (i32, i32) = (0,0);
  let mut houses: HashMap<(i32, i32), u32> = HashMap::new();

  for (turn, direction) in directions.chars().enumerate() {
    if turn % 2 == 0 {
      walk(&mut santa, direction, &mut houses);
    } else {
      walk(&mut robot, direction, &mut houses);
    }
  }

  println!("Santa and the robot visited {} houses that received at least one present.",
            houses.keys().count());
}

fn walk(person: &mut (i32, i32), direction: char, houses:  &mut HashMap<(i32, i32), u32>) {

  match direction {
    '>' => person.0 += 1,
    '<' => person.0 -= 1,
    '^' => person.1 += 1,
    'v' => person.1 -= 1,
    _ => panic!("Something got wrong while reading input."),
  };

  let visits = houses.entry((person.0, person.1)).or_insert(0);
  *visits += 1;
}