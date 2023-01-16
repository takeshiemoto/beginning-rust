pub fn run() {
    // 可変な配列
    let mut ary1 = ["Rust", "Go", "TypeScript"];
    // 同じ要素数であれば配列の要素を書き換えられる
    ary1 = ["Java", "Python", "Scala"];
    // 型は変えられない
    // ary1 = [1, 2, 3]; // Error

    println!("{:?}", ary1)
}
