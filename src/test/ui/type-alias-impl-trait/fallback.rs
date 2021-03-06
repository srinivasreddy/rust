// Tests that we correctly handle the instantiated
// inference variable being completely unconstrained.
//
// check-pass
// revisions: min_tait full_tait
#![feature(min_type_alias_impl_trait)]
#![cfg_attr(full_tait, feature(type_alias_impl_trait))]
//[full_tait]~^ WARN incomplete

type Foo = impl Copy;

enum Wrapper<T> {
    First(T),
    Second
}

// This method constrains `Foo` to be `bool`
fn constrained_foo() -> Foo {
    true
}


// This method does not constrain `Foo`.
// Per RFC 2071, function bodies may either
// fully constrain an opaque type, or place no
// constraints on it.
fn unconstrained_foo() -> Wrapper<Foo> {
    Wrapper::Second
}

fn main() {}
