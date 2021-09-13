mod create_pokemon;
mod delete_pokemon;
mod fetch_all_pokemons;
mod fetch_pokemon;
mod health;

use crate::repositories::pokemon::Repository;
use std::sync::Arc;

pub fn serve(url: &str, repo: Arc<dyn Repository>) {
    rouille::start_server(url, move |req| {
        router!(req,
            (GET) (/) => {
                fetch_all_pokemons::serve(repo.clone())
            },
            (GET) (/{number: u16}) => {
                fetch_pokemon::serve(repo.clone(), number)
            },
            (GET) (/health) => {
                health::serve()
            },
            (POST) (/) => {
                create_pokemon::serve(repo.clone(), req)
            },
            (DELETE) (/{number: u16}) => {
                delete_pokemon::serve(repo.clone(), number)
            },
            _ => {
                rouille::Response::from(Status::NotFound)
            }
        )
    });
}

enum Status {
    Ok,
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
}

impl From<Status> for rouille::Response {
    fn from(status: Status) -> Self {
        let status_code = match status {
            Status::Ok => 200,
            Status::BadRequest => 400,
            Status::NotFound => 404,
            Status::Conflict => 409,
            Status::InternalServerError => 500,
        };
        Self {
            status_code,
            headers: vec![],
            data: rouille::ResponseBody::empty(),
            upgrade: None,
        }
    }
}
