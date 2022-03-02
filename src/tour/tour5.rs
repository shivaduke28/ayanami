struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("f.x={}", f.x);
}

fn incre_foo(f: &mut Foo){
    f.x += 1;
    println!("incre: {}", f.x);
}

pub fn main() {
    println!("\n借用\n");
    {
        let f = Foo { x: 1 };
        // fooの所有権がdo_somethingにmoveした
        do_something(f);
        // コンパイルエラー（所有権をmoveしたので使えない）
        // println!("{}", f.x);

        let f2 = Foo { x: 2 };
        let f3 = &f2;

        // コンパイルエラー（f2はf3に借用されてる）
        // do_something(f2);

        // コンパイルエラー（型がミスマッチ：Fooと&Fooは別物）
        //do_something(f3);
        println!("{}", f3.x);
        // ここでf3がドロップ

        // f3がドロップしたので借用が消えて使えるようになる
        do_something(f2);
    }
    println!("\n &mut \n");
    {
        let mut foo = Foo { x: 42 };
        let f = &mut foo; // 可変な借用

        println!("f.x={}", f.x);
        f.x = 10;
        println!("f.x={}", f.x);
        // fドロップ

        do_something(foo); // 10になる！
    }
    println!("\n 参照外し \n");
    {
        let mut foo = 42;
        let f = &mut foo;
        let bar = *f; // 参照が外れるっぽい
        *f = 13;
        println!("{}", bar);
        println!("{}", foo);
    }
    println!("\n借用の受け渡し\n");
    {
        let mut foo = Foo { x: 42};

        // 不変な参照は複数存在できる
        let f = & foo;
        let g = & foo;
        println!("{}", f.x); // 42
        println!("{}", g.x); // 42
        // fとgはここでドロップ

        // &mut は同時にひとつか存在できない
        incre_foo(&mut foo); // incre_fooの終わりで&mutはドロップ
        incre_foo(&mut foo);
        println!("{}", foo.x); // 44

        // println!("{}", f.x); // これを書くと &mutがコンパイルエラー（可変と不変は共存できない）
    }
    println!("\nスタティックライフタイム\n");
    {
        static SECRET :&str = "aaa";
    }
}
