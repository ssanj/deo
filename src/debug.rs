use std::collections::HashMap;

use console::style;
use crate::models::EntryType;
use crate::models::MovieEncodeDir;
use crate::models::MovieSession;
use crate::models::MovieToEncodeDir;
use crate::models::SessionId;
use crate::models::SessionToEncodeDir;
use crate::colours::*;
use crate::models::TVSeriesEncodeDir;
use crate::models::TVSeriesSession;
use crate::models::TVSeriesToEncodeDir;

// See: https://askubuntu.com/questions/821157/print-a-256-color-test-pattern-in-the-terminal
pub fn dump_entry_types(entry_types: &[EntryType], verbose: bool) {
  if verbose {
    println!("{}", style("-- Entry Names --").bg(LIGHT).fg(BLACK));
    for et in entry_types {
      match et {
        EntryType::TVSeriesRename { path, session, episode, file } => {
          let msg = style(format!("EntryType.TV.Rename:\n  session:{}\n  path:{}\n  episode:{}\n  file:{}", session.id(), path.to_string_lossy(), episode, file)).bg(GRAY);
          println!("{}", msg);
          println!()
        },

        EntryType::TVSeriesEncode { session, path, season } => {
          let msg = style(format!("EntryType.TV.Encode:\n  session:{}\n  path:{}\n  season:{}", session.id(), path.to_string_lossy(), season)).bg(GRAY);
          println!("{}", msg);
          println!()
        },

        EntryType::MovieRename { path, session, file } => {
          let msg = style(format!("EntryType.Movie.Rename:\n  session:{}\n  path:{}\n  file:{}", session.id(), path.to_string_lossy(), file)).bg(GRAY);
          println!("{}", msg);
          println!()
        },

        EntryType::MovieEncode { session, path, movie_name } => {
          let msg = style(format!("EntryType.Movie.Encode:\n  session:{}\n  path:{}\n  movie_name:{}", session.id(), path.to_string_lossy(), movie_name)).bg(GRAY);
          println!("{}", msg);
          println!()
        },

        _ => ()
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

pub fn dump_tv_sessions_hash(session_hash: &HashMap<SessionId, TVSeriesSession>, verbose: bool) {
  if verbose {
    println!("{}", style("-- TVSeries Sessions Hash --").bg(BLUE));
    for (si, sess) in session_hash {
      let msg = style(format!("SessionId:{}", si.id())).bg(GRAY);
      println!("{}", msg);
      for file in sess.files() {
        let episode = file.episode;
        let pathbuf = file.path;
        let path = pathbuf.to_string_lossy();
        let mkv_file = file.mkv_file;
        let mp4_file = file.mp4_file;
        let session_id = file.session;
        let msg = style(format!("\n  tvseries session:{session_id}\n  path:{path}\n  episode:{episode}\n  mkv_file:{mkv_file}\n  mp4_file:{mp4_file}")).bg(GRAY);
        println!("{}", msg);
        println!();
      }
    }
  }
}

pub fn dump_movie_sessions_hash(session_hash: &HashMap<SessionId, MovieSession>, verbose: bool) {
  if verbose {
    println!("{}", style("-- TVSeries Sessions Hash --").bg(BLUE));
    for (si, sess) in session_hash {
      let msg = style(format!("SessionId:{}", si.id())).bg(GRAY);
      println!("{}", msg);
      for file in sess.files() {
        let pathbuf = file.path;
        let path = pathbuf.to_string_lossy();
        let mkv_file = file.mkv_file;
        let mp4_file = file.mp4_file;
        let session_id = file.session;
        let msg = style(format!("\n  movie session:{session_id}\n  path:{path}\n  mkv_file:{mkv_file}\n  mp4_file:{mp4_file}")).bg(GRAY);
        println!("{}", msg);
        println!();
      }
    }
  }
}

pub fn dump_tv_series_encodes_hash(encode_dir_hash: &HashMap<SessionId, TVSeriesEncodeDir>, verbose: bool) {
  if verbose {
    println!("{}", style("-- TV Series Encodes Hash --").bg(LIGHT_BLUE));
    for (si, ed) in encode_dir_hash {
      let session_id = &ed.session_id;
      let encode_path = ed.path.clone();
      let path = encode_path.to_string_lossy();
      let location = &ed.season;
      let msg = style(format!("SessionId:{}\n  session:{session_id}\n  path:{path}\n  location:{location}", si.id())).bg(GRAY);
      println!("{}", msg);
      println!();
    }
  }
}

pub fn dump_movie_encodes_hash(encode_dir_hash: &HashMap<SessionId, MovieEncodeDir>, verbose: bool) {
  if verbose {
    println!("{}", style("-- Movie Encodes Hash --").bg(LIGHT_BLUE));
    for (si, ed) in encode_dir_hash {
      let session_id = &ed.session_id;
      let encode_path = ed.path.clone();
      let path = encode_path.to_string_lossy();
      let location = &ed.movie_name;
      let msg = style(format!("SessionId:{}\n  session:{session_id}\n  path:{path}\n  location:{location}", si.id())).bg(GRAY);
      println!("{}", msg);
      println!();
    }
  }
}

pub fn dump_sessions_to_encode_dirs(session_to_encode_dirs: &[SessionToEncodeDir], verbose: bool) {
  if verbose {
    for sted in session_to_encode_dirs {
      match sted {
        SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) => dump_tv_series_to_encode_dirs(tvseries_to_encode_dir),
        SessionToEncodeDir::MovieMapping(movie_to_encode_dir) => dump_movie_to_encode_dirs(movie_to_encode_dir),
      }
    }
  }
}

fn dump_tv_series_to_encode_dirs(sted: &TVSeriesToEncodeDir) {
  println!("{}", style("-- SessionToEncodeDir Mapping --").bg(GREEN));
  let session_id = sted.session_id();
  let encodes_dir = &sted.encode_dir().path;
  let encodes_path = encodes_dir.to_string_lossy();

  let location = &sted.encode_dir().season;
  let msg = style(format!("SessionId:{session_id}")).bg(GRAY);
  println!("{}", msg);

  let encodes_msg = style(format!("\n  TV Series Encodes:\n    session:{session_id}\n    path:{encodes_path}\n    location:{location}")).bg(GRAY);
  println!("{}", encodes_msg);

  for file in sted.session().files() {
    let pathbuf = file.path;
    let path = pathbuf.to_string_lossy();
    let episode = file.episode;
    let mkv_file = file.mkv_file;
    let mp4_file = file.mp4_file;
    let session_session_id = sted.session_id();

    let session_msg = style(format!("\n  TV Series Session:\n    session:{session_session_id}\n    path:{path}\n{episode}    mkv_file:{mkv_file}\n    mp4_file:{mp4_file}")).bg(GRAY);
    println!("{}", session_msg);
    println!();
  }
}

fn dump_movie_to_encode_dirs(sted: &MovieToEncodeDir) {
  println!("{}", style("-- SessionToEncodeDir Mapping --").bg(GREEN));
  let session_id = sted.session_id();
  let encodes_dir = &sted.encode_dir().path;
  let encodes_path = encodes_dir.to_string_lossy();

  let msg = style(format!("SessionId:{session_id}")).bg(GRAY);
  println!("{}", msg);

  let encodes_msg = style(format!("\n  Movie Encodes:\n    session:{session_id}\n    path:{encodes_path}")).bg(GRAY);
  println!("{}", encodes_msg);

  for file in sted.session().files() {
    let pathbuf = file.path;
    let path = pathbuf.to_string_lossy();
    let mkv_file = file.mkv_file;
    let mp4_file = file.mp4_file;
    let session_session_id = sted.session_id();

    let session_msg = style(format!("\n  Movie Session:\n    session:{session_session_id}\n    path:{path}\n    mkv_file:{mkv_file}\n    mp4_file:{mp4_file}")).bg(GRAY);
    println!("{}", session_msg);
    println!();
  }
}

pub fn dump_unmapped_tv_series_sessions_and_encode_dirs(tv_series_session_to_encode_dir: &[SessionToEncodeDir], tv_series_session: &HashMap<SessionId, TVSeriesSession>, tv_series_encode_dir: &HashMap<SessionId, TVSeriesEncodeDir>, verbose: bool) {
    if verbose {
      let mapped_session_ids: Vec<SessionId> =
        tv_series_session_to_encode_dir
          .into_iter()
          .map(|sed| sed.session_id().clone())
          .collect();

      let mut has_unmapped_sessions = false;
      for (session_id, session) in tv_series_session {
        if !mapped_session_ids.contains(session_id) {
          if !has_unmapped_sessions {
            has_unmapped_sessions = true;
            let msg = style("-- Unmapped TV Series Sessions --").bg(ORANGE);
            println!("{}", msg);
          }

          for file in session.files() {
            let pathbuf = file.path;
            let path = pathbuf.to_string_lossy();
            let episode = &file.episode;
            let mkv_file = file.mkv_file;
            let mp4_file = file.mp4_file;
            let session_session_id = file.session;

            let session_msg = style(format!("\n  Session:\n    session:{session_session_id}\n    path:{path}\n    episode:{episode}\n    mkv_file:{mkv_file}\n    mp4_file:{mp4_file}")).bg(GRAY);
            println!("{}", session_msg);
            println!();
          }
        }
      }

      let mut has_unmapped_encodes = false;
      for (session_id, encodes) in tv_series_encode_dir {
        if !mapped_session_ids.contains(session_id) {
          if !has_unmapped_encodes {
            has_unmapped_encodes = true;
            let msg = style("-- Unmapped TV Series Encodes --").bg(ORANGE);
            println!("{}", msg);
          }

          let encodes_session_id = &encodes.session_id;
          let path = &encodes.path;
          let encodes_path = path.to_string_lossy();
          let season = &encodes.season;
          let encodes_msg = style(format!("\n  Encodes:\n    session:{encodes_session_id}\n    {encodes_path}\n    season:{season}")).bg(GRAY);
          println!("{}", encodes_msg);
        }
      }
    }
}

pub(crate) fn dump_unmapped_movie_sessions_and_encode_dirs(movie_session_to_encode_dir: &[SessionToEncodeDir], movies_session: &HashMap<SessionId, MovieSession>, movie_encode_dir: &HashMap<SessionId, MovieEncodeDir>, verbose: bool) {
    if verbose {
      let mapped_session_ids: Vec<SessionId> =
        movie_session_to_encode_dir
          .into_iter()
          .map(|sed| sed.session_id().clone())
          .collect();

      let mut has_unmapped_sessions = false;
      for (session_id, session) in movies_session {
        if !mapped_session_ids.contains(session_id) {
          if !has_unmapped_sessions {
            has_unmapped_sessions = true;
            let msg = style("-- Unmapped Movie Sessions --").bg(ORANGE);
            println!("{}", msg);
          }

          for file in session.files() {
            let pathbuf = file.path;
            let path = pathbuf.to_string_lossy();
            let mkv_file = file.mkv_file;
            let mp4_file = file.mp4_file;
            let session_session_id = file.session;

            let session_msg = style(format!("\n  Session:\n    session:{session_session_id}\n    path:{path}\n    mkv_file:{mkv_file}\n    mp4_file:{mp4_file}")).bg(GRAY);
            println!("{}", session_msg);
            println!();
          }
        }
      }

      let mut has_unmapped_encodes = false;
      for (session_id, encodes) in movie_encode_dir {
        if !mapped_session_ids.contains(session_id) {
          if !has_unmapped_encodes {
            has_unmapped_encodes = true;
            let msg = style("-- Unmapped Movie Encodes --").bg(ORANGE);
            println!("{}", msg);
          }

          let encodes_session_id = &encodes.session_id;
          let path = &encodes.path;
          let encodes_path = path.to_string_lossy();
          let movie_name = &encodes.movie_name;
          let encodes_msg = style(format!("\n  Encodes:\n    session:{encodes_session_id}\n    {encodes_path}\n     movie_name:{movie_name}")).bg(GRAY);
          println!("{}", encodes_msg);
        }
      }
    }
}
