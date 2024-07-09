use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::collections::HashMap;



type NounMap = HashMap<String,NounRecord>;
type AdjectiveMap = HashMap<String,AdjectiveRecord>;
type VerbMap = HashMap<String,VerbRecord>;



pub struct Latin {

    noun_map: NounMap,
    adj_map: AdjectiveMap,
    verb_map: VerbMap,
}


#[derive(Debug, Deserialize,Clone)]
struct NounRecord {
    word: String,
    nominative: String,
    genitive: String,
    #[serde(deserialize_with = "deserialize_gender")]
    gender: Gender,
    #[serde(deserialize_with = "deserialize_pluralia")]
    pluralia_tantum: bool,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Declension {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    OneTwo,
    Irregular,
}

//word,canonical,present_infinitive,perfect_active,supine,conjugation,irregular
#[derive(Debug, Deserialize,Clone)]
struct VerbRecord {
    word: String,
    canonical: String,
    present_infinitive: String,
    perfect_active: String,
    supine: String,
    
    #[serde(deserialize_with = "deserialize_declension")]
    conjugation: Declension,
    #[serde(deserialize_with = "deserialize_pluralia")]
    irregular: bool,
}


//word,feminine,neuter,comparative,superlative,adverb,declension,adj_stem
#[derive(Debug, Deserialize,Clone)]
struct AdjectiveRecord {
    word: String,
    feminine: String,
    neuter: String,
    comparative: String,
    superlative: String,
    adverb: String,
    #[serde(deserialize_with = "deserialize_declension")]
    declension: Declension,

    adj_stem: String,
}

fn deserialize_declension<'de, D>(deserializer: D) -> Result<Declension, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    match s.as_str() {
        "1" => Ok(Declension::First),
        "2" => Ok(Declension::Second),
        "3" => Ok(Declension::Third),
        "4" => Ok(Declension::Fourth),
        "12" => Ok(Declension::OneTwo),
        "i" => Ok(Declension::Irregular),
        _ => Err(serde::de::Error::custom("unknown declension")),
    }
}

fn deserialize_gender<'de, D>(deserializer: D) -> Result<Gender, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    match s.as_str() {
        "m" => Ok(Gender::Masculine),
        "f" => Ok(Gender::Feminine),
        "n" => Ok(Gender::Neuter),
        _ => Err(serde::de::Error::custom("unknown gender")),
    }
}

fn deserialize_pluralia<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    match s.as_str() {
        "fa" => Ok(false),
        "tr" => Ok(true),

        _ => Err(serde::de::Error::custom("unknown pluralia")),
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Nom,
    Gen,
    Dat,
    Acc,
    Abl,
    Loc,
    Voc,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseEndings {
    pub nom_sg: &'static str,
    pub acc_sg: &'static str,
    pub gen_sg: &'static str,
    pub dat_sg: &'static str,
    pub abl_sg: &'static str,
    pub loc_sg: &'static str,
    pub voc_sg: &'static str,
    pub nom_pl: &'static str,
    pub acc_pl: &'static str,
    pub gen_pl: &'static str,
    pub dat_pl: &'static str,
    pub abl_pl: &'static str,
    pub loc_pl: &'static str,
    pub voc_pl: &'static str,
}

impl CaseEndings {
    pub fn ending(&self, case: Case, number: Number) -> &str {
        match number {
            Number::Singular => match case {
                Case::Nom => self.nom_sg,
                Case::Acc => self.acc_sg,
                Case::Gen => self.gen_sg,
                Case::Dat => self.dat_sg,
                Case::Abl => self.abl_sg,
                Case::Loc => self.loc_sg,
                Case::Voc => self.voc_sg,
            },
            Number::Plural => match case {
                Case::Nom => self.nom_pl,
                Case::Acc => self.acc_pl,
                Case::Gen => self.gen_pl,
                Case::Dat => self.dat_pl,
                Case::Abl => self.abl_pl,
                Case::Loc => self.loc_pl,
                Case::Voc => self.voc_pl,
            },
        }
    }
}

const TEST_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "nom_sg",
    acc_sg: "acc_sg",
    gen_sg: "gen_sg",
    dat_sg: "dat_sg",
    abl_sg: "abl_sg",
    loc_sg: "loc_sg",
    voc_sg: "voc_sg",
    nom_pl: "nom_pl",
    acc_pl: "acc_pl",
    gen_pl: "gen_pl",
    dat_pl: "dat_pl",
    abl_pl: "abl_pl",
    loc_pl: "loc_pl",
    voc_pl: "voc_pl",
};

// have a possesive func, but reflexive person?
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Singular,
    Plural,
}

type Noun = (String, Gender);

#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
    Reflexive,
}

impl Latin {



    pub fn new() -> Self {

        Latin {
            noun_map: Latin::load_nouns_from_csv(),
            adj_map: Latin::load_adjectives_from_csv(),
            verb_map: Latin::load_verbs_from_csv(),
        }
    }

    pub fn load_nouns_from_csv() -> NounMap{

        let mut nounmap = HashMap::new();
        let file_path = "nouns.csv";

        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.deserialize() {
            let record: NounRecord = result.unwrap();


            nounmap.insert(record.word.clone(),record.clone());

            println!("{:?}", record);
        }
        nounmap
    }
    pub fn load_adjectives_from_csv() -> AdjectiveMap{
        let file_path = "adjectives.csv";
        let mut adjmap = HashMap::new();
        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.deserialize() {
            println!("{:?}", result);
            let record: AdjectiveRecord = result.unwrap();
            adjmap.insert(record.word.clone(),record.clone());
            println!("{:?}", record);
        }
        adjmap
    }

    pub fn load_verbs_from_csv() -> VerbMap{
        let file_path = "verbs.csv";
        let mut verbmap = HashMap::new();
        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.deserialize() {
            println!("{:?}", result);
            let record: VerbRecord = result.unwrap();
            verbmap.insert(record.word.clone(),record.clone());
            println!("{:?}", record);
        }
        verbmap
    }
    pub fn noun(nominative: &str, case: Case, number: Number) -> Noun {
        let stem = nominative;
        let ending = TEST_ENDINGS.ending(case, number);
        let gender = Latin::guess_gender(nominative);
        let conjugated_noun = format!("{}{}", stem, ending);
        (conjugated_noun, gender)
    }
    pub fn guess_gender(nominative: &str) -> Gender {
        Gender::Feminine
    }
    pub fn last_n_chars(word: &str, n: usize) -> String {
        let split_pos = word.char_indices().nth_back(n - 1).unwrap_or((0, 'a')).0;
        word[split_pos..].into()
    }
}

fn main() {
    println!("Hello, world!");
    println!("meow: {:#?}", TEST_ENDINGS);
    println!(
        "desu: {:#?}",
        TEST_ENDINGS.ending(Case::Nom, Number::Singular)
    );
    let boop = Latin::last_n_chars("be", 3);
    println!("boopik : {:#?}", boop);
    let new_noun = Latin::noun("gladius", Case::Acc, Number::Singular);
    println!("new_noun : {:#?}", new_noun);

    Latin::load_nouns_from_csv();
    Latin::load_adjectives_from_csv();
    Latin::load_verbs_from_csv();

    Latin::new();
}
