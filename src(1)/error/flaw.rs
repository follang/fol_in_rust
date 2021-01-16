#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]


use colored::Colorize;

use crate::scanning::locate;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub type Con<T> = Result<T, flaw_type>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum flaw_type {
    lexer(lexer),
    parser(parser),
}
impl fmt::Display for flaw_type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: String = match self {
            flaw_type::lexer(a) => {
                " lexing stage ".black().bold().on_white().to_string()
                    + ":"
                    + &a.to_string()
            }
            flaw_type::parser(b) => {
                " parsing stage ".black().bold().on_white().to_string()
                    + ":"
                    + &b.to_string()
            }
        };
        write!(f, "{}", value)
    }
}
impl std::error::Error for flaw_type {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum parser {
    parser_unexpected,
    parser_missmatch,
    parser_space_rem,
    parser_space_add,
    parser_type_disbalance,
    parser_body_forbidden,
    parser_no_type,
    parser_needs_body,
    parser_many_unexpected,
}
impl fmt::Display for parser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: String = match self {
            parser::parser_no_type => " MISSING TYPE ANNOTATION ".to_string(),
            parser::parser_missmatch => " MISSMATCHED ARGUMENTS ".to_string(),
            parser::parser_space_add => " MISSING BLANK SPACE ".to_string(),
            parser::parser_space_rem => " OBSOLETE BLANK SPACE ".to_string(),
            parser::parser_needs_body => " MISSING DECLARATATION ".to_string(),
            parser::parser_unexpected => " UNEXPECTED TOKEN ".to_string(),
            parser::parser_body_forbidden => " DECLARATATION FORBIDDEN ".to_string(),
            parser::parser_type_disbalance => " DISBALANCE OF TYPES ".to_string(),
            parser::parser_many_unexpected => " UNEXPECTED TOKEN ".to_string(),
        };
        write!(f, "{}", value.on_red().to_string())
    }
}
impl std::error::Error for parser {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum lexer {
    lexer_bracket_unmatch,
    lexer_space_add,
    lexer_primitive_access,
}
impl fmt::Display for lexer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: String = match self {
            lexer::lexer_space_add => " MISSING BLANK SPACE ".to_string(),
            lexer::lexer_bracket_unmatch => " UNMATCHED BRACKET ".to_string(),
            lexer::lexer_primitive_access => " PRIMITIVE_ACCESS ".to_string(),
        };
        write!(f, "{}", value.on_red().to_string())
    }
}
impl std::error::Error for lexer {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct flaw {
    typ: flaw_type,
    msg: String,
    loc: locate::LOCATION,
}

impl flaw {
    pub fn new(typ: flaw_type, msg: &str, loc: locate::LOCATION) -> Self {
        flaw {
            typ,
            msg: msg.to_string(),
            loc,
        }
    }
}

impl fmt::Display for flaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n {}\n {:>5}\n {:>5}  {}\n {:>5} {}{}\n {}",
            self.typ,
            self.loc,
            " |".red(),
            (self.loc.row().to_string() + " |").red(),
            get_line_at(self.loc.path(), self.loc.row()).red(),
            " |".red(),
            " ".repeat(self.loc.col()),
            "^".repeat(self.loc.len()),
            self.msg,
        )
    }
}


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FLAW {
    el: Vec<flaw>,
}

impl FLAW {
    pub fn init() -> Self {
        FLAW { el: Vec::new() }
    }
    pub fn list(&self) -> &Vec<flaw> {
        &self.el
    }
    pub fn report(&mut self, typ: flaw_type, msg: &str, loc: locate::LOCATION) {
        let e = flaw::new(typ, msg, loc);
        &self.el.push(e);
    }
    pub fn show(&mut self) {
        for (i, e) in self.el.iter().enumerate() {
            println!(
                "\n\n{} >> {}",
                " FLAW ".black().on_red(),
                e
            );
        }
        if self.el.len() != 0 {
            let num = if self.el.len() == 1 { "flaw" } else { "flaws" };
            println!(
                "\n\n{:^10} due to {:^3} previous {}",
                "ABORTING".black().on_red(),
                self.el.len().to_string().black().on_red(),
                num
            );
        }
    }
}

