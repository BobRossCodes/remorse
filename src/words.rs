use std::{collections::HashMap, fs::File, io::BufReader};

use serde_json::Value;

fn calculate_score(word: &str, difficult_letters: &str) -> i32 {
    let mut score = 0;
    for c in word.chars() {
        if let Some(_) = difficult_letters.chars().find(|dc| dc == &c) {
            score += 4;
        }
    }

    score - word.len() as i32
}

pub fn learning_words(difficult_letters: Option<String>) -> anyhow::Result<Vec<String>> {
    let file = File::open("words.json")?;
    let reader = BufReader::new(file);
    let word_list: Value = serde_json::from_reader(reader)?;

    let mut scored_words: HashMap<&str, i32> = HashMap::new();

    for inner_word_list in word_list.as_object().unwrap().values() {
        for word in inner_word_list.as_array().unwrap() {
            scored_words.insert(
                word.as_str().unwrap(),
                calculate_score(
                    word.as_str().unwrap(),
                    &difficult_letters.clone().unwrap_or("".into()),
                ),
            );
        }
    }

    let mut scored_words = scored_words.into_iter().collect::<Vec<(&str, i32)>>();

    scored_words.sort_by(|(_, a), (_, b)| a.cmp(b));

    const SAMPLE_SIZE: usize = 100;

    Ok(
        scored_words[scored_words.len() - SAMPLE_SIZE..scored_words.len()]
            .into_iter()
            .map(|(v, _)| String::from(*v))
            .collect::<Vec<String>>(),
    )
}
