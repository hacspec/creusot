extern crate creusot_contracts;
use creusot_contracts::*;

#[logic]
pub fn bool_to_bool(b: bool) -> bool {
    b
}

#[logic]
pub fn ex() {
    pearlite! { bool_to_bool(!true) };
}
