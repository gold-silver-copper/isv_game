use interslavic_rs::*;

fn main() {
    let guessed_noun = ISV::guess_noun("hibiscus", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::guess_noun("maj", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::guess_noun("desna", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::guess_noun("suma", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::guess_noun("mųž", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);

    println!("{:#?}", ISV::string_without_last_n("hello", 2));
    //Output: "hibiscorum"
}
