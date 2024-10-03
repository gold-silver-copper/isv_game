use crate::*;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Default, Deserialize)]
struct WordEntry {
    id: i64,
    isv: String,
    addition: String,
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,

    en: String,

    ru: String,
    be: String,
    uk: String,
    pl: String,
    cs: String,
    sk: String,
    sl: String,
    hr: String,
    sr: String,
    mk: String,
    bg: String,
    cu: String,
    de: String,
    nl: String,
    eo: String,
}

impl ISV {
    pub fn initialize_dictionary(&mut self, isv_words: &str) {
        // Open the CSV file
        let file = File::open(isv_words).unwrap();
        let reader = BufReader::new(file);

        // Create a CSV reader
        let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

        // Create a vector to hold words that are m.anim
        let mut m_anim_words: Vec<String> = Vec::new();

        // Iterate through the records
        for result in csv_reader.deserialize() {
            let record: WordEntry = result.unwrap();

            // Check if the partOfSpeech is "m.anim"
            if record.part_of_speech.contains("m.anim.") {
                m_anim_words.push(record.isv.to_lowercase());
            }
        }
        println!("{:#?}", m_anim_words);

        self.animate_nouns = m_anim_words;
    }
}
