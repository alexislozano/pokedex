use crate::cli::prompt_number;
use crate::domain::fetch_pokemon;
use crate::repositories::pokemon::Repository;
use std::sync::Arc;

#[derive(Debug)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn run(repo: Arc<dyn Repository>) {
    let number = prompt_number();

    let req = match number {
        Ok(number) => fetch_pokemon::Request { number },
        _ => {
            println!("An error occurred during the prompt");
            return;
        }
    };
    match fetch_pokemon::execute(repo, req) {
        Ok(res) => println!(
            "{:?}",
            Response {
                number: res.number,
                name: res.name,
                types: res.types,
            }
        ),
        Err(fetch_pokemon::Error::BadRequest) => println!("The request is invalid"),
        Err(fetch_pokemon::Error::NotFound) => println!("The Pokemon does not exist"),
        Err(fetch_pokemon::Error::Unknown) => println!("An unknown error occurred"),
    }
}
