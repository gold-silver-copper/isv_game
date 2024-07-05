
pub struct Latin {}

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
    pub acc_sg:&'static str,
    pub gen_sg:&'static str,
    pub dat_sg:&'static str,
    pub abl_sg:&'static str,
    pub loc_sg:&'static str,
    pub voc_sg:&'static str,
    pub nom_pl:&'static str,
    pub acc_pl:&'static str,
    pub gen_pl:&'static str,
    pub dat_pl:&'static str,
    pub abl_pl:&'static str,
    pub loc_pl:&'static str,
    pub voc_pl:&'static str,
}

const TEST_ENDINGS: CaseEndings = CaseEndings {
      nom_sg: "nom_sg",
      acc_sg:"acc_sg",
      gen_sg:"gen_sg",
      dat_sg:"dat_sg",
      abl_sg:"abl_sg",
      loc_sg:"loc_sg",
      voc_sg:"voc_sg",
      nom_pl:"nom_pl",
      acc_pl:"acc_pl",
      gen_pl:"gen_pl",
      dat_pl:"dat_pl",
      abl_pl:"abl_pl",
      loc_pl:"loc_pl",
      voc_pl:"voc_pl",
    
};


// have a possesive func, but reflexive person?
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Sing,
    Plur,
}

type Noun = (String,Gender);


#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
    Reflexive,
}

impl Latin {


    pub fn noun(nominative: String , case: Case , number: Number ) -> Noun {


        ("hello".into(),Gender::Masculine)





    }


}



fn main() {
    println!("Hello, world!");
    println!("meow: {:#?}", TEST_ENDINGS);
}