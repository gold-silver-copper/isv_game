use animate_nouns::ANIMATE_NOUNS;

mod animate_nouns;
mod case_endings;
use case_endings::*;

#[derive(Debug, Clone, Default)]
pub struct ISV {}
#[derive(Debug, PartialEq, Clone)]
pub struct ComplexNoun {
    pub head_noun: String,
    pub adjective: Vec<String>,
}

impl Default for ComplexNoun {
    fn default() -> Self {
        Self {
            head_noun: "exemplum".into(),

            adjective: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Singular,
    Plural,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Nom,
    Acc,
    Gen,
    Loc,
    Dat,
    Ins,
    //vocative will be handle seperately
}
#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
}

impl CaseEndings {
    pub fn ending(&self, case: &Case, number: &Number) -> &str {
        match number {
            Number::Singular => match case {
                Case::Nom => self.nom_sg,
                Case::Acc => self.acc_sg,
                Case::Gen => self.gen_sg,
                Case::Loc => self.loc_sg,
                Case::Dat => self.dat_sg,
                Case::Ins => self.ins_sg,
            },
            Number::Plural => match case {
                Case::Nom => self.nom_pl,
                Case::Acc => self.acc_pl,
                Case::Gen => self.gen_pl,
                Case::Loc => self.loc_pl,
                Case::Dat => self.dat_pl,
                Case::Ins => self.ins_pl,
            },
        }
    }
}

pub type Noun = (String, Gender);
pub type Adjective = String;
pub type Verb = String;

pub const VOWELS: &[char] = &[
    'a', 'e', 'i', 'í', 'ó', 'o', 'u', 'å', 'ą', 'ę', 'ė', 'é', 'ȯ', 'ų', 'ů', 'ú', 'ý', 'y', 'ě',
    'A', 'E', 'I', 'O', 'U', 'á',
];

pub const HARD_CONSONANTS: &[char] = &[
    'p', 'b', 'f', 'v', 'm', 's', 'z', 't', 'd', 'r', 'n', 'l', 'k', 'g', 'h',
];

impl ISV {
    pub fn guess_noun(word: &str, case: &Case, number: &Number) -> Noun {
        let gender = ISV::guess_gender(word);
        let word_is_animate = ISV::noun_is_animate(word);
        let word_stem_is_soft = ISV::stem_of_noun_is_soft(word);
        let word_stem = ISV::get_noun_stem(word, number);

        let endings = if ISV::is_ost_class(word) {
            &OST_ENDINGS
        } else {
            match gender {
                Gender::Masculine => {
                    if word_is_animate {
                        if word_stem_is_soft {
                            &ANIMATE_SOFT_ENDINGS
                        } else {
                            &ANIMATE_HARD_ENDINGS
                        }
                    } else {
                        if word_stem_is_soft {
                            &INANIMATE_SOFT_ENDINGS
                        } else {
                            &INANIMATE_HARD_ENDINGS
                        }
                    }
                }
                Gender::Feminine => {
                    if word_stem_is_soft {
                        &FEMININE_SOFT_ENDINGS
                    } else {
                        &FEMININE_HARD_ENDINGS
                    }
                }
                Gender::Neuter => {
                    if word_stem_is_soft {
                        &NEUTER_SOFT_ENDINGS
                    } else {
                        &NEUTER_HARD_ENDINGS
                    }
                }
            }
        };

        let ending = endings.ending(case, number);
        let merged = format!("{}{}", word_stem, ending);
        return (merged, gender.clone());
    }
    pub fn noun_is_animate(word: &str) -> bool {
        ANIMATE_NOUNS.contains(&word)
    }

    pub fn guess_gender(word: &str) -> Gender {
        let last_one = ISV::last_n_chars(word, 1);

        if ISV::is_ost_class(word) || (last_one == "a") || (last_one == "i") {
            return Gender::Feminine;
        } else if (last_one == "o") || (last_one == "e") {
            return Gender::Neuter;
        } else {
            return Gender::Masculine;
        }
    }

    pub fn last_n_chars(word: &str, n: usize) -> String {
        let split_pos = word.char_indices().nth_back(n - 1).unwrap_or((0, 'a')).0;
        word[split_pos..].into()
    }
    pub fn is_ost_class(word: &str) -> bool {
        word.ends_with("ost́")
    }

    pub fn get_noun_stem(word: &str, number: &Number) -> String {
        if word.ends_with("anin") && number == &Number::Plural {
            return ISV::string_without_last_n(word, 2);
        }
        if word.ends_with("anina") && number == &Number::Plural {
            return ISV::string_without_last_n(word, 3);
        }

        if ISV::is_vowel(&ISV::last_in_stringslice(word)) {
            return ISV::string_without_last_n(word, 1);
        } else {
            return String::from(word);
        }
    }
    pub fn is_vowel(c: &char) -> bool {
        VOWELS.contains(c)
    }

    pub fn stem_of_noun_is_soft(word: &str) -> bool {
        ISV::ends_with_soft_consonant(&ISV::get_noun_stem(word, &Number::Singular))
    }
    pub fn ends_with_soft_consonant(word: &str) -> bool {
        ISV::is_soft_consonant(&ISV::last_in_stringslice(word))
    }

    pub fn is_hard_consonant(c: &char) -> bool {
        HARD_CONSONANTS.contains(c)
    }

    pub fn is_soft_consonant(c: &char) -> bool {
        !ISV::is_hard_consonant(c) && !ISV::is_vowel(c)
    }
    pub fn last_in_stringslice(s: &str) -> char {
        s.to_string().pop().unwrap_or(' ')
    }
    pub fn string_without_last_n(s: &str, n: i64) -> String {
        let mut stringik = s.to_string();
        for _ in 0..n {
            stringik.pop();
        }

        stringik
    }
}
