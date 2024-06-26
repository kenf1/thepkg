#[cfg(feature = "crypt")]
pub mod cryptlib;

#[cfg(feature = "io")]
pub mod iofn;

#[cfg(feature = "qr")]
pub mod imagefn;

#[cfg(feature = "webscrape")]
pub mod scrapefn;

use std::fmt;

//cout result
pub fn debug_fn<T>(fn_input: &T) where T: fmt::Debug{
    println!("{:?}",fn_input);
}