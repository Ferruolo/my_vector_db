use std::error::Error;
use crate::database_core::VectorDBCore;

mod database_core;
mod ml_interface;

const TOKENIZER_PATH: &str = "Meta-Llama-3.1-8B/tokenizer.model";
const EMBEDDING_PATH: &str = "embedding.pth";

fn main() -> Result<(), Box<dyn Error>> {
    let mut vector_db = VectorDBCore::new(TOKENIZER_PATH, EMBEDDING_PATH)?;

    // Add sentences to the database
    let sentences = [
        "The sun set behind the mountains, painting the sky in vibrant hues.",
        "She carefully planted the seeds in her garden, hoping for a bountiful harvest.",
        "The old car sputtered to a stop on the side of the deserted highway.",
        "Children laughed and played in the park on a warm summer afternoon.",
        "The chef prepared a gourmet meal using locally sourced ingredients.",
        "Astronauts conducted experiments in the weightlessness of space.",
        "The university library was filled with students preparing for final exams.",
        "A flock of geese flew overhead in a perfect V formation.",
        "The detective examined the crime scene for any overlooked clues.",
        "The artist's latest sculpture was unveiled at the museum's grand opening.",
        "Hikers trekked through dense forests to reach the mountain summit.",
        "The startup company celebrated securing its first round of funding.",
        "Waves crashed against the rocky shore, spraying mist into the air.",
        "The violinist's fingers danced across the strings during the solo performance.",
        "Scientists discovered a new species of deep-sea creature in the Mariana Trench.",
        "The ancient ruins stood as a testament to a long-lost civilization.",
        "Firefighters battled the blaze that threatened to engulf the entire block.",
        "The comedian's jokes had the audience roaring with laughter.",
        "Athletes from around the world gathered for the opening ceremony of the Olympics.",
        "The aroma of freshly baked bread wafted through the small bakery.",
    ];

    for sentence in &sentences {
        vector_db.add_item(sentence);
    }

    // Query sentence
    let query = "Researchers conducted experiments in space to study the effects of weightlessness on plant growth.";

    // Perform k-nearest neighbors search
    let k = 5;
    let results = vector_db.find_k_neighbors(query, k);

    println!("Top {} results for query: \"{}\"", k, query);
    for (i, result) in results.iter().enumerate() {
        println!("{}. {}", i + 1, result);
    }

    Ok(())
}