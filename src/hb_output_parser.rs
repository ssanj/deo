pub enum Output {
  Progress(f32),
  Pass(u32),
  Ignore,
  Done(u32),
}

pub fn parse(line: &str) -> Output {
  if line.starts_with("        \"Progress\":") {
    let pieces: Vec<_> = line.split(':').collect();
    let progress_str = pieces.get(1).map(|v| v.to_string()).take().unwrap_or_else(|| format!("Expected 2 tokens but got: {:?}", pieces));
    // Remove the trailing ",""
    let progress = &progress_str[..progress_str.len() - 1].trim().parse::<f32>().unwrap() * 100.0;
    Output::Progress(progress)
  } else if line.starts_with("        \"Pass\":") {
    let pieces: Vec<_> = line.split(':').collect();
    let pass_str = pieces.get(1).map(|v| v.to_string()).take().unwrap_or_else(|| format!("Expected 2 tokens but got: {:?}", pieces));
    // Remove the trailing ",""
    let pass = &pass_str[..pass_str.len() - 1].trim().parse::<u32>().unwrap();
    Output::Pass(*pass)
  } else if line.starts_with("  :      \"Error\":") {
    let pieces: Vec<_> = line.split(':').collect();
    let progress_str = pieces.get(1).map(|v| v.to_string()).take().unwrap_or_else(|| format!("Expected 2 tokens but got: {:?}", pieces));
    // Remove the trailing ",""
    let errors = &progress_str[..progress_str.len() - 1].trim().parse::<u32>().unwrap();
    Output::Done(*errors)
  } else {
    Output::Ignore
  }
}
