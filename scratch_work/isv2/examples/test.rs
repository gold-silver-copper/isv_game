use interslavic_rs::*;

fn main() {
    let guessed_noun = ISV::decline_noun("hibiscus", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::decline_noun("maj", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::decline_noun("desna", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::decline_noun("suma", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::decline_noun("mųž", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_adj = ISV::decline_adj(
        "samy",
        &Case::Gen,
        &Number::Singular,
        &Gender::Masculine,
        true,
    );
    println!("{:#?}", guessed_adj);
    let guessed_adj = ISV::decline_adj(
        "samy",
        &Case::Gen,
        &Number::Singular,
        &Gender::Masculine,
        true,
    );
    println!("{:#?}", guessed_adj);
    let guessed_adj = ISV::decline_adj(
        "teply",
        &Case::Gen,
        &Number::Singular,
        &Gender::Neuter,
        true,
    );
    println!("{:#?}", guessed_adj);
    let guessed_adj = ISV::decline_adj(
        "nizky",
        &Case::Gen,
        &Number::Singular,
        &Gender::Feminine,
        true,
    );
    println!("{:#?}", guessed_adj);

    let verbiki = ["učiti", "briti", "sniti", "obriti"];

    for verbik in verbiki {
        let guessed_verb = ISV::conjugate_verb(
            verbik,
            &Person::First,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = ISV::conjugate_verb(
            verbik,
            &Person::Second,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = ISV::conjugate_verb(
            verbik,
            &Person::Third,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = ISV::conjugate_verb(
            verbik,
            &Person::First,
            &Number::Plural,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = ISV::conjugate_verb(
            verbik,
            &Person::Second,
            &Number::Plural,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = ISV::conjugate_verb(
            verbik,
            &Person::Third,
            &Number::Plural,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
    }

    println!("{:#?}", ISV::string_without_last_n("hello", 2));
    //Output: "hibiscorum"
}
