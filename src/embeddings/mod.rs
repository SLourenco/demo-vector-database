pub fn calculate_embeddings(movie: &str, category: &str, runtime: usize) -> Vec<usize> {
    vec![movie.len(), get_category_code(category), runtime]
}

fn get_category_code(category: &str) -> usize {
    match category {
        "sci-fi" => 1,
        "comedy" => 2,
        "action" => 3,
        "romance" => 4,
        "horror" => 5,
        "drama" => 6,
        "fantasy" => 7,
        "thriller" => 8,
        "animation" => 9,
        "adventure" => 10,
        _ => 999,
    }
}
