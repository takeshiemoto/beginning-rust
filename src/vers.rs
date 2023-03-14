// 定数は型を指定する
// 変数はどのスコープでも定義できる
const MAX_POINTS: u32 = 100_000;

pub fn run() {
    // let x = 5;
    // イミュータブルな変数は変更できない
    // x = 6

    let mut x = 5;
    x = 6;
    println!("{}", x);

    // 型推論でi32が設定される
    // デフォルトで符号付き32ビット
    let i1 = 3;
    println!("{}", i1);

    // 小数点はf64に推論される
    // 使用しない場合はアンダースコアを付ける
    let _f1 = 0.1;
    println!("{}", usize::BITS);

    // メモリのアドレスを表示させる
    // 16進数の表記で表示される
    // const定義は静的領域に格納される
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    // 8バイトのサイズを持つ
    // データサイズが決まっている値はスタックに積まれる
    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2); // 8byte = 64bit
    println!("Stack address of i3 is: {:p}", &i3); // 8byte = 64bit

    let y = 5; // 32bit = 4byte
    println!("Stack address of y is: {:p}", &y);
    // shadowing
    // 同じスコープで再バインドできる
    // シャドーイングされたらポインタの値は変わる
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);
    {
        // 別のスコープ
        let y = 0;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);

    // タプル
    // リテラル = 文字列スライス = &strで表す
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    // 取り出す
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    // 参照外しで値を更新する
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);
}
