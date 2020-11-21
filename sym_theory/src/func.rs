//! A theory is composed of types, which are labels for
//! collections, and functions, which have their own
//! labels, and are also a label designating the collection
//! they belong to.

use crate::types::Type;

#[derive(Clone)]
pub struct Function {
    label: String,
    input_ty: Option<Vec<Type>>,
    output_ty: Type,
    input: Vec<Option<Function>>,
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
            input_ty: Some(input.iter().map(|x| x.output_ty).collect()),
            output_ty,
            input: input.into_iter().map(|x| Some(x)).collect(),
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
}

pub struct IterStackElem<'a> {
    len: usize,
    ix: usize,
    seq: &'a Vec<Option<Function>>,
}

pub struct FuncIter<'a> {
    stack: Vec<IterStackElem<'a>>,
}

impl<'a> Iterator for FuncIter<'a> {
    type Item = &'a Function;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(mut elem) => {
                if elem.ix < elem.len {
                    let r = elem.seq[elem.ix].as_ref();
                    elem.ix += 1;
                    self.stack.push(elem);
                    match r {
                        Some(f) => {
                            self.stack.push(IterStackElem {
                                len: f.input.len(),
                                ix: 0,
                                seq: &f.input,
                            });
                            Some(f)
                        }
                        None => self.next(),
                    }
                } else {
                    self.next()
                }
            }
            None => None,
        }
    }
}
