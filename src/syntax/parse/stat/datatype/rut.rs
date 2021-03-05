use crate::types::*;
use crate::syntax::index::Source;
use crate::syntax::nodes::{Node, Nodes, NodeStatAssFun};
use crate::syntax::nodes::*;
use crate::syntax::token::*;
use crate::syntax::lexer;
use super::Parse;
use crate::syntax::parse::{eater, check};
use crate::syntax::parse::stat::datatype;
use crate::syntax::parse::stat::parameters;



pub struct ParserStatData {
    pub nodes: Nodes,
}

impl ParserStatData {
    pub fn init() -> Self {
        Self { 
            nodes: Nodes::new(),
        } 
    }
}


impl Parse for ParserStatData {
    fn nodes(&self) -> Nodes { self.nodes.clone() }
    fn parse(&mut self, lex: &mut lexer::Elements) -> Vod {
        let mut node = datatype::NodeStatDatatypes::default();
        // match type
        check::expect_ident(lex, true)?; lex.eat();
        node.set_string(lex.curr(true)?.con().to_string());
        lex.jump(0, false)?; 

        // match options after type  -> "[opts]"
        if lex.curr(true)?.key() == KEYWORD::symbol(SYMBOL::squarO_) {
            lex.jump(0, true)?; 
            let mut lnode = fun::NodeStatAssFun::default();
            // match parameters after (  -> "(one, two)"
            let mut parameters = parameters::ParserStatParameters::init();
            parameters.parse(lex)?; lex.eat();
            if parameters.nodes.len() > 0 { lnode.set_parameters(Some(parameters.nodes.clone())) }

            // match datatypes after :  -> "int[opts][]"
            let mut dt = datatype::ParserStatDatatypes::init();
            dt.parse(lex)?;
            if dt.nodes.len() > 0 { lnode.set_datatype(Some(dt.nodes.get(0).clone())); }

            let mut ids = Nodes::new();
            let id = Node::new(Box::new(lnode));
            ids.push(id);
            node.set_form(Some(ids));
            check::expect(lex, KEYWORD::symbol(SYMBOL::squarC_), true)?;
            lex.jump(0, true)?;

        }

        // match restrictions after type  -> "[rest]"
        if lex.curr(true)?.key() == KEYWORD::symbol(SYMBOL::squarO_) {
            eater::until_bracket(lex)?;
        }
        let id = Node::new(Box::new(node));
        self.nodes.push(id);
        Ok(())
    }
}


