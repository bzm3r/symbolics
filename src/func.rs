//! A theory is composed of types, which are labels for
//! collections, and functions, which have their own
//! labels, and are also a label designating the collection
//! they belong to.

use crate::types::Type;
use druid::{Data, Env, EventCtx};

#[derive(Clone)]
pub struct Function {
    pub(crate) label: String,
    input_ty: Option<Vec<Type>>,
    output_ty: Type,
    input: Vec<Option<Function>>,
}

impl Data for Function {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

pub enum InputSetError {
    OutOfBounds,
    TypeMismatch,
    CannotInputConstant,
}

impl Function {
    pub fn new_const(label: &str, output_ty: Type) -> Function {
        Function {
            label: label.into(),
            input_ty: None,
            output_ty,
            input: vec![],
        }
    }

    pub fn new_var(label: &str, output_ty: Type) -> Function {
        Function {
            label: label.into(),
            input_ty: Some(vec![]),
            output_ty,
            input: vec![],
        }
    }

    pub fn from_input(label: &str, input: Vec<Function>, output_ty: Type) -> Function {
        Function {
            label: label.into(),
            input_ty: Some(input.iter().map(|x| x.output_ty.clone()).collect()),
            output_ty,
            input: input.into_iter().map(Some).collect(),
        }
    }

    pub fn set_input(&self, ix: usize, new: Function) -> Result<Function, InputSetError> {
        match &self.input_ty {
            Some(itys) => {
                if ix < itys.len() {
                    if itys[ix] == new.output_ty {
                        let mut r = self.clone();
                        r.input[ix] = Some(new);
                        Ok(r)
                    } else {
                        Err(InputSetError::TypeMismatch)
                    }
                } else {
                    Err(InputSetError::OutOfBounds)
                }
            }
            None => Err(InputSetError::CannotInputConstant),
        }
    }

    pub fn click(_ctx: &mut EventCtx, data: &mut Function, _env: &Env) {
        println!("{}", &data.label)
    }

    pub fn data_len(&self) -> usize {
        let mut len = 1;
        for opt_i in self.input.iter() {
            match opt_i {
                Some(i) => {
                    len += i.data_len();
                }
                None => {}
            }
        }
        len
    }

    pub fn iter(&self) -> FuncIter {
        FuncIter {
            stack: vec![IterStackElem::Primitive(&self)],
        }
    }
}

enum IterStackElem<'a> {
    Primitive(&'a Function),
    Sequence {
        len: usize,
        ix: usize,
        dat: &'a Vec<Option<Function>>,
    },
}

pub struct FuncIter<'a> {
    stack: Vec<IterStackElem<'a>>,
}

impl<'a> Iterator for FuncIter<'a> {
    type Item = &'a Function;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(mut elem) => match elem {
                IterStackElem::Primitive(f) => {
                    self.stack.push(IterStackElem::Sequence {
                        len: f.input.len(),
                        ix: 0,
                        dat: &f.input,
                    });
                    Some(f)
                }
                IterStackElem::Sequence { len, ix, dat } => {
                    if ix < len {
                        let r = dat[ix].as_ref();
                        let ix = ix + 1;
                        self.stack.push(IterStackElem::Sequence { len, ix, dat });
                        match r {
                            Some(f) => {
                                self.stack.push(IterStackElem::Sequence {
                                    len: f.input.len(),
                                    ix: 0,
                                    dat: &f.input,
                                });
                                Some(f)
                            }
                            None => self.next(),
                        }
                    } else {
                        self.next()
                    }
                }
            },
            None => None,
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
            && self.input_ty == other.input_ty
            && self.output_ty == other.output_ty
            && self.input.len() == other.input.len()
            && self
                .input
                .iter()
                .zip(other.input.iter())
                .all(|(x, y)| x == y)
    }
}