fn get_line_at(filepath: &str, line_num: usize) -> String {
    let file = File::open(Path::new(filepath)).unwrap();
    let mut lines = BufReader::new(&file).lines();
    lines.nth(line_num - 1).unwrap().unwrap()
}

fn line_loc_str(loc: &locate::LOCATION) -> String {
    format!(
        "{}\n {:>5}\n {:>5}  {}\n {:>5} {}{}",
        loc,
        " |".red(),
        (loc.row().to_string() + " |").red(),
        get_line_at(loc.path(), loc.row()).red(),
        " |".red(),
        " ".repeat(loc.col()),
        "^".repeat(loc.len()),
    )
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Typo {
    ParserUnexpected {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    ParserMissmatch {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    ParserSpaceRem {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    ParserSpaceAdd {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    ParserTypeDisbalance {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    ParserBodyForbidden {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    ParserNoType {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    ParserNeedsBody {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    ParserManyUnexpected {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    LexerPrimitiveAccess {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    LexerBracketUnmatch {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
    LexerSpaceAdd {
        msg: Option<&'static str>,
        loc: Option<locate::LOCATION>,
    },
}

impl fmt::Display for Typo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (v, s);
        let (mut l, mut m) = (None, None);
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
            },
            Typo::ParserBodyForbidden { msg, loc } => { 
                v = " DECLARATATION FORBIDDEN ".to_string(); 
                s = "parsing".to_string();
            },
            Typo::ParserMissmatch { msg, loc } => { 
                v = " MISSMATCHED ARGUMENTS ".to_string(); 
                s = "parsing".to_string();
            },
            Typo::ParserSpaceAdd { msg, loc } => { 
                v = " MISSING BLANK SPACE ".to_string(); 
                s = "parsing".to_string();
            },
            Typo::ParserSpaceRem { msg, loc } => { 
                v = " OBSOLETE BLANK SPACE ".to_string(); 
                s = "parsing".to_string();
            },
            Typo::ParserTypeDisbalance { msg, loc } => { 
                v = " DISBALANCE OF TYPES ".to_string(); 
                s = "parsing".to_string();
            },
            Typo::ParserNoType { msg, loc } => { 
                v = " MISSING TYPE ANNOTATION ".to_string(); 
                s = "parsing".to_string();
            },
            Typo::ParserManyUnexpected { msg, loc } => { 
                v = " UNEXPECTED TOKEN ".to_string(); 
                s = "parsing".to_string();
            },
            Typo::LexerBracketUnmatch { msg, loc } => { 
                v = " UNMATCHED BRACKET ".to_string(); 
                s = "lexing".to_string();
            },
            Typo::LexerSpaceAdd { msg, loc } => { 
                v = " MISSING BLANK SPACE ".to_string(); 
                s = "lexing".to_string();
            },
            Typo::LexerPrimitiveAccess { msg, loc } => { 
                v = " PRIMITIVE_ACCESS ".to_string(); 
                s = "lexing".to_string();
            },
        };
        write!(f, "\n\n{} >> {}:{}{}{}",
            " TYPO ".black().on_red(),
            (" ".to_string() + &s + " stage ").black().bold().on_white().to_string(), v.on_red().to_string(),
            match l { Some(val) => "\n".to_string() + &line_loc_str(val), None => "".to_string() },
            match m { Some(val) => "\n".to_string() + &val.to_string(), None => "".to_string() },
        )
    }
}
impl std::error::Error for Typo  {}
impl Glitch for Typo  {
    fn report(typ: Self) -> Self {
        typ
    }
}

pub trait Glitch: std::error::Error + fmt::Display {
    fn report(typ: Self) -> Self;
}

pub type con<T> = Result<T, dyn Glitch>;
pub type vod = Result<(), dyn Glitch>;