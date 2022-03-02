pub fn variable() {
    let x = 1;
    println!("{x}");
    let mut y = 2;
    println!("{}", y);
    y = 3;
    println!("{}", y)
}

pub fn types() {
    let my_bool: bool = true;

    // 整数
    // 符号なし
    let a: u8 = 255; // 0 ~ 255
    let b: u16 = 0;
    let c: u32 = 4;
    let d: u64 = 6;
    let e: u128 = 2;

    // 符号付き
    let x = -1i8; // 後ろにつけて型宣言もできる
    let y: i16 = -2;
    let z = 1; // デフォルトは i32

    // ポインタサイズ整数型
    // 配列のインデックスとかに使うやつっぽい
    let var_usize: usize = 0; // よくわかってない
    let var_isize: isize = -1; // よくわかってない２

    // 浮動小数点
    let my_float = 1.0; // デフォルトでf64
    let my_float2 = 1.0f32;

    // タプル
    let my_tuple = (a, x); // tuple

    // 文字列
    let tex: &str = "aaa"; //&str とは?
}

const PI: f32 = 3.141592; // private 定数
pub const FOO: u8 = 5; // pub定数

pub fn tour_array() {
    let nums = [1, 3, 5]; // [i32; 3]
    let nums2: [i8; 3] = [1, 3, 5]; // 明示的に型を宣言
    let x = nums2[1];
    let index = 1 as usize;
    let y = nums[index]; // 配列のインデックスはusize
}

pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
    // return x + y
    // x + y
}

// tuple
pub fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

pub fn tour_function() {
    let a = add(1, 2);
    print!("{}", a);
    let (b, c) = swap(10, 20);
    print!("{}, {}", b, c);
}

pub fn make_nothing() {
    return ();
}
pub fn make_nothing2() {}

pub fn tour_nothing() {
    let a = make_nothing();
    let b = make_nothing2();
    println!("make_nothing:{:?}", a);
    println!("make_nothing2:{:?}", b);
}

pub fn tour_if(x: i32) {
    if x < 42 {
        println!("42より小さい")
    } else if x == 42 {
        println!("42に等しい")
    } else {
        println!("42より大きい")
    }
}

pub fn tour_loop(){
    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }

    print!("{}", x)
}

pub fn tour_for(){
    for x in 0..5{
        println!("{x}")
    }
}

pub fn tour_match(x:i32){
    match x {
        0 => println!("0"),
        1 => println!("1"),
        2 => println!("2"),
        3..=4 => {println!("3..4")}
        _ => {println!("others")}
    }
}

pub fn tour_match2() -> i32{
    let x = 42;

    // Rust の三項式
    let v = if x < 42 { -1 } else { 1 };
    println!("if より: {}", v);

    let food = "ハンバーガー";
    let result = match food {
        "ホットドッグ" => "ホットドッグです",
        // 単一の式で値を返す場合、中括弧は省略可能
        _ => "ホットドッグではありません",
    };
    println!("食品の識別: {}", result);

    let v = {
        // ブロックのスコープは関数のスコープから分離されている
        let a = 1;
        let b = 2;
        a + b
    };
    println!("ブロックより: {}", v);

    // Rust で関数の最後から値を返す慣用的な方法
    v + 4
}

