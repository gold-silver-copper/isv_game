#[derive(Debug,  Clone, Default)]
pub struct ISV {

}
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
    Plural
}
#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Nom,
    Acc,
    Gen,
    Loc,
    Dat,
    Ins,
    Voc
}
#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter
}

#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseEndings {
    pub gender: Gender,
    pub nom_sg: &'static str,
    pub acc_sg: &'static str,
    pub gen_sg: &'static str,
    pub loc_sg: &'static str,
    pub dat_sg: &'static str,
    pub ins_sg: &'static str,
    pub voc_sg: &'static str,

    pub nom_pl: &'static str,
    pub acc_pl: &'static str,
    pub gen_pl: &'static str,
    pub loc_pl: &'static str,
    pub dat_pl: &'static str,
    pub ins_pl: &'static str,
    pub voc_pl: &'static str,
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
                Case::Voc => self.voc_sg,
            },
            Number::Plural => match case {
                Case::Nom => self.nom_pl,
                Case::Acc => self.acc_pl,
                Case::Gen => self.gen_pl,
                Case::Loc => self.loc_pl,
                Case::Dat => self.dat_pl,
                Case::Ins => self.ins_pl,
                Case::Voc => self.voc_pl,
            },
        }
    }
}


pub const TEST_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "nom_sg",  // Nominative Singular
    acc_sg: "acc_sg",  // Accusative Singular
    gen_sg: "gen_sg",  // Genitive Singular
    loc_sg: "loc_sg",  // Locative Singular
    dat_sg: "dat_sg",  // Dative Singular
    ins_sg: "ins_sg",  // Instrumental Singular
    voc_sg: "voc_sg",  // Vocative Singular

    nom_pl: "nom_pl",  // Nominative Plural
    acc_pl: "acc_pl",  // Accusative Plural
    gen_pl: "gen_pl",  // Genitive Plural
    loc_pl: "loc_pl",  // Locative Plural
    dat_pl: "dat_pl",  // Dative Plural
    ins_pl: "ins_pl",  // Instrumental Plural
    voc_pl: "voc_pl",  // Vocative Plural
};


pub type Noun = (String, Gender);
pub type Adjective = String;
pub type Verb = String;

impl ISV {

    pub fn guess_gender(word: &str) -> Gender {
        let last_one = ISV::last_n_chars(word, 1);

        let last_three = ISV::last_n_chars(word, 3);

        assert_eq!("ost́", ISV::last_n_chars("kost́", 4));

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
        let last_four = ISV::last_n_chars(word, 4);
        //the only exception is gost́ (m anim, not ost class conjugation)
        last_four == String::from("ost́")
    }


}