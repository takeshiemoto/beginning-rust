// クロージャーを受け取る関数
// 1. クロージャはコンパイル時にサイズがわからないので、引数として渡すことができない
// 2. そうなるとクロージャーをコード上で型を指定できない...
// 3. 代わりにFnOnece, FnMut, Fnのいずれかのトレイトを指定することで、コンパイル時にサイズがわからないクロージャを引数として渡すことができる

// FnOnceを受け取る関数
// 所有権を奪うため一度しか実行できない = Onece
fn call_with_one<F>(some_closure: F) -> i32
where
    F: FnOnce(i32) -> i32,
{
    some_closure(1)
}

// FnMutを受け取る関数
fn call_with_one_v2<F>(some_closure: &mut F) -> i32
where
    F: FnMut(i32) -> i32,
{
    some_closure(1)
}

// Fnを受け取る関数
fn call_with_one_v3<F>(some_closure: &F) -> i32
where
    F: Fn(i32) -> i32,
{
    some_closure(1)
}

// クロージャ自由変数の扱いでどのトレイトのインスタンスかが決まる
// 三つとは、Fn、FnMut、FnOneceである
