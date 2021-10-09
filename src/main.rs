mod api;
mod cli;
mod domain;
mod repositories;

#[macro_use]
extern crate rouille;
#[macro_use]
extern crate clap;
extern crate serde;

use clap::{App, Arg};
use repositories::pokemon::InMemoryRepository;
use std::sync::Arc;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("cli").long("cli").help("Runs in CLI mode"))
        .get_matches();

    match matches.occurrences_of("cli") {
        0 => api::serve("localhost:8000", repo),
        _ => cli::run(repo),
    }
}
