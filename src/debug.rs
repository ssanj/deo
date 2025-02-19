use std::collections::HashMap;

use console::style;

use crate::entry_type::EncodeDir;
use crate::entry_type::EntryType;
use crate::entry_type::MKVTypeAware;
use crate::entry_type::Session;
use crate::entry_type::SessionId;
use crate::entry_type::SessionToEncodeDir;
use crate::colours::*;
use crate::entry_type::SessionTypeAware;

// See: https://askubuntu.com/questions/821157/print-a-256-color-test-pattern-in-the-terminal
// TODO: Fix up episodic printing
pub fn dump_entry_types(entry_types: &[EntryType], verbose: bool) {
  if verbose {
    println!("{}", style("-- Entry Names --").bg(LIGHT).fg(BLACK));
    for et in entry_types {
      match et {
        EntryType::TVSeriesRename { path, session, episode, file } => {
          let msg = style(format!("EntryType.Rename:\n  session:{}\n  path:{}\n  episode:{}\n  file:{}", session.id(), path.to_string_lossy(), episode, file)).bg(GRAY);
          println!("{}", msg);
          println!()
        },
        EntryType::TVSeriesEncode { session, path, season } => {
          let msg = style(format!("EntryType.Encode:\n  session:{}\n  path:{}\n  season:{}", session.id(), path.to_string_lossy(), season)).bg(GRAY);
          println!("{}", msg);
          println!()
        },
        _ =>  ()
      }
    }
  }
}

pub fn dump_unmatched_entry_types(entry_types: &[EntryType], verbose: bool) {
  if verbose {
    println!("{}", style("-- Unmatched Entry Names --").bg(LIGHT_RED).fg(BLACK));
    for et in entry_types {
      match et {
        EntryType::UnknownFileType { path } => {
          let msg = format!("{}\n  path:{}", style("EntryType.Unknown:").bg(LIGHT_RED_2), path.to_string_lossy());
          println!("{}", msg);
          println!()
        },
        EntryType::InvalidEncodeDirPath { defined_path } => {
          let msg = format!("{}\n  path:{}", style("EntryType.InvalidEncodeDirPath:").bg(RED_2), &defined_path);
          println!("{}", msg);
          println!()
        },
        _ => ()
      }
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
        let episode = match &file {
            crate::entry_type::RenameTypes::TVSeries(tvseries_rename_file) => Some(&tvseries_rename_file.episode),
            crate::entry_type::RenameTypes::Movie(_) => None,
        };
        let pathbuf = file.mkv_path();
        let path = pathbuf.to_string_lossy();
        let episode_str = match episode {
            Some(e) => format!("  episode:{e}\n"),
            None => "".to_string(),
        };
        let mkv_file = file.mkv_file();
        let mp4_file = file.mp4_file();
        let session_id = file.session_id();
        let msg = style(format!("\n  session:{session_id}\n  path:{path}\n{episode_str}  mkv_file:{mkv_file}\n  mp4_file:{mp4_file}")).bg(GRAY);
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
        let pathbuf = file.mkv_path();
        let path = pathbuf.to_string_lossy();
        let episode = match &file {
            crate::entry_type::RenameTypes::TVSeries(tvseries_rename_file) => Some(&tvseries_rename_file.episode),
            crate::entry_type::RenameTypes::Movie(_) => None,
        };
        let episode_str = match episode {
            Some(e) => format!("    episode:{e}\n"),
            None => "".to_string(),
        };

        let mkv_file = file.mkv_file();
        let mp4_file = file.mp4_file();
        let session_session_id = file.session_id();

        let session_msg = style(format!("\n  Session:\n    session:{session_session_id}\n    path:{path}\n{episode_str}    mkv_file:{mkv_file}\n    mp4_file:{mp4_file}")).bg(GRAY);
        println!("{}", session_msg);
        println!();
      }
    }
  }
}

pub fn dump_unmapped_sessions_and_encode_dirs(
    sessions_to_encode_dir: &[SessionToEncodeDir],
    sessions_hash: &HashMap<SessionId, Session>,
    encode_dir_hash: &HashMap<SessionId, EncodeDir>,
    verbose: bool) {

  if verbose {
    let mapped_session_ids: Vec<SessionId> =
      sessions_to_encode_dir
        .iter()
        .map(|sed| sed.session_id().clone())
        .collect();

      let mut has_unmapped_sessions = false;
      for (session_id, session) in sessions_hash {
        if !mapped_session_ids.contains(session_id) {
          if !has_unmapped_sessions {
            has_unmapped_sessions = true;
            let msg = style("-- Unmapped sessions --").bg(ORANGE);
            println!("{}", msg);
          }

          for file in session.files() {
            let pathbuf = file.mkv_path();
            let path = pathbuf.to_string_lossy();
            let episode = match &file {
                crate::entry_type::RenameTypes::TVSeries(tvseries_rename_file) => Some(&tvseries_rename_file.episode),
                crate::entry_type::RenameTypes::Movie(_) => None,
            };
            let episode_str = match episode {
                Some(e) => format!("    episode:{e}\n"),
                None => "".to_string(),
            };
            let mkv_file = file.mkv_file();
            let mp4_file = file.mp4_file();
            let session_session_id = file.session_id();

            let session_msg = style(format!("\n  Session:\n    session:{session_session_id}\n    path:{path}\n{episode_str}    mkv_file:{mkv_file}\n    mp4_file:{mp4_file}")).bg(GRAY);
            println!("{}", session_msg);
            println!();
          }
        }
      }

      let mut has_unmapped_encodes = false;
      for (session_id, encodes) in encode_dir_hash {
        if !mapped_session_ids.contains(session_id) {
          if !has_unmapped_encodes {
            has_unmapped_encodes = true;
            let msg = style("-- Unmapped encodes --").bg(ORANGE);
            println!("{}", msg);
          }

          let encodes_session_id = encodes.session_id.id();
          let encodes_path = encodes.path.to_string_lossy();
          let encodes_season = &encodes.season;
          let encodes_msg = style(format!("\n  Encodes:\n    session:{encodes_session_id}\n    {encodes_path}\n    {encodes_season}")).bg(GRAY);
          println!("{}", encodes_msg);
        }
      }
  }
}
