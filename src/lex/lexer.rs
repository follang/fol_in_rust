#![allow(dead_code)]

use std::fmt;
use std::str;
use crate::lex::locate;
use crate::lex::reader;
use crate::lex::parts;
use crate::lex::token;

pub fn is_eol(ch: &char) -> bool {
    return *ch == '\n' || *ch == '\r'
}

pub fn is_space(ch: &char) -> bool {
    return *ch == ' ' || *ch == '\t'
}

pub fn is_digit(ch: &char) -> bool {
    return '0' <= *ch && *ch <= '9'
}

pub fn is_alpha(ch: &char) -> bool {
    return 'a' <= *ch && *ch <= 'z' || 'A' <= *ch && *ch <= 'Z' || *ch == '_'
}

pub fn is_symbol(ch: &char) -> bool {
    return '!' <= *ch && *ch <= '/' || ':' <= *ch && *ch <= '@' || '[' <= *ch && *ch <= '^' || '{' <= *ch && *ch <= '~'
}

pub fn is_oct_digit(ch: &char) -> bool {
    return '0' <= *ch && *ch <= '7' || *ch == '_'
}
pub fn is_hex_digit(ch: &char) -> bool {
    return '0' <= *ch && *ch <= '9' || 'a' <= *ch && *ch <= 'f' || 'A' <= *ch && *ch <= 'F' || *ch == '_'
}

pub fn is_alphanumeric(ch: &char) -> bool {
    return is_digit(ch) || is_alpha(ch)
}

pub struct LEX {
    key: token::KEYWORD,
    loc: locate::LOCATION,
    con: String,
}

impl LEX {
    fn new(key: token::KEYWORD, loc: locate::LOCATION, con: String) -> Self {
        LEX { key, loc, con }
    }
}

impl LEX {
    pub fn key(&self) -> &token::KEYWORD {
        &self.key
    }
    pub fn loc(&self) -> &locate::LOCATION {
        &self.loc
    }
    pub fn con(&self) -> &String {
        &self.con
    }
}

impl fmt::Display for LEX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <10} {: <10} {: <10}", self.loc, self.con, self.key)
    }
}

/// Creates an iterator that produces tokens from the input string.
// pub fn tokenize(mut input: &str) -> impl Iterator<Item = LEX> + '_ {
    // let mut loc = locate::LOCATION::new(&input);
    // std::iter::from_fn(move || {
        // if input.is_empty() { return None; }
        // let token = parts::PART::new(&input).advance_token(&mut loc);
        // input = &input[token.loc.len()..];
        // Some(token)
    // })
// }

// pub fn reader2<'a, I>(mut inp: I) -> impl Iterator<Item = LEX> +'a
// where
    // I: Iterator<Item = &'a reader::READER>,
// {
    // let red = inp.by_ref().next();
    // let mut loc = locate::LOCATION::new(&red.unwrap().name);
    // std::iter::from_fn(move || {
        // if red.unwrap().data.is_empty() {
            // return None;
        // }
        // let token = parts::PART::new(&red.unwrap().data).advance_token(&mut loc);
        // let a = red.unwrap().data[token.len..].to_string();
        // red.unwrap().set(a);
        // Some(token)
    // })
// }



/// Creates an iterator that produces tokens from the input string.
pub fn vectorize(red: &mut reader::READER) -> Vec<LEX> {
    let mut vec: Vec<LEX> = Vec::new();
    let mut loc = locate::LOCATION::new(&red.path);
    while !red.data.is_empty() {
        let token = parts::PART::new(&red.data).advance_token(&mut loc);
        red.data = red.data[token.loc.len()..].to_string();
        vec.push(token)
    }
    vec
}

/// Creates an iterator that produces tokens from the input string.
pub fn iteratize(red: &mut reader::READER) -> impl Iterator<Item = LEX> + '_ {
    let mut loc = locate::LOCATION::new(&red.path);
    std::iter::from_fn(move || {
        if red.data.is_empty() {
            return None;
        }
        let token = parts::PART::new(&red.data).advance_token(&mut loc);
        red.data = red.data[token.loc.len()..].to_string();
        Some(token)
    })
}

use crate::lex::token::*;
use crate::lex::token::KEYWORD::*;
impl parts::PART<'_> {
    fn eat_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(char) -> bool,
        {
            let mut eaten: usize = 0;
            while predicate(self.next_char()) && !self.is_eof() {
                eaten += 1;
                self.bump();
            }
            eaten
        }

    /// Parses a token from the input string.
    fn advance_token(&mut self, loc: &mut locate::LOCATION) -> LEX {
        loc.new_word();
        let key = illegal;
        let first_char = self.bump().unwrap();
        let mut result = LEX::new(key, loc.clone(), String::new());
        if is_eol(&first_char) {
            result.endline(self, false);
        } else if is_space(&first_char) {
            result.space(self);
        } else if first_char == '"' || first_char == '\'' || first_char == '`' {
            result.encap(self);
        } else if is_digit(&first_char) {
            result.digit(self);
        } else if is_symbol(&first_char) {
            result.symbol(self);
        } else if is_alpha(&first_char) {
            result.alpha(self);
        }
        *loc = result.loc.clone();
        result.loc.adjust();
        return result
    }
}

impl LEX {
    fn endline(&mut self, part: &mut parts::PART, terminated: bool) {
        self.loc.new_line();
        self.key = void(VOID::endline_ {terminated});
        while is_eol(&part.next_char()) || is_space(&part.next_char()) {
            if is_eol(&part.next_char()) { self.loc.new_line(); }
            part.bumpit(&mut self.loc);
        }
        self.con = " ".to_string();
    }
    fn space(&mut self, part: &mut parts::PART) {
        while is_space(&part.next_char()) {
            part.bumpit(&mut self.loc);
        }
        if is_eol(&part.next_char()) {
            part.bumpit(&mut self.loc);
            self.endline(part, false);
            return
        }
        self.key = void(VOID::space_);
        self.con = " ".to_string();
    }
    fn digit(&mut self, part: &mut parts::PART) {
        self.push_char(part);
        while is_digit(&part.next_char()) {
            // part.bumpit(&mut self.loc);
            // self.push_char(part);
            self.push_bump(part);
        }
        self.key = literal(LITERAL::decimal_);
    }
    fn encap(&mut self, part: &mut parts::PART) {
        self.push_char(part);
        self.key = literal(LITERAL::string_);
    }
    fn symbol(&mut self, part: &mut parts::PART) {
        self.push_char(part);
        self.key = symbol(SYMBOL::curlyBC_);
        match part.curr_char() {
            '{' | '[' | '(' => self.loc.deepen(),
            '}' | ']' | ')' => self.loc.soften(),
            _ => {}
        }
    }
    fn alpha(&mut self, part: &mut parts::PART) {
        self.push_char(part);
        while is_alpha(&part.next_char()) {
            // part.bumpit(&mut self.loc);
            // self.push_char(part);
            self.push_bump(part);
        }
        self.key = ident;
    }

    fn push_char(&mut self, part: &mut parts::PART) {
        self.con.push_str(&part.curr_char().to_string());
    }
    fn push_bump(&mut self, part: &mut parts::PART) {
        part.bumpit(&mut self.loc);
        self.con.push_str(&part.curr_char().to_string());
    }
}

