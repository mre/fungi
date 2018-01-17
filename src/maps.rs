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

fn sample() {
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
