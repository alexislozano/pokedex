use crate::cli::prompt_number;
use crate::domain::delete_pokemon;
use crate::repositories::pokemon::Repository;
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    let number = prompt_number();

    let req = match number {
        Ok(number) => delete_pokemon::Request { number },
        _ => {
            println!("An error occurred during the prompt");
            return;
        }
    };
    match delete_pokemon::execute(repo, req) {
        Ok(()) => println!("The Pokemon has been deleted"),
        Err(delete_pokemon::Error::BadRequest) => println!("The request is invalid"),
        Err(delete_pokemon::Error::NotFound) => println!("The Pokemon does not exist"),
        Err(delete_pokemon::Error::Unknown) => println!("An unknown error occurred"),
    }
}
