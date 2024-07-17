use crate::*;

// fishes, amphibians, reptiles, birds, and mammals.
#[derive(Clone, Debug, Display, PartialEq)]
pub enum ImperiumNaturae {
    Regio,
    Regnum,
    Phylum,
    Classis,
    Ordo,
    Familia,
    Genus,
    Species,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Regna {
    Animalia,
    Vegetabilia,
    Lapides,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Lapides {
    Petrae,
    Minerae,
    Fossilia,
    Vitamentra,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum SubspeciesAdjective {
    Maximus,
    Australis,
    Borealis,
    Familiaris,
    Domesticus,
    Minor,
    Aureus,
    Unicus,
    Vulgatis,
    Albus,
    Agrarius,
    Minutus,
    Barbarus,
    Timidus,
    Alpinus,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Animalia {
    Mammalia,
    Aves,
    Amphibia,
    Pisces,
    Insecta,
    Vermes,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Aves {
    Accipitres, //hawk
    Picae,      //magpie
    Anseres,    //gooese
    Grallae,    //stilt
    Gallinae,
    Passeres,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Pica {
    Trochilus,
    Certhia,
    Upupa,
    Glaucopis,

    Buphaga,
    Sitta,
    Oriolus,
    Coracias,
    Gracula,

    Corvus,
    Paradisea,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Accipiter {
    Falco,
    Vultur,
    Strix,
    Lanius,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Mammalia {
    Primates,
    Bruta,
    Ferae,
    Glires,
    Pecora,
    Belluae,
    Cete,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Primates {
    Homo,
    Simia,
    Lemur,
    Vespertilio,
    Troglodytes,
    Vampyrus,
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Bruta {
    Rhinoceros,
    Unicornis,
    Bicornis,
    Elephas,
    Trichechus,
    Bradypus,
    Myrmecophaga,
    Manis,   //Armadillo like thing scaly
    Dasypus, //rabbit
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Ferae {
    Phoca, //seal
    Canis,
    Lupus,
    Hyaena,
    Lycaon,
    Vulpes,
    Felis,
    Leo,
    Catus,
    Tigris,
    Serval,
    Caracal,
    Lynx,
    Leopardus,
    Viverra, //ferret
    Mustela, //weasel
    Ursus,
    Didelphis,
    Talpa,
    Sorex,
    Erinaceus,
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Glires {
    Hystrix,
    Castor,
    Mus,
    Rattus,
    Arctomys,
    Sciurus,
    Myoxus,
    Dipus,
    Lepus,
    Hyrox,
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Pecora {
    Camelus,
    Moschus,
    Giraffa,
    Cervus,
    Antilope,
    Capra,
    Ovis,
    Aries,
    Taurus,

    Bos,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Belluae {
    Equus,
    Zebra,

    Hippopotamus,
    Tapir,
    Sus,
    Porcus,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Cete {
    Monodon,
    Balaena,
    Physeter,
    Delphinus,
    Boops,
    Catodon,
    Orca,
}
