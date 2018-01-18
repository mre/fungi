pub fn sample() {
    let data = "initial contents";

    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
    let mut s = String::from("foo");
    s.push_str("bar");

    // The push_str method takes a string slice because we don’t necessarily
    // want to take ownership of the parameter.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);

    // This code also sets s to tic-tac-toe. The format! macro works in the same
    // way as println!, but instead of printing the output to the screen, it
    // returns a String with the contents. The version of the code using format!
    // is much easier to read and also doesn’t take ownership of any of its
    // parameters.

    let hello = "Здравствуйте";

    let _s = &hello[0..4];
    // Here, s will be a &str that contains the first four bytes of the string.
    // Earlier, we mentioned that each of these characters was two bytes, which
    // means s will be Зд.

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
