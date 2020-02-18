// run-pass

#![feature(track_caller)]
#![allow(dead_code)]

trait Trait {
    #[track_caller] //~ WARN: `#[track_caller]` is ignored on function prototypes
    fn foo(&self);
}

extern "Rust" {
    #[track_caller] //~ WARN: `#[track_caller]` is ignored on function prototypes
    fn bar();
}

fn main() {}
