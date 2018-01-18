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

pub fn sample() {
    one();
    two();
    bar();
}
