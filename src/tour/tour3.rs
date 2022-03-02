struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

// "crab"は読み取り専用なのでデータメモリに入る

// String::from("crab");
// String構造体。関数の中で呼び出されたのでスタックに入る
// String構造体は文字列を変更できるようになっている
// ヒープにテキストを入れる
// テキストへの参照アドレスをヒープに保存して、String構造体に入れる

fn tour_memory() {
    // SeaCreatureのデータはスタックに入ります。
    let ferris = SeaCreature {
        // String構造体もスタックに入りますが、
        // ヒープに入るデータの参照アドレスが一つ入ります。
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("none"),
    };

    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );
}

// tupleのような構造体
struct Location(i32, i32);
// ()の別名称
struct Marker;

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}

struct Creature {
    species: Species,
}

pub fn tour_enum() {
    let mut c = Creature {
        species: Species::Crab,
    };
    match c.species {
        Species::Crab => println!("crab"),
        _ => println!("not crab"),
    }
    c.species = Species::Octopus;
    match c.species {
        Species::Crab => println!("crab"),
        _ => println!("not crab"),
    }
}


pub fn tour_enum2(){
    let x = MyEnum::Foo(1);
    let y = MyEnum::Foo;
    let z = y(12);
    let w = MyEnum::Buzz;

    match x{
        MyEnum::Foo(a) => println!("{a}"),
        _ => println!("not Foo")
    }
}

enum MyEnum {
    Foo (i32),
    Bar (i8, i8),
    Buzz,
}
