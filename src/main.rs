use crate::embeddings::calculate_embeddings;
use crate::storage::table;
use crate::storage::table::Table;
use serde::{Deserialize, Serialize};
use std::fs;

mod embeddings;
mod storage;

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    name: String,
    category: String,
    runtime: usize,
}

fn initialize_db() -> Table {
    println!("Initializing vector database...\n");
    let mut db = storage::init_storage().expect("Failed to initialize storage");
    let mut movies = db
        .init_table("movies")
        .expect("error creating movies table");

    if movies.size == 0 {
        let movies_json = parse_movies_file();
        for (idx, movie) in movies_json.into_iter().enumerate() {
            movies
                .add_data(table::Record {
                    id: idx,
                    name: movie.name.clone(),
                    runtime_minutes: movie.runtime,
                    vector: calculate_embeddings(
                        movie.name.as_str(),
                        movie.category.as_str(),
                        movie.runtime,
                    ),
                })
                .expect(format!("error adding {} to movies table", movie.name).as_str());
        }
    }
    movies
}

fn main() {
    let movies = initialize_db();
    println!("Searching for similar movies...\n");
    let similar_movies = movies
        .search(
            calculate_embeddings("Lord of the Rings - Fellowship of the Ring", "fantasy", 178),
            2,
            "cosine",
        )
        .expect("error searching for similar movies");
    for similar_movie in similar_movies {
        println!("Recommendation:");
        println!(
            "Title: {}\nRuntime: {} (minutes) \n",
            similar_movie.1.name, similar_movie.1.runtime_minutes
        );
    }
}

fn parse_movies_file() -> Vec<Movie> {
    let file = fs::File::open("movies.json").expect("file should open read only");
    let movies: Vec<Movie> = serde_json::from_reader(file).expect("file should be proper JSON");
    movies
}
