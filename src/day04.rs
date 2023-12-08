pub fn resolve() {
  let mut i: u32 = 0;

  loop {
    let entry = format!("ckczppom{}", &i);
    let digest =  md5::compute(entry.as_bytes());
    let hex_string = format!("{:x}", digest);

    if &hex_string[0..6] == "000000" {
      break;
    }
    i += 1;
  }

  println!("The lowest positive number that produces such a hash is {i}.");
}