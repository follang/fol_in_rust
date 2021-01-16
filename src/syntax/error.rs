#![allow(dead_code)]


use std::fmt;
use colored::Colorize;
use crate::syntax::point;

// pub trait Boxing {
//     fn box (&self) -> Box<Self>;
// }

pub trait Glitch: std::error::Error {}
macro_rules! glitch { ($err:expr $(,)?) => ({ Box::new($err) }); }
// macro_rules! E { ($err:expr $(,)?) => ({ Err(Box::new($err)) }); }
// macro_rules! V { ($val:expr $(,)?) => ({ Ok($val) }); () => ({ Ok(()) });  }

// macro_rules! log { 
//     ($(e:expr),*) => ({ 
//         let message = String::new()
//         $(message.push(&format!("{}", $e));)*
//         println!("{}" message);
//     });
// }


pub(crate) type Cont<T> = Result<T, Box<(dyn Glitch + 'static)>>;
pub(crate) type Void = Result<(), Box<(dyn Glitch + 'static)>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Typo {
    ParserUnexpected {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    ParserMissmatch {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    ParserSpaceRem {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    ParserSpaceAdd {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    ParserTypeDisbalance {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    ParserBodyForbidden {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    ParserNoType {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    ParserNeedsBody {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    ParserManyUnexpected {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    LexerPrimitiveAccess {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    LexerBracketUnmatch {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
    LexerSpaceAdd {
        msg: Option<String>,
        loc: Option<point::Location>,
    },
}

impl fmt::Display for Typo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (v, s, l, m);
        match self {
            Typo::ParserUnexpected { msg, loc } => { 
                v = " UNEXPECTED TOKEN ".to_string(); 
                s = "parsing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::ParserNeedsBody { msg, loc } => {
                v = " MISSING DECLARATATION ".to_string(); 
                s = "parsing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::ParserBodyForbidden { msg, loc } => { 
                v = " DECLARATATION FORBIDDEN ".to_string(); 
                s = "parsing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::ParserMissmatch { msg, loc } => { 
                v = " MISSMATCHED ARGUMENTS ".to_string(); 
                s = "parsing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::ParserSpaceAdd { msg, loc } => { 
                v = " MISSING BLANK SPACE ".to_string(); 
                s = "parsing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::ParserSpaceRem { msg, loc } => { 
                v = " OBSOLETE BLANK SPACE ".to_string(); 
                s = "parsing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::ParserTypeDisbalance { msg, loc } => { 
                v = " DISBALANCE OF TYPES ".to_string(); 
                s = "parsing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::ParserNoType { msg, loc } => { 
                v = " MISSING TYPE ANNOTATION ".to_string(); 
                s = "parsing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::ParserManyUnexpected { msg, loc } => { 
                v = " UNEXPECTED TOKEN ".to_string(); 
                s = "parsing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::LexerBracketUnmatch { msg, loc } => { 
                v = " UNMATCHED BRACKET ".to_string(); 
                s = "lexing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::LexerSpaceAdd { msg, loc } => { 
                v = " MISSING BLANK SPACE ".to_string(); 
                s = "lexing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
            Typo::LexerPrimitiveAccess { msg, loc } => { 
                v = " PRIMITIVE_ACCESS ".to_string(); 
                s = "lexing".to_string();
                m = msg.as_ref();
                l = loc.as_ref();
            },
        };
        write!(f, "\n{} >> {}:{}{}{}",
            " TYPO ".black().on_red(),
            (" ".to_string() + &s + " stage ").black().on_white().to_string(), v.on_red().bold().to_string(),
            match l { Some(val) => "\n".to_string() + &val.visualize(), None => "".to_string() },
            match m { Some(val) => "\n".to_string() + &val.to_string(), None => "".to_string() },
        )
    }
}
impl std::error::Error for Typo  {  }
impl Glitch for Typo {  }
impl Typo { pub fn r#box(self) -> Box<Self> { Box::new(self) } }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Flaw {
    GettingWrongPath {
        msg: Option<String>,
    },
    GettingNoEntry {
        msg: Option<String>,
    },
    ReadingEmptyFile {
        msg: Option<String>,
    },
    ReadingBadContent {
        msg: Option<String>,
    },
}
impl fmt::Display for Flaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (v, s, m);
        match self {
            Flaw::GettingWrongPath { msg } => { 
                s = "getting".to_string();
                v = " PATH_DOES_NOT_EXIST ".to_string(); 
                m = msg.as_ref();
            },
            Flaw::GettingNoEntry { msg } => { 
                s = "getting".to_string();
                v = " NO_FILE_FOUND ".to_string(); 
                m = msg.as_ref();
            },
            Flaw::ReadingBadContent { msg } => { 
                s = "reading".to_string();
                v = " NOT_VALID_TEXT_FILE ".to_string(); 
                m = msg.as_ref();
            },
            Flaw::ReadingEmptyFile { msg } => { 
                s = "reading".to_string();
                v = " FILE_IS_EMPTY ".to_string(); 
                m = msg.as_ref();
            },
        };
        write!(f, "\n{} >> {}:{}{}",
            " FLAW ".black().on_red(),
            (" ".to_string() + &s + " file ").black().on_white().to_string(), v.on_red().bold().to_string(),
            match m { Some(val) => "\n".to_string() + &val.to_string(), None => "".to_string() },
        )
    }
}
impl std::error::Error for Flaw  {  }
impl Glitch for Flaw  {  }
impl Flaw { pub fn r#box(self) -> Box<Self> { Box::new(self) } }
