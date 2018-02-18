#![feature(rc_unique, cell_extras)]

pub mod first;
pub mod second;
pub mod third;
pub mod fourth;
pub mod fifth;
pub mod six;

#[cfg(test)]
mod lib {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
