fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn run() {
    println!("add one result v1 {:?}", add_one_v1(1));

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    println!("add one result v2 {:?}", add_one_v2(1));

    // 関数と違い、型を明示しなくてもコンパイラが型を推論してくれる
    // v3はi32で推論される。一度型を推論したら、その型を変更することはできない
    let add_one_v3 = |x| x + 1;
    println!("add one result v3 {:?}", add_one_v3(1));

    // 環境をキャプチャするとは？
    // 環境 = 外部の変数や関数
    // キャプチャ = クロージャ内で使えるようにすること
    let x = 4;
    let y = 4;

    // xをキャプチャしている
    // xは自由変数と呼ばれる
    let equal_to_x = |z| z == x;
    println!("equal to x {:?}", equal_to_x(y));

    // クロージャでは無く関数の場合は外部の変数を使うことができない
    // Can't capture dynamic environment in a fn item [E0434]
    // fn equal_to_x_v2(z: i32) -> bool {
    //     z == x
    // }

    // クロージャは参照を利用している
    // moveキーワードを使うと、クロージャは環境をキャプチャするのではなく所有権を奪うことができる。
    let x2 = vec![1, 2, 3];
    let y2 = vec![1, 2, 3];

    let equal_to_x2 = move |z| z == x2;
    println!("equal to x2 {:?}", equal_to_x2(y2));

    // Error
    // キャプチャしたx2はmoveしているのでこの時点で所有権はequal_to_x2に移っている
    // println!("{:?}", x2)
}
