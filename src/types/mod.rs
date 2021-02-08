#![allow(dead_code)]
#![allow(unused_macros)]

pub mod error;
pub use crate::types::error::*;

pub mod id;
pub use crate::types::id::*;

pub const SLIDER: usize = 9;
pub type Win<T> = (Vec<T>, T, Vec<T>);
pub type Con<T> = Result<T, Box<(dyn Glitch + 'static)>>;
pub type Vod = Result<(), Box<(dyn Glitch + 'static)>>;

// impl<T> From<Con<T>> for Vod {
//     fn from(stat: Con<T>) -> Self {
//         if let Err(e) = stat {
//             return Err(e)
//         } else { return () }
//     }
// }

#[macro_export]
macro_rules! catch {
    ($err:expr $(,)?) => ({ Box::new($err) });
}

#[macro_export]
macro_rules! crash {
    () => ({ std::process::exit(0); });
    ($err:expr $(,)?) => ({ println!("{}", $err); std::process::exit(0); });
}

#[macro_export]
macro_rules! halt {
    () => ({ println!("\n ... UNIMPLEMENTED ... \n"); std::process::exit(0); });
}


#[macro_export]
macro_rules! printer {
    ($err:expr $(,)?) => ({ for e in $err { println!("{}", e)} });
}

#[macro_export]
macro_rules! repo {
    () => ({ Err( Box::new(Repo::Error) ) });
}
