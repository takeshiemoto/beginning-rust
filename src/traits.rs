// 複数の型に対して共通の振る舞いを持たせたい

use csv::ReaderBuilder;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::PathBuf;

trait Fruits {
    fn price(&self) -> u32;
}

struct Apple;

impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;

impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    // デフォルト実装
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}

struct NewArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewArticle {
    // オーバーライド
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

impl Message for NewArticle {}

struct Tweet {
    username: String,
    content: String,
    replay: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

pub trait Calculator {
    fn calc(&self) -> anyhow::Result<u64>;
}

pub struct Rectangle {
    width: u64,
    height: u64,
}

impl Calculator for Rectangle {
    fn calc(&self) -> anyhow::Result<u64> {
        Ok(self.height * self.width)
    }
}

fn use_rectangle() {
    let r = Rectangle {
        width: 100,
        height: 50,
    };
    let result = r.calc();
    println!("面積 = {}", result.unwrap());
}

pub fn run() {
    use_rectangle();

    let apple = Apple {};
    let banana = Banana {};

    get_price(apple);
    get_price(banana);

    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        replay: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewArticle {
        headline: "Penguins win the Stanley Cup Championship!".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "iceburgh".to_string(),
        content: "The pittsburgh penguins once again".to_string(),
    };

    println!("{}", article.summarize());

    notify(&article);
    notify_another(&article);
}

// トレイト境界
fn get_price<T: Fruits>(fruits: T) {
    println!("price is: {}", fruits.price());
}

// Summaryトレイトを実装しているデータ型であれば渡すことができる
// そのデータの参照を渡すことができる
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// SummaryとMessageを実装しているトレイトの参照を受け取れる
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message {}", item.message());
}

pub trait CsvReader<T>
where
    T: DeserializeOwned,
{
    /// ## 読み取り
    /// ### 引数 file_path:ファイルパス
    /// ### 戻り値 Result<Vec<T>> 読み取り結果
    fn read(&self, file_path: &str) -> anyhow::Result<Vec<T>>;
}

pub trait JsonReader<T>
where
    T: DeserializeOwned,
{
    /// ## 読み取り
    /// ### 引数 file_path:ファイルパス
    /// ### 戻り値 Result<Vec<T>> 読み取り結果
    fn read(&self, file_path: &str) -> anyhow::Result<Vec<T>>;
}

#[derive(Default)]
pub struct CsvReaderImpl<T> {
    phantom: PhantomData<T>,
}
impl<T> CsvReader<T> for CsvReaderImpl<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> anyhow::Result<Vec<T>> {
        let path_buf = PathBuf::from(file_path);
        let string_data = read_to_string(path_buf)?;
        let mut reader = ReaderBuilder::new()
            .delimiter(b',')
            .from_reader(string_data.as_bytes());
        let rows = reader.deserialize::<T>();
        let mut result = Vec::<T>::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }
}

#[derive(Default)]
pub struct JsonReaderImpl<T> {
    phantom: PhantomData<T>,
}
impl<T> JsonReader<T> for JsonReaderImpl<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> anyhow::Result<Vec<T>> {
        let path_buf = PathBuf::from(file_path);
        let buf_reader = File::open(path_buf).map(|file| BufReader::new(file))?;
        let result = serde_json::from_reader(buf_reader)?;
        Ok(result)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: u32,
}

#[allow(dead_code)]
pub fn use_generics_method() {
    let csv_path = concat!(env!("CARGO_MANIFEST_DIR"), "product.csv");
    let json_path = concat!(env!("CARGO_MANIFEST_DIR"), "product.json");

    let csv_method = CsvReaderImpl::<Product>::default();
    let json_method = JsonReaderImpl::<Product>::default();

    let csv_result = csv_method.read(csv_path).unwrap();
    let json_result = json_method.read(json_path).unwrap();

    for result in csv_result {
        println!("{:?}", result);
    }

    for result in json_result {
        println!("{:?}", result);
    }
}
