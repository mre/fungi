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

use std::fmt;

impl fmt::Debug for GameState<Game> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameState: {}", self.name)
    }
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

// The standard library provides a special trait, Deref. It’s normally used to
// overload *, the dereference operator.
// https://doc.rust-lang.org/std/ops/trait.Deref.html
// If T implements Deref<Target = U>, and x is a value of type T, then:
//  - In immutable contexts, *x on non-pointer types is equivalent
//    to *Deref::deref(&x).
//  - Values of type &T are coerced to values of type &U
//  - T implicitly implements all the (immutable) methods of the type U.
//
// https://doc.rust-lang.org/book/second-edition/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
// Implicit deref coercions with functions and methods.
//
// When a GameState is dereferenced, the "next" function included in that
// GameState is returned.
// This will simplify the change state execution from:
//    game_state = (game_state.next)(&mut game);
// to this:
//    game_state = game_state(&mut game);
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

// https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned
// Creates owned data from borrowed data, usually by cloning.
//
// let s: &str = "a";
// let ss: String = s.to_owned();
//
// let v: &[i32] = &[1, 2];
// let vv: Vec<i32> = v.to_owned();
//
// https://doc.rust-lang.org/std/default/trait.Default.html
// A trait for giving a type a useful default value.
impl ::std::default::Default for Game {
    fn default() -> Self {
        Game {
            player: Player {
                name: "".to_owned(),
                has_key: false,
            },
            door_locked: true,
            last_command: "".to_owned(),
        }
    }
}

impl Game {
    fn reset(&mut self) {
        self.player.has_key = false;
        self.door_locked = true;
    }

    fn start(&mut self) -> GameState<Game> {
        println!("You wake up in cell. You feel confused... How do you wanto to be remembered?");
        GameState::with_input(Self::save_name, String::from("start"))
    }

    fn end(&mut self) -> GameState<Game> {
        println!(
            "You solved the game! {} will be remembered!",
            self.player.name
        );
        GameState::completed(Self::end)
    }
}
