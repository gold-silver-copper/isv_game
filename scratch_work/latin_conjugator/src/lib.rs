use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::error::Error;

pub type NounMap = HashMap<String, NounRecord>;
pub type AdjectiveMap = HashMap<String, AdjectiveRecord>;
pub type VerbMap = HashMap<String, VerbRecord>;
pub type Verb = String;


#[derive(Debug, Deserialize, Clone, Default)]
pub struct Latin {
    pub noun_map: NounMap,
    pub adj_map: AdjectiveMap,
    pub verb_map: VerbMap,
}

pub struct ComplexNoun {
    //   pub case: Case,
    //  pub number: Number,
    pub head_noun: String,
    pub adjective: Vec<String>,
    pub adposition_noun: Vec<String>,
}

impl Default for ComplexNoun {
    fn default() -> Self {
        Self {
            head_noun: "exemplum".into(),
            adposition_noun: Vec::new(),
            adjective: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct NounRecord {
    pub word: String,
    pub nom_sg: String,
    pub gen_sg: String,
    pub dat_sg: String,
    pub acc_sg: String,
    pub abl_sg: String,
    pub voc_sg: String,
    pub loc_sg: String,
    pub nom_pl: String,
    pub gen_pl: String,
    pub dat_pl: String,
    pub acc_pl: String,
    pub abl_pl: String,
    pub voc_pl: String,
    pub loc_pl: String,

    #[serde(deserialize_with = "deserialize_gender")]
    pub gender: Gender,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mood {
    Indicative,
    Subjunctive,
    Imperative,
    Infinitive,
    Participle,
    VerbalNoun,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Voice {
    Active,
    Passive,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Perfect,
    Pluperfect,
    FuturePerfect,
}

impl Default for Gender {
    fn default() -> Gender {
        Gender::Masculine
    }
}

//word,canonical,present_infinitive,perfect_active,supine,conjugation,irregular
#[derive(Debug, Deserialize, Clone, Default)]
pub struct VerbRecord {
    pub word: String,
    pub canonical: String,
    pub present_infinitive: String,
    pub perfect_active: String,
    pub supine: String,
    pub indicative_active_present_singular_first: String,
    pub indicative_active_present_singular_second: String,
    pub indicative_active_present_singular_third: String,
    pub indicative_active_present_plural_first: String,
    pub indicative_active_present_plural_second: String,
    pub indicative_active_present_plural_third: String,
    pub indicative_active_imperfect_singular_first: String,
    pub indicative_active_imperfect_singular_second: String,
    pub indicative_active_imperfect_singular_third: String,
    pub indicative_active_imperfect_plural_first: String,
    pub indicative_active_imperfect_plural_second: String,
    pub indicative_active_imperfect_plural_third: String,
    pub indicative_active_future_singular_first: String,
    pub indicative_active_future_singular_second: String,
    pub indicative_active_future_singular_third: String,
    pub indicative_active_future_plural_first: String,
    pub indicative_active_future_plural_second: String,
    pub indicative_active_future_plural_third: String,
    pub indicative_active_perfect_singular_first: String,
    pub indicative_active_perfect_singular_second: String,
    pub indicative_active_perfect_singular_third: String,
    pub indicative_active_perfect_plural_first: String,
    pub indicative_active_perfect_plural_second: String,
    pub indicative_active_perfect_plural_third: String,
    pub indicative_active_pluperfect_singular_first: String,
    pub indicative_active_pluperfect_singular_second: String,
    pub indicative_active_pluperfect_singular_third: String,
    pub indicative_active_pluperfect_plural_first: String,
    pub indicative_active_pluperfect_plural_second: String,
    pub indicative_active_pluperfect_plural_third: String,
}

//word,feminine,neuter,comparative,superlative,adverb,declension,adj_stem
#[derive(Debug, Deserialize, Clone, Default)]
pub struct AdjectiveRecord {
    pub word: String,

    pub comparative: String,
    pub superlative: String,
    pub adverb: String,
    pub nom_sg_masc: String,
    pub gen_sg_masc: String,
    pub dat_sg_masc: String,
    pub acc_sg_masc: String,
    pub abl_sg_masc: String,
    pub nom_sg_fem: String,
    pub gen_sg_fem: String,
    pub dat_sg_fem: String,
    pub acc_sg_fem: String,
    pub abl_sg_fem: String,
    pub nom_sg_neut: String,
    pub gen_sg_neut: String,
    pub dat_sg_neut: String,
    pub acc_sg_neut: String,
    pub abl_sg_neut: String,
    pub nom_pl_masc: String,
    pub gen_pl_masc: String,
    pub dat_pl_masc: String,
    pub acc_pl_masc: String,
    pub abl_pl_masc: String,
    pub nom_pl_fem: String,
    pub gen_pl_fem: String,
    pub dat_pl_fem: String,
    pub acc_pl_fem: String,
    pub abl_pl_fem: String,
    pub nom_pl_neut: String,
    pub gen_pl_neut: String,
    pub dat_pl_neut: String,
    pub acc_pl_neut: String,
    pub abl_pl_neut: String,
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

// have a possesive func, but reflexive person?
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Singular,
    Plural,
}

pub type Noun = (String, Gender);
pub type Adjective = String;

#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
}

impl Latin {
    pub fn new(noun_path: String, adjective_path: String, verb_path: String) -> Self {
        Latin {
            noun_map: Latin::load_nouns_from_csv(noun_path),
            adj_map: Latin::load_adjectives_from_csv(adjective_path),
            verb_map: Latin::load_verbs_from_csv(verb_path),
        }
    }

    //"nouns.csv"
    pub fn load_nouns_from_csv(path: String) -> NounMap {
        let mut nounmap = HashMap::new();
        let file_path: &str = path.as_str();

        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.deserialize() {
            let record: NounRecord = result.unwrap();

            nounmap.insert(record.word.clone(), record.clone());

            //    //println!("{:?}", record);
        }
        nounmap
    }
    pub fn load_adjectives_from_csv(path: String) -> AdjectiveMap {
        let file_path: &str = path.as_str();
        let mut adjmap = HashMap::new();
        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.deserialize() {
            //   //println!("{:?}", result);
            let record: AdjectiveRecord = result.unwrap();
            adjmap.insert(record.word.clone(), record.clone());
            //  //println!("{:?}", record);
        }
        adjmap
    }

    pub fn load_verbs_from_csv(path: String) -> VerbMap {
        let file_path: &str = path.as_str();
        let mut verbmap = HashMap::new();
        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.deserialize() {
            //println!("{:?}", result);
            let record: VerbRecord = result.unwrap();
            verbmap.insert(record.word.clone(), record.clone());
            //println!("{:?}", record);
        }
        verbmap
    }

    pub fn complex_noun(
        &self,
        complex_nomen: &ComplexNoun,
        case: &Case,
        number: &Number,
    ) -> String {
        let noun = self.noun(&complex_nomen.head_noun, case, number);

        let mut response = noun.0;


        for adpos in &complex_nomen.adposition_noun {

            let adposik = self.noun(adpos, case, number);
            if adposik.0 != "" {
                response = format!("{} {}", response, adposik.0);
            }


        }
      
        for adj in &complex_nomen.adjective {



            let adjik = self.adjective(adj, case, number, &noun.1);
            if adjik != "" {
                response = format!("{} {}", response, adjik);
            }

        }



        

       

      
       

        response
    }

    pub fn noun(&self, word: &str, case: &Case, number: &Number) -> Noun {
        let defik = NounRecord::default();

        let record = self.noun_map.get(word).unwrap_or(&defik);

        let mut response = match number {
            Number::Singular => match case {
                Case::Nom => (record.nom_sg.clone(), record.gender.clone()),
                Case::Gen => (record.gen_sg.clone(), record.gender.clone()),
                Case::Dat => (record.dat_sg.clone(), record.gender.clone()),
                Case::Acc => (record.acc_sg.clone(), record.gender.clone()),
                Case::Abl => (record.abl_sg.clone(), record.gender.clone()),
                Case::Voc => (record.voc_sg.clone(), record.gender.clone()),
                Case::Loc => (record.loc_sg.clone(), record.gender.clone()),
            },
            Number::Plural => match case {
                Case::Nom => (record.nom_pl.clone(), record.gender.clone()),
                Case::Gen => (record.gen_pl.clone(), record.gender.clone()),
                Case::Dat => (record.dat_pl.clone(), record.gender.clone()),
                Case::Acc => (record.acc_pl.clone(), record.gender.clone()),
                Case::Abl => (record.abl_pl.clone(), record.gender.clone()),
                Case::Voc => (record.voc_pl.clone(), record.gender.clone()),
                Case::Loc => (record.loc_pl.clone(), record.gender.clone()),
            },
        };

        if case == &Case::Loc && (response.0 == "" || response.0 == "-") {
            response.0 = format!("in {}", record.abl_sg.clone());
        }

        if (response.0 == "" || response.0 == "-") {
            response.0 = "".into();
        }

        response
    }

    pub fn adjective(
        &self,
        word: &str,
        case: &Case,
        number: &Number,
        gender: &Gender,
    ) -> Adjective {
        let defik = AdjectiveRecord::default();

        let record = self.adj_map.get(word).unwrap_or(&defik);

        match gender {
            Gender::Masculine => match number {
                Number::Singular => match case {
                    Case::Nom => record.nom_sg_masc.clone(),
                    Case::Gen => record.gen_sg_masc.clone(),
                    Case::Dat => record.dat_sg_masc.clone(),
                    Case::Acc => record.acc_sg_masc.clone(),
                    Case::Abl => record.abl_sg_masc.clone(),
                    _ => record.abl_sg_masc.clone(),
                },
                Number::Plural => match case {
                    Case::Nom => record.nom_pl_masc.clone(),
                    Case::Gen => record.gen_pl_masc.clone(),
                    Case::Dat => record.dat_pl_masc.clone(),
                    Case::Acc => record.acc_pl_masc.clone(),
                    Case::Abl => record.abl_pl_masc.clone(),
                    _ => record.abl_pl_masc.clone(),
                },
            },
            Gender::Feminine => match number {
                Number::Singular => match case {
                    Case::Nom => record.nom_sg_fem.clone(),
                    Case::Gen => record.gen_sg_fem.clone(),
                    Case::Dat => record.dat_sg_fem.clone(),
                    Case::Acc => record.acc_sg_fem.clone(),
                    Case::Abl => record.abl_sg_fem.clone(),
                    _ => record.abl_sg_fem.clone(),
                },
                Number::Plural => match case {
                    Case::Nom => record.nom_pl_fem.clone(),
                    Case::Gen => record.gen_pl_fem.clone(),
                    Case::Dat => record.dat_pl_fem.clone(),
                    Case::Acc => record.acc_pl_fem.clone(),
                    Case::Abl => record.abl_pl_fem.clone(),
                    _ => record.abl_pl_fem.clone(),
                },
            },
            Gender::Neuter => match number {
                Number::Singular => match case {
                    Case::Nom => record.nom_sg_neut.clone(),
                    Case::Gen => record.gen_sg_neut.clone(),
                    Case::Dat => record.dat_sg_neut.clone(),
                    Case::Acc => record.acc_sg_neut.clone(),
                    Case::Abl => record.abl_sg_neut.clone(),
                    _ => record.abl_sg_neut.clone(),
                },
                Number::Plural => match case {
                    Case::Nom => record.nom_pl_neut.clone(),
                    Case::Gen => record.gen_pl_neut.clone(),
                    Case::Dat => record.dat_pl_neut.clone(),
                    Case::Acc => record.acc_pl_neut.clone(),
                    Case::Abl => record.abl_pl_neut.clone(),
                    _ => record.abl_pl_neut.clone(),
                },
            },
        }
    }

    pub fn verb(
        &self,
        word: &str,
        mood: &Mood,
        voice: &Voice,
        tense: &Tense,
        number: &Number,
        person: &Person,
    ) -> Verb {
        let defik = VerbRecord::default();

        let record = self.verb_map.get(word).unwrap_or(&defik);

        match mood {
            Mood::Indicative => match voice {
                Voice::Active => match tense {
                    Tense::Present => match number {
                        Number::Singular => match person {
                            Person::First => {
                                record.indicative_active_present_singular_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_present_singular_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_present_singular_third.clone()
                            }
                        },
                        Number::Plural => match person {
                            Person::First => record.indicative_active_present_plural_first.clone(),
                            Person::Second => {
                                record.indicative_active_present_plural_second.clone()
                            }
                            Person::Third => record.indicative_active_present_plural_third.clone(),
                        },
                    },
                    Tense::Imperfect => match number {
                        Number::Singular => match person {
                            Person::First => {
                                record.indicative_active_imperfect_singular_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_imperfect_singular_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_imperfect_singular_third.clone()
                            }
                        },
                        Number::Plural => match person {
                            Person::First => {
                                record.indicative_active_imperfect_plural_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_imperfect_plural_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_imperfect_plural_third.clone()
                            }
                        },
                    },
                    Tense::Future => match number {
                        Number::Singular => match person {
                            Person::First => record.indicative_active_future_singular_first.clone(),
                            Person::Second => {
                                record.indicative_active_future_singular_second.clone()
                            }
                            Person::Third => record.indicative_active_future_singular_third.clone(),
                        },
                        Number::Plural => match person {
                            Person::First => record.indicative_active_future_plural_first.clone(),
                            Person::Second => record.indicative_active_future_plural_second.clone(),
                            Person::Third => record.indicative_active_future_plural_third.clone(),
                        },
                    },
                    Tense::Perfect => match number {
                        Number::Singular => match person {
                            Person::First => {
                                record.indicative_active_perfect_singular_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_perfect_singular_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_perfect_singular_third.clone()
                            }
                        },
                        Number::Plural => match person {
                            Person::First => record.indicative_active_perfect_plural_first.clone(),
                            Person::Second => {
                                record.indicative_active_perfect_plural_second.clone()
                            }
                            Person::Third => record.indicative_active_perfect_plural_third.clone(),
                        },
                    },
                    Tense::Pluperfect => match number {
                        Number::Singular => match person {
                            Person::First => {
                                record.indicative_active_pluperfect_singular_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_pluperfect_singular_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_pluperfect_singular_third.clone()
                            }
                        },
                        Number::Plural => match person {
                            Person::First => {
                                record.indicative_active_pluperfect_plural_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_pluperfect_plural_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_pluperfect_plural_third.clone()
                            }
                        },
                    },
                    Tense::FuturePerfect => {
                        todo!("IMPLEMENT FUTURE PERFECT")
                    }
                },
                _ => todo!("IMPLEMENT PASSIVE VOICE"),
            },
            _ => todo!("IMPLEMENT OTHER MOODS"),
        }
    }

    pub fn last_n_chars(word: &str, n: usize) -> String {
        let split_pos = word.char_indices().nth_back(n - 1).unwrap_or((0, 'a')).0;
        word[split_pos..].into()
    }
}

/*

fn main() {
    //println!("Hello, world!");

    let boop = Latin::last_n_chars("be", 3);
    //println!("boopik : {:#?}", boop);
    let conji = Latin::new();

    let testik = conji.noun_map.clone();
    let testik2 = conji.adj_map.clone();
    let testik3 = conji.verb_map.clone();

    for wot in testik {
        //println!("new_noun : {:#?}", wot);
        let new_noun = conji.noun(&wot.0, &Case::Acc, &Number::Singular);
        //println!("new_noun : {:#?}", new_noun);
    }
    for wot in testik2 {
        //println!("adj : {:#?}", wot);
        let new_noun = conji.adjective(&wot.0, &Case::Acc, &Number::Singular, &Gender::Feminine);
        //println!("adj : {:#?}", new_noun);
    }
    for wot in testik3 {
        //println!("verb : {:#?}", wot);
        let new_noun = conji.verb(&wot.0, &Mood::Indicative, &Voice::Active, &Tense::Perfect, &Number::Plural, &Person::Second);
        //println!("verb : {:#?}", new_noun);
    }
}


*/
