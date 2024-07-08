use std::io::{self, Write};

pub fn parse(line: String) {
  println!("{line}")
  // if line.starts_with("        \"Progress\":") {
  //   let pieces: Vec<_> = line.split(":").collect();
  //   let progress_str = pieces.get(1).take().expect(&format!("Expected 2 tokens but got: {:?}", pieces));
  //   // Remove the trailing ",""
  //   let progress = &progress_str[..progress_str.len() - 1].trim().parse::<f32>().unwrap() * 100.0;
  //   print!("{progress}\r");
  //   io::stdout().flush().unwrap();
  // } else if line.starts_with("  :      \"Error\":") {
  //   let pieces: Vec<_> = line.split(":").collect();
  //   let progress_str = pieces.get(1).take().expect(&format!("Expected 2 tokens but got: {:?}", pieces));
  //   // Remove the trailing ",""
  //   let errors = &progress_str[..progress_str.len() - 1].trim().parse::<f32>().unwrap();
  //   println!("errors: {errors}");
  // } else {
  //   // println!("other: {line}")
  // }
}
