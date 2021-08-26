use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};
use std::sync::Mutex;

pub enum Insert {
    Ok(PokemonNumber),
    Conflict,
    Error,
}

pub trait Repository: Send + Sync {
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert;
}

pub struct InMemoryRepository {
    error: bool,
    pokemons: Mutex<Vec<Pokemon>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let pokemons: Mutex<Vec<Pokemon>> = Mutex::new(vec![]);
        Self {
            error: false,
            pokemons,
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

impl Repository for InMemoryRepository {
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert {
        if self.error {
            return Insert::Error;
        }

        let mut lock = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Insert::Error,
        };

        if lock.iter().any(|pokemon| pokemon.number == number) {
            return Insert::Conflict;
        }

        let number_clone = number.clone();
        lock.push(Pokemon::new(number_clone, name, types));
        Insert::Ok(number)
    }
}
