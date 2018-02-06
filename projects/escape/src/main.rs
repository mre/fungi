// https://twitter.com/Argorak/status/940221231709683713
// https://github.com/itafroma/zork-mdl
// https://github.com/olson-dan/rustzork
// http://i7-dungeon.sourceforge.net/source_3.html
// https://dev.to/mindflavor/lets-build-zork-using-rust-1opm

use std::ops::Deref;

/// This function takes in account the mutable reference to its associated type
/// and returns the next GameState.
type ChangeStateFn<T> = fn(&mut T) -> GameState<T>;

/// A game state is a container for the "next" function and for the switches
/// that controls the input (required or not) and the completion (reached or
/// not).
struct GameState<T> {
    next: ChangeStateFn<T>,
    requires_input: bool,
    completed: bool,
    name: String,
}

impl<T> GameState<T> {
    fn new(
        next: ChangeStateFn<T>,
        requires_input: bool,
        completed: bool,
        name: String,
    ) -> GameState<T> {
        GameState {
            next: next,
            requires_input: requires_input,
            completed: completed,
            name: name,
        }
    }

    /// Returns an uncomplete GameState that requires the user input and with
    /// the given "next" function.
    fn with_input(next: ChangeStateFn<T>, name: String) -> GameState<T> {
        GameState::new(next, true, false, name)
    }

    /// Returns a uncomplete GameState that does not require the user input and
    /// with the given "next" function.
    fn without_input(next: ChangeStateFn<T>, name: String) -> GameState<T> {
        GameState::new(next, false, false, name)
    }

    /// Retuns a completed GameState.
    fn completed(function: ChangeStateFn<T>) -> GameState<T> {
        GameState::new(function, false, true, String::from("completed"))
    }
}

impl<T> Deref for GameState<T> {
    type Target = ChangeStateFn<T>;

    fn deref(&self) -> &Self::Target {
        &self.next
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    has_key: bool,
}

#[derive(Debug)]
struct Game {
    player: Player,
    last_command: String,
    door_locked: bool,
}
