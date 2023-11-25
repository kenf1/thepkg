pub mod Image;

use std::fmt;

//cout result
pub fn debug_fn<T>(fn_input: &T) where T: fmt::Debug{
    println!("{:?}",fn_input);
}