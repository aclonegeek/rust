// run-pass

#![feature(track_caller)]

fn ptr_call(f: fn()) {
    f();
}

#[track_caller]
fn tracked() {
    let expected_line = line!() - 1;
    let location = std::panic::Location::caller();
    assert_eq!(location.file(), file!());
    assert_eq!(location.line(), expected_line, "call shims report location as fn definition");
}

trait Trait {
    fn trait_tracked();
}

impl Trait for () {
    #[track_caller]
    fn trait_tracked() {
        let expected_line = line!() - 1;
        let location = std::panic::Location::caller();
        assert_eq!(location.file(), file!());
        assert_eq!(location.line(), expected_line, "call shims report location as fn definition");
    }
}

trait TrackedTrait {
    #[track_caller]
    fn trait_tracked_default() {
        let expected_line = line!() - 1;
        let location = std::panic::Location::caller();
        assert_eq!(location.file(), file!());
        assert_eq!(location.line(), expected_line, "call shims report location as fn definition");
    }
}

impl TrackedTrait for () {}

fn main() {
    ptr_call(tracked);
    ptr_call(<() as Trait>::trait_tracked);
    ptr_call(<() as TrackedTrait>::trait_tracked_default);
}
