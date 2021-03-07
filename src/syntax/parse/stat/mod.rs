use crate::types::Vod;

use crate::syntax::nodes::Nodes;
use crate::syntax::token::*;
use crate::syntax::lexer;
use super::Parse;
use crate::syntax::parse::{check, branch, Body, eater};

pub mod parameters;
pub mod generics;
pub mod datatype;
pub mod assign;
pub mod ident;
pub use crate::syntax::parse::stat::{
    parameters::*,
    generics::*,
    datatype::*,
    assign::*,
    ident::*,
};

pub struct ParserStat {
    pub nodes: Nodes,
    _style: Body,
}

impl ParserStat {
    pub fn init() -> Self {
        Self { nodes: Nodes::new(), _style: Body::Fun} 
    }
    pub fn style(&mut self, style: Body) {
        self._style = style;
    }
}
impl Parse for ParserStat {
    fn nodes(&self) -> Nodes { self.nodes.clone() }
    fn parse(&mut self, lex: &mut lexer::Elements) -> Vod {
        self.parse_one(lex)?;
        Ok(())
    }
}

impl ParserStat {
    fn parse_one(&mut self, lex: &mut lexer::Elements) -> Vod {
        match self._style {
            Body::Top => {
                let token = lex.curr(true)?; lex.eat();
                if (lex.curr(true)?.key().is_assign()
                    || (matches!(lex.curr(true)?.key(), KEYWORD::Symbol(_)) && lex.peek(0, true)?.key().is_assign()))
                    && branch::body_top(lex, true)? {
                    let mut parse_ass = ParserStatAss::init();
                    parse_ass.parse(lex)?;
                    self.nodes.extend(parse_ass.nodes);
                    return Ok(())
                } else if lex.curr(false)?.key().is_void() { return Ok(());
                } else { 
                    eater::until_term(lex, true)?;
                    return check::unexpected_top(lex, token); 
                }
            }
            Body::Typ => {
                let deep = lex.curr(false)?.loc().deep() - 1;
                loop{
                    let token = lex.curr(true)?; lex.eat();
                    if (matches!(lex.curr(false)?.key(), KEYWORD::Symbol(SYMBOL::CurlyC)) && lex.curr(false)?.loc().deep() == deep ) 
                        || lex.curr(false)?.key().is_eof() { 
                            return Ok(()) 
                    } 

                    if (lex.curr(true)?.key().is_assign() || (matches!(lex.curr(true)?.key(), KEYWORD::Symbol(_)) && lex.peek(0, true)?.key().is_assign()))
                        && branch::body_typ(lex, true)? {
                            let mut parse_ass = ParserStatAss::init();
                            parse_ass.parse(lex)?;
                            self.nodes.extend(parse_ass.nodes);
                            lex.eat(); lex.jump(0, false)?;
                    } else if lex.curr(false)?.key().is_void() { lex.jump(0, false)?;
                    } else { 
                        eater::until_term(lex, true)?;
                        return check::unexpected_typ(lex, token); 
                    }
                }
            }
            Body::Imp => {
                let deep = lex.curr(false)?.loc().deep() - 1;
                loop{
                    let token = lex.curr(true)?; lex.eat();
                    if (matches!(lex.curr(false)?.key(), KEYWORD::Symbol(SYMBOL::CurlyC)) && lex.curr(false)?.loc().deep() == deep ) 
                        || lex.curr(false)?.key().is_eof() { 
                            return Ok(()) 
                    } 

                    if (lex.curr(true)?.key().is_assign() || (matches!(lex.curr(true)?.key(), KEYWORD::Symbol(_)) && lex.peek(0, true)?.key().is_assign()))
                        && branch::body_imp(lex, true)? {
                            let mut parse_ass = ParserStatAss::init();
                            parse_ass.parse(lex)?;
                            self.nodes.extend(parse_ass.nodes);
                            lex.eat(); lex.jump(0, false)?;
                    } else if lex.curr(false)?.key().is_void() { lex.jump(0, false)?;
                    } else { 
                        eater::until_term(lex, true)?;
                        return check::unexpected_imp(lex, token); 
                    }
                }
            }
            Body::Fun => {
                let deep = lex.curr(false)?.loc().deep() - 1;
                loop{
                    // let token = lex.curr(true)?; lex.eat();
                    if (matches!(lex.curr(false)?.key(), KEYWORD::Symbol(SYMBOL::CurlyC)) && lex.curr(false)?.loc().deep() == deep ) 
                        || lex.curr(false)?.key().is_eof() { 
                            return Ok(()) 
                    } 

                    if (lex.curr(true)?.key().is_assign() || (matches!(lex.curr(true)?.key(), KEYWORD::Symbol(_)) && lex.peek(0, true)?.key().is_assign()))
                        && branch::body_fun(lex, true)? {
                            let mut parse_ass = ParserStatAss::init();
                            parse_ass.parse(lex)?;
                            self.nodes.extend(parse_ass.nodes);
                            lex.eat(); lex.jump(0, false)?;
                    } else if lex.curr(false)?.key().is_void() { lex.jump(0, false)?;
                    } else { 
                        eater::expr_body2(lex)?;
                        return Ok(())
                    }
                }
            }
        }
    }
    fn parse_two(&mut self, lex: &mut lexer::Elements) -> Vod {
        match self._style {
            Body::Top => {
                let token = lex.curr(true)?; lex.eat();
                if (lex.curr(true)?.key().is_assign()
                    || (matches!(lex.curr(true)?.key(), KEYWORD::Symbol(_)) && lex.peek(0, true)?.key().is_assign()))
                    && branch::body_top(lex, true)? {
                    let mut parse_ass = ParserStatAss::init();
                    parse_ass.parse(lex)?;
                    self.nodes.extend(parse_ass.nodes);
                    return Ok(())
                } else { 
                    eater::until_term(lex, true)?;
                    return check::unexpected_top(lex, token); 
                }
            }
            Body::Typ => {
                let token = lex.curr(true)?; lex.eat();
                if (lex.curr(true)?.key().is_assign()
                    || (matches!(lex.curr(true)?.key(), KEYWORD::Symbol(_)) && lex.peek(0, true)?.key().is_assign()))
                    && branch::body_typ(lex, true)? {
                    eater::expr_body2(lex)?;
                    // let mut parse_ass = ParserStatAss::init();
                    // parse_ass.parse(lex)?;
                    // self.nodes.extend(parse_ass.nodes);
                    return Ok(())
                } else { return check::unexpected_typ(lex, token); }
            }
            Body::Imp => {
                let token = lex.curr(true)?; lex.eat();
                if (lex.curr(true)?.key().is_assign()
                    || (matches!(lex.curr(true)?.key(), KEYWORD::Symbol(_)) && lex.peek(0, true)?.key().is_assign()))
                    && branch::body_imp(lex, true)? {
                    eater::expr_body2(lex)?;
                    // let mut parse_ass = ParserStatAss::init();
                    // parse_ass.parse(lex)?;
                    // self.nodes.extend(parse_ass.nodes);
                    return Ok(())
                } else { return check::unexpected_imp(lex, token); }
            }
            Body::Fun => {
                // let token = lex.curr(true)?;
                lex.eat();
                if (lex.curr(true)?.key().is_assign()
                    || (matches!(lex.curr(true)?.key(), KEYWORD::Symbol(_)) && lex.peek(0, true)?.key().is_assign()))
                    && branch::body_fun(lex, true)? {
                    eater::expr_body2(lex)?;
                    // let mut parse_ass = ParserStatAss::init();
                    // parse_ass.parse(lex)?;
                    // self.nodes.extend(parse_ass.nodes);
                    return Ok(())
                } else {
                    eater::expr_body2(lex)?;
                    return Ok(())
                    // return check::unexpected_fun(lex, token); 
                }
            }
        }
    }
}

