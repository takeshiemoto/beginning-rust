pub fn fizz_buzz_for_pattern_match(value: i32) -> String {
    match value {
        // vという変数名をvalueにバインドしている
        v if v % 15 == 0 => "fizz buzz".to_string(),
        v if v % 5 == 0 => "buzz".to_string(),
        v if v % 3 == 0 => "fizz".to_string(),
        _ => value.to_string(),
    }
}

pub fn run_if_let() {
    let some_option = Some(10);

    // パターンマッチにて一つの結果にのみ関心がある場合はif letが良い
    // すべてのケースを網羅する場合はmatchを使う
    if let Some(value) = some_option {
        println!("value is {}", value)
    }
}

fn hello_print(message: String) {
    println!("hello {}", message);
}

fn hello_print_v2(message: &str) {
    println!("hello {}", message);
}

pub fn run() {
    let world = String::from("world");
    // worldの所有権が奪われる
    hello_print(world);

    // 所有権が無いのでエラーになる
    // borrow of moved value: `world` [E0382]
    // println!("{}", world);

    let world_2 = String::from("world");
    // 参照のみを渡す
    // &Stringは内部的に&strに変換される
    hello_print_v2(&world_2);
    // world_2の所有権は奪われていないため利用可能である
    // これを借用と呼ぶ
    println!("{}", world_2);
}
