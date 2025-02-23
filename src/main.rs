use args::cli;

mod args;
mod entry_type;
mod user_selection;
mod hb_output_parser;
mod file_mapper;
mod user_choices;
mod handbrake;
mod profiles;
mod error;
mod debug;
mod colours;
mod workflow;
mod models;

fn main() {
  workflow::perform(cli::get_cli_args())
}
