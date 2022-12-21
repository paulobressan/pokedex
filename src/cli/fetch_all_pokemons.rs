use std::sync::Arc;

use crate::{domain::fetch_all_pokemons, repositories::pokemon::Repository};

#[derive(Debug)]
struct Response(u16, String, Vec<String>);

pub fn run(repo: Arc<dyn Repository>) {
    match fetch_all_pokemons::execute(repo) {
        Ok(res) => res.into_iter().for_each(|p| {
            println!("{:?}", Response(p.number, p.name, p.types,));
        }),
        Err(fetch_all_pokemons::Error::Unknown) => println!("An unknown error occurred"),
    };
}
