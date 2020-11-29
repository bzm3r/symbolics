//! A theory is composed of types, which are labels for
//! collections, and functions, which have their own
//! labels, and are also a label designating the collection
//! they belong to.

use crate::types::Type;
use druid::{Data};
use std::fmt::Display;
use uuid::Uuid;
use druid::im::Vector;

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    Const {
        name: String,
        output_ty: Type,
        id: Uuid,
    },
    Variable {
        tag: String,
        output_ty: Type,
        id: Uuid,
    },
    Concrete {
        name: String,
        input_ty: Vector<Type>,
        output_ty: Type,
        input: Vector<Function>,
        id: Uuid,
    },
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Function::Const { name, .. } => name,
                Function::Concrete { name, .. } => name,
                Function::Variable { tag, .. } => tag,
            }
        )
    }
}

impl Data for Function {
    fn same(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

pub enum InputSetError {
    OutOfBounds,
    TypeMismatch,
    CannotInputConstant,
}

impl Function {
    pub fn new_const(name: &str, output_ty: Type) -> Function {
        Function::Const {
            name: name.into(),
            output_ty,
            id: Uuid::new_v4(),
        }
    }

    pub fn new_var(tag: &str, output_ty: Type) -> Function {
        Function::Variable {
            tag: tag.into(),
            output_ty,
            id: Uuid::new_v4(),
        }
    }

    pub fn output_ty(&self) -> Type {
        match &self {
            Function::Const { output_ty, .. } => output_ty.clone(),
            Function::Variable { output_ty, .. } => output_ty.clone(),
            Function::Concrete { output_ty, .. } => output_ty.clone(),
        }
    }

    pub fn new_concrete(name: &str, input: Vector<Function>, output_ty: Type) -> Function {
        Function::Concrete {
            name: name.into(),
            input_ty: input.iter().cloned().map(|x| x.output_ty()).collect(),
            output_ty,
            input,
            id: Uuid::new_v4(),
        }
    }

    // pub fn set_children(&self, ix: usize, new: Function) -> Result<Function, InputSetError> {
    //     match &self.children_ty {
    //         Some(itys) => {
    //             if ix < itys.len() {
    //                 if itys[ix] == new.output_ty {
    //                     let mut r = self.clone();
    //                     r.children[ix] = Some(new);
    //                     Ok(r)
    //                 } else {
    //                     Err(InputSetError::TypeMismatch)
    //                 }
    //             } else {
    //                 Err(InputSetError::OutOfBounds)
    //             }
    //         }
    //         None => Err(InputSetError::CannotInputConstant),
    //     }
    // }

    // pub fn click(_ctx: &mut EventCtx, childrena: &mut Function, _env: &Env) {
    //     println!("{}", &childrena.label)
    // }

    pub fn input_len(&self) -> usize {
        match &self {
            Function::Concrete { input, .. } => {
                let mut len = 1;
                for i in input {
                    len += i.input_len();
                }
                len
            }
            _ => 0,
        }
    }

    pub fn iter(&self) -> FuncIter {
        FuncIter {
            stack: vec![IterStackElem {
                ix: 0,
                children: vec![&self],
            }],
        }
    }

    pub fn input_as_ref(&self) -> Vec<&Function> {
        match &self {
            Function::Concrete { input, .. } => input.iter().collect(),
            _ => vec![],
        }
    }

    pub fn get_id(&self) -> &Uuid {
        match &self {
            Function::Const { id, .. } => id,
            Function::Variable { id, .. } => id,
            Function::Concrete { id, .. } => id,
        }
    }

    pub fn by_id(&self, id: &Uuid) -> Option<Function> {
        for (_, child) in self.iter() {
            if child.get_id() == id {
                return Some(child.clone());
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct IterStackElem<'a> {
    ix: usize,
    children: Vec<&'a Function>,
}

pub struct FuncIter<'a> {
    stack: Vec<IterStackElem<'a>>,
}

impl<'a> Iterator for FuncIter<'a> {
    type Item = (usize, &'a Function);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(elem) => {
                let IterStackElem { ix, children } = elem;
                if ix < children.len() {
                    let r = children[ix];
                    let indent = self.stack.len();
                    self.stack.push(IterStackElem {
                        ix: ix + 1,
                        children,
                    });
                    match r {
                        Function::Const { .. } => Some((indent, r)),
                        Function::Variable { .. } => Some((indent, r)),
                        Function::Concrete { .. } => {
                            self.stack.push(IterStackElem {
                                ix: 0,
                                children: r.input_as_ref(),
                            });
                            Some((indent, r))
                        }
                    }
                } else {
                    self.next()
                }
            }
            None => None,
        }
    }
}
