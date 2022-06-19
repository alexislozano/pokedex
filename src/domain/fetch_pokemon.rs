use crate::domain::entities::{Pokemon, PokemonNumber};
use crate::repositories::pokemon::{FetchOneError, Repository};
use std::sync::Arc;

pub struct Request {
    pub number: u16,
}

pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub enum Error {
    BadRequest,
    NotFound,
    Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
    match PokemonNumber::try_from(req.number) {
        Ok(number) => match repo.fetch_one(number) {
            Ok(Pokemon {
                number,
                name,
                types,
            }) => Ok(Response {
                number: u16::from(number),
                name: String::from(name),
                types: Vec::<String>::from(types),
            }),
            Err(FetchOneError::NotFound) => Err(Error::NotFound),
            Err(FetchOneError::Unknown) => Err(Error::Unknown),
        },
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{PokemonName, PokemonTypes};
    use crate::repositories::pokemon::InMemoryRepository;

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::bad());

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_a_not_found_error_when_the_repo_does_not_contain_the_pokemon() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_the_pokemon_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        )
        .ok();
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Ok(res) => {
                assert_eq!(res.number, u16::from(PokemonNumber::pikachu()));
                assert_eq!(res.name, String::from(PokemonName::pikachu()));
                assert_eq!(res.types, Vec::<String>::from(PokemonTypes::pikachu()));
            }
            _ => unreachable!(),
        };
    }

    impl Request {
        fn new(number: PokemonNumber) -> Self {
            Self {
                number: u16::from(number),
            }
        }
    }
}
