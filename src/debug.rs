use console::style;
use console::Color;

use crate::entry_type::EntryType;

const LIGHT_GRAY: Color = Color::Color256(240);
const GRAY: Color = Color::Color256(236);

pub fn dump_entry_types(entry_types: &[EntryType], verbose: bool) {
  if verbose {
    println!("{}", style("-- Entry Names --").bg(LIGHT_GRAY));
    for et in entry_types {
      match et {
        EntryType::Rename { path, session, episode, file } => {
          let msg = style(format!("EntryType.Rename:\n  session:{}\n  path:{}\n  episode:{}\n  file:{}", session.id(), path.to_string_lossy(), episode, file)).bg(GRAY);
          println!("{}", msg)
        },
        EntryType::Encode { session, path, season } => {
          let msg = style(format!("EntryType.Encode:\n  session:{}\n  path:{}\n  season:{}", session.id(), path.to_string_lossy(), season)).bg(GRAY);
          println!("{}", msg)
        },
      }
      println!()
    }
  }
}
