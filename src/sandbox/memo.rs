pub fn if_let() {
    let r = Option::Some(1);

    // match式のアームが1つの場合
    match r {
        Some(x) => println!("{}", x),
        _ => (),
    }

    // if letで短く書ける
    if let Some(x) = r {
        println!("{}", x);
    }
}

// メソッド
// structを定義する
struct MyStruct {
    num: i32,
}

// メソッドを定義する
// メソッドの引数はself, &self, &mut selfのいずれか
// 一般的には &mut にする
// &mut self にすると、可変コピーにのみ使えるメソッドになる
// self にすると、所有権を持ってしまうのでそれ以降誰も使えなくなる（unwarpとか）
impl MyStruct {
    fn m1(&self) {
        println!("{}", self.num);
    }
    fn m2(&self, x: i32) {
        println!("{}", self.num + x);
    }
    // 関連関数（structに対するstaticメソッド？）
    fn new() -> Self {
        return Self { num: 1 };
    }
}

pub fn test_method() {
    let s = MyStruct { num: 12 };
    s.m1();
    s.m2(100);

    let t = MyStruct::new();
    t.m2(100);
}

use std::fmt;

// タプル構造体でStringのラッパーを作る
struct Password(String);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.chars().map(|_| '*').collect::<String>())
    }
}

pub fn test_password() {
    let a = String::from("my password");
    let p = Password(a);
    println!("{}", p);
}
