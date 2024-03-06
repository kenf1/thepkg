pub mod imagefn;
pub mod iofn;
pub mod scrapefn;

use std::fmt;

//cout result
pub fn debug_fn<T>(fn_input: &T) where T: fmt::Debug{
    println!("{:?}",fn_input);
}