use std::collections::HashMap;

use console::style;
use console::Color;

use crate::entry_type::EncodeDir;
use crate::entry_type::EntryType;
use crate::entry_type::Session;
use crate::entry_type::SessionId;
use crate::entry_type::SessionToEncodeDir;

// See: https://askubuntu.com/questions/821157/print-a-256-color-test-pattern-in-the-terminal
const BLUE: Color = Color::Color256(21);
const LIGHT: Color = Color::Color256(15);
const LIGHT_BLUE: Color = Color::Color256(33);
const GREEN: Color = Color::Color256(35);
const GRAY: Color = Color::Color256(236);

pub fn dump_entry_types(entry_types: &[EntryType], verbose: bool) {
  if verbose {
    println!("{}", style("-- Entry Names --").bg(LIGHT).fg(Color::Black));
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
    println!("{}", style("-- Sessions Hash --").bg(BLUE));
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

pub fn dump_encodes_hash(encode_dir_hash: &HashMap<SessionId, EncodeDir>, verbose: bool) {
  if verbose {
    println!("{}", style("-- Encodes Hash --").bg(LIGHT_BLUE));
    for (si, ed) in encode_dir_hash {
      let session_id = ed.session_id.id();
      let path = ed.path.to_string_lossy();
      let season = &ed.season;
      let msg = style(format!("SessionId:{}\n  session:{session_id}\n  {path}\n  {season}", si.id())).bg(GRAY);
      println!("{}", msg);
      println!();
    }
  }
}

pub fn dump_sessions_to_encode_dirs(session_to_encode_dirs: &[SessionToEncodeDir], verbose: bool) {
  if verbose {
    println!("{}", style("-- SessionToEncodeDir Mapping --").bg(GREEN));
    for sted in session_to_encode_dirs {
      let session_id = sted.session_id().id();
      let encodes_dir = sted.encode_dir();
      let encodes_path = encodes_dir.path.to_string_lossy();
      let encodes_session_id = encodes_dir.session_id.id();
      let encodes_season = &encodes_dir.season;

      let msg = style(format!("SessionId:{session_id}")).bg(GRAY);
      println!("{}", msg);

      let encodes_msg = style(format!("\n  Encodes:\n    session:{encodes_session_id}\n    {encodes_path}\n    {encodes_season}")).bg(GRAY);
      println!("{}", encodes_msg);

      for file in sted.session().files() {
        let path = file.path.to_string_lossy();
        let episode = file.episode;
        let mkv_file = file.mkv_file;
        let mp4_file = file.mp4_file;
        let session_session_id = file.session.id();

        let session_msg = style(format!("\n  Session:\n    session:{session_session_id}\n    path:{path}\n    episode:{episode}\n    mkv_file:{mkv_file}\n    mp4_file:{mp4_file}")).bg(GRAY);
        println!("{}", session_msg);
        println!();
      }


    }
  }
}
