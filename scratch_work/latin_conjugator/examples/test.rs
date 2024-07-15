use botanical_latin::*;
fn main() {
    println!("Hello, world!");

    let boop = Latin::last_n_chars("be", 3);
    println!("boopik : {:#?}", boop);
    let conji = Latin::new(
        "nouns.csv".into(),
        "adjectives.csv".into(),
        "verbs.csv".into(),
    );

    let complexik = ComplexNoun {
        head_noun: "lorica".into(),
        adjective: "hamatus".into(),
        adposition_noun: "manica".into(),
    };
    let comp2 = conji.complex_noun(&complexik, &Case::Acc, &Number::Plural);
    println!("{:#?}", comp2);

    let testik = conji.noun_map.clone();
    let testik2 = conji.adj_map.clone();
    let testik3 = conji.verb_map.clone();

    /* for wot in testik {
        println!("new_noun : {:#?}", wot);
        let new_noun = conji.noun(&wot.0, , &Number::Singular);
        println!("new_noun : {:#?}", new_noun);
    }
    for wot in testik2 {
        println!("adj : {:#?}", wot);
        let new_noun = conji.adjective(&wot.0, &Case::Acc, &Number::Singular, &Gender::Feminine);
        println!("adj : {:#?}", new_noun);
    }
    for wot in testik3 {
        println!("verb : {:#?}", wot);
        let new_noun = conji.verb(&wot.0, &Mood::Indicative, &Voice::Active, &Tense::Perfect, , &Person::Second);
        println!("verb : {:#?}", new_noun);
    } */
}
