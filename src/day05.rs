use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::RegexSet;
use fancy_regex::Regex as Regex;

pub fn resolve() {
  let file = match File::open("inputs/day05.txt") {
    Ok(file) => file,
    Err(e) => panic!("A problem occurred while opening file: {e}."),
  };

  let entry = BufReader::new(file);
  let mut counter: u32 = 0;

  for line in entry.lines() {
    let entry = line.unwrap();

    // Set of rules for Part One
    let contains_at_least_three_vowels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let contain_certain_substrings = RegexSet::new(&[r"ab", r"cd", r"pq", r"xy",]).unwrap();
    let has_consecutive_letters = Regex::new(r"([a-zA-Z])\1").unwrap();

    if !contains_at_least_three_vowels.is_match(&entry).unwrap() { continue };
    if contain_certain_substrings.is_match(&entry) { continue };
    if !has_consecutive_letters.is_match(&entry).unwrap() { continue };

    // Set of rules for Part Two
    let letter_repeats_with_one_letter_in_between = Regex::new(r"([a-zA-Z])\w\1").unwrap();
    let contains_pair_of_two_letters_at_least_twice = Regex::new(r"([a-zA-Z]{2}).*\1").unwrap();

    if !letter_repeats_with_one_letter_in_between.is_match(&entry).unwrap() { continue };
    if !contains_pair_of_two_letters_at_least_twice.is_match(&entry).unwrap() { continue };

    counter += 1;
  }

  println!("There are {counter} nice strings.");
}