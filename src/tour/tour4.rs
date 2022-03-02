struct Foo<T> {
    item: T,
}

// RustにはnullがないのでNoneを使うのが一般的
enum Item {
    Inventory(String),
    None,
}

struct Bag {
    item: Item,
}

pub fn tour_generic() {
    let foo1 = Foo::<i32> { item: 1 }; // 型宣言
    let foo2 = Foo { item: 1u8 }; // 型推論
    let x = foo1.item + foo2.item as i32;
    println!("{}", x);

    let item1 = Item::Inventory(String::from("やくそう"));
    let bag = Bag { item: item1 };
    match bag.item {
        Item::Inventory(name) => println!("{}", name),
        Item::None => println!("none"),
    }

    println!("Option");

    let bag1 = Bag2::<u32> { item: None };
    let bag2 = Bag2 { item: Some(12) };
    log_bag2(bag1);
    log_bag2(bag2);

    println!("");
    println!("Result");
    println!("");

    let input = 3;
    let result = do_something(input);
    match result {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }

    println!("");
    println!("簡潔なエラー処理");
    println!("");
    simple_error_handling();

    println!("");
    println!("ベクタ型");
    println!("");
    tour_vector();

}

fn log_bag2(bag: Bag2<u32>) {
    match bag.item {
        Some(num) => println!("バッグには{}つのものが入っている", num),
        None => println!("バッグには何も入っていない"),
    }

    // is_none()とis_some()もある
    if bag.item.is_none() {
    } else {
    }
}

// Option
struct Bag2<T> {
    item: Option<T>,
}

// enum Result<T, R> {
//     Ok(T),
//     Err(E),
// }

fn do_something(i: i32) -> Result<i32, String> {
    if i % 2 == 0 {
        Ok(i)
    } else {
        Err(String::from("入力が偶数ではありません"))
    }
}

// Ok(())かErr(String)を返すメソッド
fn simple_error_handling() -> Result<(), String> {
    let input = 3;

    let v = match do_something(input) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    // 上は下と同じ（シュッとした書き方）

    let v = do_something(input)?;
    println!("{}", v);
    Ok(())
}

// ベクタ型
fn tour_vector() {
    // mutableである必要がある
    let mut my_vec = Vec::new();
    my_vec.push(1);
    my_vec.push(2);

    for num in my_vec.iter(){
        println!("{}", num);
    }

    // vec!マクロ
    let my_vec2 = vec![1, 2, 3];

}
