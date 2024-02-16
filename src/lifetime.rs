// <'a>でライフタイムアノテーション'aを定義
// x、yは同じライフタイム'aを共有する
// この関数が返す参照も同じライフタイム'aを持つ
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // xの長さがyの長さよりも大きか
    if x.len() > y.len() {
        // xが長ければxの参照を返す
        x
    } else {
        // yが長ければxの参照を返す
        y
    }
}

// 'aライフタイムを持つpartというフィールドを持っています。
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn run() {
    let novel = String::from("こんにちは. 今日の天気はいかがでしょう");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    // excerpt.partはfirst_sentenceの参照を保持
    println!("Important part: {}", excerpt.part);
}

// この関数は、2つの文字列スライス`str1`と`str2`を受け取り、常に最初の文字列スライス`str1`を返します。
// ライフタイムアノテーション`'a`は、`str1`と`str2`が同じライフタイムを持ち、
// この関数が返す参照も同じライフタイムを持つことを示しています。
fn select_first<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    str1
}

fn run2() {
    let word1 = "Hello";
    let word2 = "world";
    let result = select_first(word1, word2);
    println!("The first string is: {}", result);
}

fn run3() {
    // 有効な参照
    let s1 = String::from("hello");
    // s1の参照を渡す
    let len = calculate_length(&s1);

    println!("The length if '{}' is {}.", s1, len);
}

// 文字列スライスの参照を受け取り、長さを返す
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn run4() {
    let string1 = String::from("ling string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // ここでresultを使用しようとすると、string2がもう存在しないため、
    // 無効な参照にアクセスすることになり、コンパイルエラーが発生します。
    // println!("The longest string is {}", result); // コンパイルエラー: 借用された値がここでドロップされます
}
