use noise::{NoiseFn, Perlin, Seedable};
use slov_common::*;

fn main() {
    let kost = "kost";
    let dom = "dom";
    let kostik = "kost́";

    println!(" kost is {:#?}", ISV::guess_gender(kost));
    println!(" dom is {:#?}", ISV::guess_gender(dom));
    println!(" kost́ is {:#?}", ISV::guess_gender(kostik));
    println!("{}", "Žuravina".to_lowercase());
    println!("los is animate? - {:#?}", ISV::noun_is_animate("los"));
    println!(
        "jablanj is animate? - {:#?}",
        ISV::noun_is_animate("jablanj")
    );
    println!("{}", ISV::acc_sg("dom"));
    println!("{}", ISV::acc_sg("deva"));
    println!("{}", ISV::acc_sg("masina"));

    println!("{}", ISV::acc_sg("jelenj"));

    println!("{}", ISV::dat_sg("dom"));
    println!("{}", ISV::dat_sg("deva"));
    println!("{}", ISV::dat_sg("pivo"));
    println!("{}", ISV::dat_sg("masina"));

    println!("{}", ISV::dat_sg("jelenj"));

    println!("{}", ISV::ins_sg("dom"));
    println!("{}", ISV::ins_sg("deva"));
    println!("{}", ISV::ins_sg("pivo"));
    println!("{}", ISV::ins_sg("masina"));

    println!("{}", ISV::ins_sg("jelenj"));

   let x = load_csv_data();

   println!("{:?}",x);

    /*

      let boop: MyPoint = (5, 5);
    let nw = MyWorld::default();
    let swordik = MeleeWeaponType::Sěkyra;
    let myitem = Item {
        item_type: ItemType::Melee(MeleeWeapon {
            weapon_type: MeleeWeaponType::Kopje,
            material_type: Material::Metal(MetalType::Bronza),
        }),
    };

    //  let ne = EntityType::Item(myitem);

    println!("{:#?}", boop);
    println!("{:#?}", nw);
    println!("swordik is {}", swordik);
    println!("myitem is is {}", myitem.to_char());

    let perlin = Perlin::new(1);
    let val = perlin.get([41111.0, 379999.7]);
    println!("val is {}", val);
    //  println!("typik is {}", typik);

     */
}
