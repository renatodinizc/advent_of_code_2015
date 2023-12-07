use std::fs;

pub fn resolve() {
    let entry = match fs::read_to_string("inputs/day01.txt") {
        Ok(content) => content,
        Err(_e) => panic!("A problem occurred while opening file."),
    };

    let mut floor: i32 = 0;
    let mut basement_not_found = true;
    let mut basement_entrance_step = 1;

    for (index, item) in entry.chars().enumerate() {
        match item {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Problem with input data."),
        }

        if basement_not_found && floor == -1 {
            basement_entrance_step += u32::try_from(index).unwrap();
            basement_not_found = false;
        }
    };

    println!("The final floor Santa arrived is {}. Entrance onto the basement was at step {}.",
        floor, basement_entrance_step)
}