pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // a method named value that borrows self, doesn’t have any other
    // parameters, and returns a u32.
    // This public method is necessary because the value field of the Guess
    // struct is private. It’s important that the value field is private so code
    // using the Guess struct is not allowed to set value directly: code outside
    // the module must use the Guess::new function to create an instance of
    // Guess, which ensures there’s no way for a Guess to have a value that
    // hasn’t been checked by the conditions in the Guess::new function.
    pub fn value(&self) -> u32 {
        self.value
    }
}
