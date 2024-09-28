use std::rc::Rc;

use data::{tree::DataTree, Path};
use mass::{IMass, Mass};

pub mod data;
pub mod mass;
fn main() {
    let data = Rc::new(DataTree::new());
    //let mass = Mass::new(Path::Mass, Path::LoadsMass, data);
    let mass = Mass::new(Path::Mass, Path::Loads(data::Loads::Mass), data);
    dbg!(mass.sum());
}