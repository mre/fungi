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

/// The Player that is playing the current game.
#[derive(Debug)]
struct Player {
    name: String,
    has_key: bool,
}

/// A Game is the general struct that is fully associated to what is easy to
/// expect from the concept of a single "game".
#[derive(Debug)]
struct Game {
    player: Player,
    last_command: String,
    door_locked: bool,
    state_name: String,
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
            state_name: "none".to_owned(),
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

    // https://doc.rust-lang.org/std/mem/fn.swap.html
    // Swaps the values at two mutable locations, without deinitializing either
    // one.
    //
    // use std::mem;
    // let mut x = 5;
    // let mut y = 42;
    // mem::swap(&mut x, &mut y);
    // assert_eq!(42, x);
    // assert_eq!(5, y);
    fn save_name(&mut self) -> GameState<Game> {
        ::std::mem::swap(&mut self.player.name, &mut self.last_command);
        println!("Your name will be {}!", self.player.name);
        GameState::without_input(Self::cell, String::from("save_name"))
    }

fn main() {
    use std::io::Write;
    use std::env;
    // the Game is created with its default values
    let mut game = Game::default();

    // the first GameState is without_input, sets the start and in named the same.
    let mut game_state = GameState::without_input(Game::start, String::from("start"));

    // we use the current game_state (here just "start") to "move/mutate/tick"
    // the current game
    game_state = game_state(&mut game);

    // the main loop that keep the game ticking, from one state to the next.
    // It stops only when the game_state is completed.
    while !game_state.completed {
        let key = "VERBOSE";
        match env::var(key) {
            Ok(_) => {
                // a game does not explicitly have a game_state but has a name
                // that can be set by the game_state only. (would be nice to
                // have some sort of invariant for this statement).
                println!("current game: {:?}", game);
            }
            Err(_) => (),
        }

        // here we check if the current game_state requires the user's input;
        // at the end we need to set the last issued command.
        if game_state.requires_input {
            let mut buffer = String::new();
            print!("> ");
            ::std::io::stdout().flush().unwrap();
            ::std::io::stdin().read_line(&mut buffer).unwrap();
            game.last_command = buffer[0..buffer.len() - 1].to_owned();
        } else {
            // if the current game_state does not require input, the
            // last_command is set to an "empty" value.
            game.last_command = "".to_owned();
        }

        // marks the game with the current state (just for tracking: this
        // dependency is not necessary at all).)
        game.state_name = game_state.name.to_owned();

        // it ticks the game to the next state.
        game_state = game_state(&mut game);
    }
}
