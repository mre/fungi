// https://doc.rust-lang.org/stable/book/second-edition/ch08-03-hash-maps.html

use std::collections::btree_map::BTreeMap;

// https://doc.rust-lang.org/stable/std/collections/
fn bar() {
    // A client of the bar. They have a blood alcohol level.
    struct Person {
        blood_alcohol: f32,
    }

    // All the orders made to the bar, by client id.
    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

    // Our clients.
    let mut blood_alcohol = BTreeMap::new();

    for id in orders {
        // If this is the first time we've seen this customer, initialize them
        // with no blood alcohol. Otherwise, just retrieve them.
        let person = blood_alcohol
            .entry(id)
            .or_insert(Person { blood_alcohol: 0.0 });

        // Reduce their blood alcohol level. It takes time to order and drink a beer!
        person.blood_alcohol *= 0.9;

        // Check if they're sober enough to have another beer.
        if person.blood_alcohol > 0.3 {
            // Too drunk... for now.
            println!("Sorry {}, I have to cut you off", id);
        } else {
            // Have another!
            person.blood_alcohol += 0.1;
        }
    }
}

fn one() {
    let mut count = BTreeMap::new();
    let message = "she sells sea shells by the sea shore";

    for c in message.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    assert_eq!(count.get(&'s'), Some(&8));

    println!("Number of occurrences of each character");
    for (char, count) in &count {
        println!("{}: {}", char, count);
    }
}

fn two() {
    use std::collections::HashMap;

    // Just like vectors, hash maps store their data on the heap. This HashMap
    // has keys of type String and values of type i32. Like vectors, hash maps
    // are homogeneous: all of the keys must have the same type, and all of the
    // values must have the same type.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map. For owned values like String, the values will be moved
    // and the hash map will be the owner of those values.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn three() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // It’s common to check whether a particular key has a value, and if it
    // doesn’t, insert a value for it. Hash maps have a special API for this
    // called entry that takes the key we want to check as a parameter. The
    // return value of the entry function is an enum called Entry that
    // represents a value that might or might not exist.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

// By default, HashMap uses a cryptographically secure hashing function that can
// provide resistance to Denial of Service (DoS) attacks. This is not the
// fastest hashing algorithm available, but the trade-off for better security
// that comes with the drop in performance is worth it. If you profile your code
// and find that the default hash function is too slow for your purposes, you
// can switch to another function by specifying a different hasher. A hasher is
// a type that implements the BuildHasher trait.
pub fn sample() {
    one();
    two();
    three();
    bar();
}
