use crate::error::EngineError;
use crate::sequence::{Concrete, Symbol, Sequence};
use std::error::Error;
use std::rc::Rc;

pub type TheoryType = Symbol;

pub enum Connector {
    Eq,
    Imp,
    Pmi,
    End,
}

pub struct HeldRef<T> {
    reference: Rc<T>,
    held: Option<Rc<Theory>>,
}

pub enum Hint {
    Theorem(HeldRef<Proof>),
    Axiom(HeldRef<Concrete>),
}
pub struct Step {
    connector: Connector,
    hint: Hint,
    next: Sequence,
}

pub struct Rule {
    p0: Sequence,
    connector: Connector,
    p1: Sequence,
}

pub struct Proof {
    chain: Vec<Step>,
    conclusion: Option<Rule>,
}

pub struct Theory {
    ty: TheoryType,
    axioms: Vec<Concrete>,
    theorems: Vec<Proof>,
}

impl Theory {
    pub fn new(ty: TheoryType, existing: &[Theory]) -> Result<Theory, Box<dyn Error>> {
        if !existing.iter().any(|th| th.ty.eq(&ty)) {
            Ok(Theory {
                ty,
                axioms: vec![],
                theorems: vec![],
            })
        } else {
            Err(Box::new(EngineError::NameConflict))
        }
    }
}
