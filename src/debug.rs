use std::collections::HashMap;

use console::style;
use console::Color;

use crate::entry_type::EntryType;
use crate::entry_type::Session;
use crate::entry_type::SessionId;

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

pub fn dump_sessions_hash(session_hash: &HashMap<SessionId, Session>, verbose: bool) {

  if verbose {
    println!("{}", style("-- Sessions Hash --").bg(LIGHT_GRAY));
    for (si, sess) in session_hash {
      let msg = style(format!("SessionId:{}", si.id())).bg(GRAY);
      println!("{}", msg);
      for file in sess.files() {
        let path = file.path.to_string_lossy();
        let episode = file.episode;
        let mkv_file = file.mkv_file;
        let mp4_file = file.mp4_file;
        let session_id = file.session.id();
        let msg = style(format!("\n  session:{session_id}\n  path:{path}\n  episode:{episode}\n  mkv_file:{mkv_file}\n  mp4_file:{mp4_file}")).bg(GRAY);
        println!("{}", msg);
        println!();
      }
    }
  }
}
