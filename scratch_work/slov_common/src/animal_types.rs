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
    Major,
    Aureus,
    Unicus,
    Vulgatis,
    Albus,
    Agrarius,
    Minutus,
    Barbarus,
    Timidus,
    Alpinus,
    Islandicus,
    Obscurus,
    Acadicus,
    Magnus,
    Agilis,
    Flavus,
    Superus,
    Arboreus,
    Velox,

    Pacificus,
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
pub enum Pisces {
    Apodes,
    Jugulares,
    Thoracici,
    Abdominales,
    Branchiotegi,
    Chondropterygii,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Apodes {
    Muraena,
    Gymnotus,
    Anarchicas,
    Trichiurus,
    Ammodytes,
    Ophidium,
    Stromateus,
    Xiphias,
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Jugulares {
    Callionymus,
    Uranoscopus,
    Trachinus,
    Gadus,
    Blennius,
    Kurtus,
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Thoracici {
    Cepola,
    Echeneis,
    Coryphaena,
    Gobius,
    Cottus,
    Scorphaena,
    Zeus,
    Pleuronectes,
    Chaetodon,
    Sparus,
    Scarus,
    Labrus,
    Sciaena,
    Perca,
    Gasterosteus,
    Scomber,
    Centrogaster,
    Mullus,
    Trigla,
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Abdominales {
    Cobitis,
    Amia,
    Silurus,
    Teuthis,
    Loricaria,
    Salmo,
    Fistularia,
    Esox,
    Elops,
    Argentina,
    Atherina,
    Mugil,
    Exocoetus,
    Polynemus,
    Clupea,
    Cyprinus,
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Branchiostegi {
    Mormyrus,
    Ostracion,
    Tetrodon,
    Diodon,
    Syngnathus,
    Pegasus,
    Centriscus,
    Balistes,
    Cyclopterus,
    Lophius,
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Chondropterygii {
    Acipenser,
    Chimaera,
    Squalus,
    Raia,
    Petromyzon,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Amphibia {
    Reptilia,
    Serpentes,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Reptilia {
    Testudo,
    Draco,
    Lacerta,
    Rana,
    Crocodilus,
    Alligator,
    Dracaena,
    Iguana,
    Salamander,
    Gecko,
    Chamaeleon,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Serpentes {
    Crotalus,
    Boa,
    Constrictor,
    Vipera,
    Scytha,
    Cobra,
    Aspis,

    Coluber,
    Anguis,
    Amphisbaena,
    Caecilia,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Aves {
    Accipitres, //hawk
    Picae,      //magpie
    Anseres,    //gooese
    Grallae,    //stilt like flamingo
    Gallinae,   //chicken
    Passeres,   //songbird
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Grallae {
    Phoenicopterus, //flamingo
    Platalea,
    Palamedea,
    Mycteria,
    Tantalus,
    Ardea,
    Recurvirostra,
    Scolopax,
    Tringa,
    Fulica,
    Parra,
    Rallus,
    Psophia,
    Cancroma,
    Haematopus,
    Charadricus,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Gallinae {
    Otis,
    Struthio,
    Didus,
    Pavo,
    Meleagris,
    Penelope,
    Crax,
    Phasianus,
    Numida,
    Tetrao,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Passeres {
    Loxia,
    Colius,
    Fringilla,
    Phytotoma,
    Emberiza,
    Caprimulgus,
    Hirundo,
    Pipra,
    Ampelis,
    Tanagra,
    Muscicapa,
    Parus,
    Motacilla,
    Alauda,
    Sturnus,
    Columba,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Anseres {
    Anas,
    Mergus,
    Phaeton,
    Plotus,
    Rhyncops,
    Diomedea,
    Aptenodyta,
    Alca,
    Procellaria,
    Pelecanus,
    Larus,
    Sterna,
    Colymbus,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Picae {
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
    Ramphastos,
    Trogon,
    Psittacus,
    Crotophaga,
    Picus,
    Yunx,
    Cuculus,
    Bucco,
    Buceros,
    Alcedo,
    Merops,
    Todus,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Accipitres {
    Falco,
    Vultur,
    Strix,
    Lanius,
    Gryphus,
    Harpyia,
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
