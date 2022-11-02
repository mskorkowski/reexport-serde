use reexport::*;

use serde::{
    self,
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate="reexport::serde")]
struct Sample {
    value: u32
}

fn main() {

}