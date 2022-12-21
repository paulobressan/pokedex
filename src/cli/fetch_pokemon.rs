use std::sync::Arc;

use crate::{domain::fetch_pokemon, repositories::pokemon::Repository};

use super::prompt_number;

#[derive(Debug)]
struct Response(u16, String, Vec<String>);

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
        Ok(res) => println!("{:?}", Response(res.number, res.name, res.types,)),
        Err(fetch_pokemon::Error::BadRequest) => println!("The request is invalid"),
        Err(fetch_pokemon::Error::NotFound) => println!("The Pokemon does not exist"),
        Err(fetch_pokemon::Error::Unknown) => println!("An unknown error occurred"),
    }
}
