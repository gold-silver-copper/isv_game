use crate::*;

pub trait ToColor {
    fn color(&self) -> RatColor;
}

impl ToColor for Tree {
    fn color(&self) -> RatColor {
        match &self {
            Tree::Glinos => RatColor::Rgb(160, 82, 45),
        }
    }
}

impl ToColor for Profession {
    fn color(&self) -> RatColor {
        match &self {
            Profession::Sluga => RatColor::Rgb(128, 128, 128), // Grey symbolizes servitude and neutrality, blending into the background.
            Profession::Žebrak | Profession::Prośak => RatColor::Rgb(90, 45, 25), // A dull, earthy brown, representing poverty and hardship.
            Profession::Vęzėnj | Profession::Zaključiĺnik => RatColor::Rgb(100, 50, 50), // A faded red, symbolizing blood, guilt, or punishment.
            Profession::Ubijca => RatColor::Rgb(153, 0, 0), // Dark red to symbolize violence, danger, and bloodshed.
            Profession::Rycaŕ | Profession::Vitęź => RatColor::Rgb(0, 0, 153), // Deep blue represents nobility, loyalty, and valor.
            Profession::Varvar => RatColor::Rgb(102, 51, 0), // Brown with a wild, earthy tone to represent untamed and primitive strength.
            Profession::Trgovec | Profession::Kupec => RatColor::Rgb(60, 82, 45), // Muted green, symbolizing money and commerce, but also modesty.
            Profession::Rybak => RatColor::Rgb(0, 102, 204), // A deep blue, reminiscent of the sea and water.
            Profession::Kovač => RatColor::Rgb(70, 70, 70), // Dark grey symbolizes iron and metalworking.
            Profession::Zemjedělec => RatColor::Rgb(34, 139, 34), // A vibrant green, representing the earth and crops.
            Profession::Lovec => RatColor::Rgb(85, 107, 47), // Olive green, symbolizing camouflage and nature.
            Profession::Mųdrec => RatColor::Rgb(153, 51, 255), // A mystical purple, representing wisdom and the arcane.
            Profession::Čarovnik => RatColor::Rgb(75, 0, 130), // Dark indigo represents deep magic and mystery.
            Profession::Dezerter => RatColor::Rgb(105, 105, 105), // Dark grey for the idea of abandonment and dishonor.
            Profession::Žertvaŕ => RatColor::Rgb(178, 34, 34), // Blood-red symbolizes sacrifice and ritualistic importance.
            Profession::Volhvaŕ => RatColor::Rgb(139, 0, 139), // Deep magenta represents the priestly-magical combination of the sorcerer-priest.
        }
    }
}

impl ToColor for AnimalType {
    fn color(&self) -> RatColor {
        match &self {
            AnimalType::Loś => RatColor::Rgb(139, 69, 19), // Moose: a dark brown, representing the large, earthy animal.
            AnimalType::Běs => RatColor::Rgb(255, 0, 0), // Demon: fiery red, symbolizing its infernal nature.
            AnimalType::Djavȯl => RatColor::Rgb(0, 0, 0), // Devil: pitch black, representing darkness and evil.
            AnimalType::Jelenj => RatColor::Rgb(160, 82, 45), // Deer: light brown, symbolizing grace and the forest.
            AnimalType::Krava => RatColor::Rgb(255, 255, 240), // Cow: a soft off-white, often seen in dairy cows.
            AnimalType::Pes => RatColor::Rgb(105, 105, 105), // Dog: a medium grey, representing domestication and loyalty.
            AnimalType::Tigr => RatColor::Rgb(255, 165, 0), // Tiger: bright orange with hints of ferocity.
            AnimalType::Gad => RatColor::Rgb(34, 139, 34), // Reptile: dark green, evoking the scaly nature of many reptiles.
            AnimalType::Jaščer => RatColor::Rgb(50, 205, 50), // Lizard: bright green for its more agile and quick nature.
            AnimalType::Iguana => RatColor::Rgb(0, 128, 0), // Iguana: deep green, reflecting its larger, more tropical nature.
            AnimalType::Vųž => RatColor::Rgb(85, 107, 47), // Snake: olive green for its stealth and earthy connection.
            AnimalType::Žȯlv => RatColor::Rgb(46, 139, 87), // Turtle: a sea-green, representing its aquatic tendencies.
            AnimalType::Sova => RatColor::Rgb(169, 169, 169), // Owl: grey, symbolizing wisdom and night.
            AnimalType::Vrabec => RatColor::Rgb(150, 75, 0), // Sparrow: brown with hints of liveliness, representing small birds.
            AnimalType::Vran => RatColor::Rgb(0, 0, 0), // Crow: pure black, symbolizing its association with mystery and death.
            AnimalType::Gavran => RatColor::Rgb(0, 0, 0), // Raven: similarly black for its mythological and ominous presence.
            AnimalType::Kos => RatColor::Rgb(0, 0, 0), // Blackbird: again black, for obvious reasons.
            AnimalType::Gųsę => RatColor::Rgb(255, 255, 255), // Goose (plural): pure white, symbolizing domesticated birds.
            AnimalType::Gųś => RatColor::Rgb(255, 255, 255), // Goose (singular): same white as Gųsę for consistency.
            AnimalType::Vȯlk => RatColor::Rgb(128, 128, 128), // Wolf: a balanced grey, representing the wild and pack instinct.
            AnimalType::Vȯlkolak => RatColor::Rgb(70, 70, 70), // Werewolf: darker grey, symbolizing its fearsome, shadowy nature.
            AnimalType::Vȯlkodav => RatColor::Rgb(105, 105, 105), // Wolfhound: slightly lighter grey, representing the domesticated wolf-hunter.
        }
    }
}
