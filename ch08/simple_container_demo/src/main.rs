use std::cmp::Ordering;
use std::collections::HashMap;

struct Foo;

fn pick(arr: [i32; 3]) {
    match arr {
        [_, _, 3] => println!("ends with 3"),
        [a, 2, c] => println!("{:?}, 2, {:?}", a, c),
        [_, _, _] => println!("pass!"),
    }
}

fn sum(num: &[i32]) {
    match num {
        [one] => println!(" at least two"),
        [first, second] => println!("{:?} + {:?} = {:?}", first, second, first + second),
        _ => println!("sum is {:?}", num.iter().fold(0, |sum, i| sum + i)),
    }
}

fn merge_extend<'a>(map1: &mut HashMap<&'a str, &'a str>, map2: HashMap<&'a str, &'a str>) {
    map1.extend(map2);
}
fn merge_chain<'a>(
    map1: HashMap<&'a str, &'a str>,
    map2: HashMap<&'a str, &'a str>,
) -> HashMap<&'a str, &'a str> {
    map1.into_iter().chain(map2).collect()
}
fn merge_by_ref<'a>(map: &mut HashMap<&'a str, &'a str>, map_ref: &HashMap<&'a str, &'a str>) {
    map.extend(map_ref.into_iter().map(|(k, v)| (k.clone(), v.clone())));
}

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
    vec[0] = 7;
    assert_eq!(vec[0], 7);
    assert_eq!(vec.get(0), Some(&7));
    assert_eq!(vec.get(10), None);
    vec.extend([1, 2, 3].iter().cloned());
    assert_eq!(vec, [7, 1, 2, 3]);
    assert_eq!(vec.get(0..2), Some(&[7, 1][..]));
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec, [7, 1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);
    vec.swap(1, 3);
    assert!(vec == [7, 3, 2, 1, 4, 5, 6]);
    let slice = [1, 2, 3, 4, 5, 6, 7];
    vec.copy_from_slice(&slice);
    assert_eq!(vec, slice);
    let slice = [4, 3, 2, 1];
    // vec.clone_from_slice(&slice);
    // assert_eq!(vec, slice);

    let mut vec = Vec::with_capacity(10);
    for i in 0..10 {
        vec.push(i);
    }
    vec.truncate(0);
    assert_eq!(10, vec.capacity());
    for i in 0..10 {
        vec.push(i);
    }
    vec.clear();
    assert_eq!(10, vec.capacity());
    vec.shrink_to_fit();
    assert_eq!(0, vec.capacity());
    for i in 0..10 {
        vec.push(i);
        println!("{:?}", vec.capacity());
    }

    let mut vec = Vec::new();
    vec.push(Foo);
    assert_eq!(vec.capacity(), std::usize::MAX);

    let v = [10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&50));
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(v.ends_with(&[30]));
    assert!(v.ends_with(&[40, 30]));
    assert!(v.ends_with(&[]));
    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));
    assert!(v.ends_with(&[]));

    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    assert_eq!(s.binary_search(&13), Ok(9));
    assert_eq!(s.binary_search(&4), Err(7));
    let r = s.binary_search(&1);
    assert!(match r {
        Ok(1..=4) => true,
        _ => false,
    });
    let seek = 13;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
    let s = [
        (0, 0),
        (2, 1),
        (4, 1),
        (5, 1),
        (3, 1),
        (1, 2),
        (2, 3),
        (4, 5),
        (5, 8),
        (3, 13),
        (1, 21),
        (2, 34),
        (4, 55),
    ];
    assert_eq!(s.binary_search_by_key(&13, |&(a, b)| b), Ok(9));

    let mut v = [-5i32, 4, 1, -3, 2];
    v.sort();
    assert!(v == [-5, -3, 1, 2, 4]);
    v.sort_by(|a, b| a.cmp(b));
    assert!(v == [-5, -3, 1, 2, 4]);
    v.sort_by(|a, b| b.cmp(a));
    assert!(v == [4, 2, 1, -3, -5]);
    v.sort_by_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);

    let result = 1.0.partial_cmp(&2.0);
    assert_eq!(result, Some(Ordering::Less));
    let result = 1.cmp(&1);
    assert_eq!(result, Ordering::Equal);
    let result = "abc".partial_cmp(&"Abc");
    assert_eq!(result, Some(Ordering::Greater));
    let mut v: [f32; 5] = [5.0, 4.1, 1.2, 3.4, 2.5];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(v == [1.2, 2.5, 3.4, 4.1, 5.0]);
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    assert!(v == [5.0, 4.1, 3.4, 2.5, 1.2]);

    let arr = [1, 2, 3];
    pick(arr);
    let arr = [1, 2, 5];
    pick(arr);
    let arr = [1, 3, 3];
    pick(arr);

    sum(&[1]);
    sum(&[1, 2]);
    sum(&[1, 2, 3]);
    sum(&[1, 2, 3, 4]);
    sum(&[1, 2, 3, 4, 5]);

    let mut book_reviews = HashMap::with_capacity(10);
    book_reviews.insert("Rust Book", "good");
    book_reviews.insert("Programming Rust", "nice");
    book_reviews.insert("The Tao of Rust", "deep");
    for key in book_reviews.keys() {
        println!("{}", key);
    }
    for val in book_reviews.values() {
        println!("{}", val);
    }
    if !book_reviews.contains_key("rust book") {
        println!("find {} times ", book_reviews.len());
    }
    book_reviews.remove("Rust Book");
    let to_find = ["Rust Book", "The Tao of Rust"];
    for book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book),
        }
    }
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }
    assert_eq!(book_reviews["The Tao of Rust"], "deep");

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("current_year").or_insert(2017);
    assert_eq!(map["current_year"], 2017);
    *map.entry("current_year").or_insert(2017) += 10;
    assert_eq!(map["current_year"], 2027);
    let last_leap_year = 2016;
    map.entry("next_leap_year")
        .or_insert_with(|| last_leap_year + 4);
    assert_eq!(map["next_leap_year"], 2020);
    assert_eq!(map.entry("current_year").key(), &"current_year");

    let mut book_reviews1 = HashMap::new();
    book_reviews1.insert("Rust Book", "good");
    book_reviews1.insert("Programming Rust", "nice");
    book_reviews1.insert("The Tao of Rust", "deep");
    let mut book_reviews2 = HashMap::new();
    book_reviews2.insert("Rust in Action", "good");
    book_reviews2.insert("Rust Primer", "nice");
    book_reviews2.insert("Matering Rust", "deep");
    // merge_extend(&mut book_reviews1, book_reviews2);
    // let book_review1 = merge_chain(book_reviews1, book_reviews2);
    merge_by_ref(&mut book_reviews1, &book_reviews2);
    for key in book_reviews1.keys() {
        println!("{}", key);
    }
}
