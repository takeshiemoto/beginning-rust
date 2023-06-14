pub fn run() {
    let v = vec![1, 2, 3, 4, 5];
    let v_iter = v.iter();

    for val in v_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_demonstration() {
    let v = vec![1, 2, 3];
    let mut v1_iter = v.iter();

    // Nextで値を取り出す
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    // 最後の要素を取り出した後はNoneを返す
    assert_eq!(v1_iter.next(), None);
}

struct City {
    name: String,
    population: i64,
    country: String,
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

// fn sort_cities(cities: &mut Vec<City>) {
//     cities.sort_by_key(city_population_descending)
// }

// クロージャ版
fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population)
}
