use std::fmt;
use crate::types::{Vod, Con, Win, SLIDER};
use crate::syntax::point;
use crate::syntax::index;

type Part<T> = (T, point::Location);

pub struct Elements {
    chars: Box<dyn Iterator<Item = Con<Part<char>>>>,
    win: Win<Con<Part<char>>>,
    _in_count: usize,
}

impl Elements {
    pub fn curr(&self) -> Con<Part<char>> {
        self.win.1.clone()
    }
    ///next vector
    pub fn next_vec(&self) -> Vec<Con<Part<char>>> {
        self.win.2.clone()
    }
    pub fn peek(&self, index: usize) -> Con<Part<char>> { 
        let u = if index > SLIDER { 0 } else { index };
        self.next_vec()[u].clone() 
    }
    ///prev vector
    pub fn prev_vec(&self) -> Vec<Con<Part<char>>> {
        let mut rev = self.win.0.clone();
        rev.reverse();
        rev
    }
    pub fn seek(&self, index: usize) -> Con<Part<char>> { 
        let u = if index > SLIDER { 0 } else { index };
        self.prev_vec()[u].clone() 
    }

    pub fn init(file: &index::Input) -> Self {
        let mut prev = Vec::with_capacity(SLIDER);
        let mut next = Vec::with_capacity(SLIDER);
        let mut chars = Box::new(gen(file));
        for _ in 0..SLIDER { prev.push(Ok(('\0', point::Location::default()))) }
        for _ in 0..SLIDER { next.push(chars.next().unwrap_or(Ok(('\0', point::Location::default())))) }
        Self {
            chars,
            win: (prev, Ok(('\0', point::Location::default())), next),
            _in_count: SLIDER,
        }
    }

    pub fn bump(&mut self) -> Option<Con<Part<char>>> {
        match self.chars.next() {
            Some(v) => {
                    // TODO: Handle better .ok()
                    self.win.0.remove(0).ok(); self.win.0.push(self.win.1.clone());
                    self.win.1 = self.win.2[0].clone();
                    // TODO: Handle better .ok()
                    self.win.2.remove(0).ok(); self.win.2.push(v);
                    return Some(self.win.1.clone());
            },
            None => {
                if self._in_count > 0 {
                    // TODO: Handle better .ok()
                    self.win.0.remove(0).ok(); self.win.0.push(self.win.1.clone());
                    self.win.1 = self.win.2[0].clone();
                    // TODO: Handle better .ok()
                    self.win.2.remove(0).ok(); self.win.2.push(Ok(('\0', point::Location::default())));
                    self._in_count -= 1;
                    return Some(self.win.1.clone());
                } else { return None }
            }
        }
    }
    pub fn debug(&self) -> Vod {
        println!("{}\t{}", self.curr()?.1, self.curr()?.0);
        Ok(())
    }
}

impl Iterator for Elements {
    type Item = Con<Part<char>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.bump()
    }
}


impl fmt::Display for Elements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(_) = self.win.1.clone() {
            write!(f, "{} {}", self.win.1.clone().unwrap().1, self.win.1.clone().unwrap().0)
        } else {
            write!(f, "ERROR")
        }
    }
}


pub fn gen(file: &index::Input) -> impl Iterator<Item = Con<Part<char>>> {
    let mut loc = point::Location::default();
    let mut lines = index::Lines::init(file);
    let (line, source) = lines.next().unwrap();
    loc.set_source(&source);
    // if let Some(s) = lines.source() { loc.set_source(&s); }
    let mut chars = get_chars(line);
    loc.adjust(1,0); 
    let mut last_eol = false;
    std::iter::from_fn(move || {
        match chars.next() {
            Some(i) => {
                loc.new_char();
                return Some (Ok((i, loc.clone())))
            },
            None => {
                match lines.next() {
                    Some(j) => { 
                        loc.new_line();
                        loc.new_word();
                        if j.0 == "\0" { 
                            loc.adjust(0,0);
                            loc.set_source(&j.1);
                        }
                        chars = get_chars(j.0);
                        return Some(Ok((chars.next().unwrap_or('\0'), loc.clone())))
                    },
                    None => {
                        if !last_eol {
                            last_eol = true;
                            return Some(Ok(('\0', loc.clone())));
                        }
                        return None
                    }
                }
            }
        };
    })
}


fn get_chars(src: String) -> impl Iterator<Item = char> {
    let mut chrs = src.clone();
    std::iter::from_fn(move || {
        if let Some(ch) =  chrs.chars().next() {
            chrs.remove(0);
            return Some(ch.clone()) 
        };
        None
    })
}
