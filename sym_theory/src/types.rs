use std::borrow::Borrow;

#[derive(Clone, PartialEq)]
pub enum Type {
    Empty,
    Func(FuncType),
    FuncSeq(FuncSeqType),
}

#[derive(Clone)]
pub struct FuncType {
    pub label: String,
    input: Box<Type>,
    output: Box<Type>,
}

pub enum FuncTypeError {
    InputMultipleEmpties,
    OutputEmpty,
}

impl FuncType {
    pub fn new(label: &str, input: &Type, output: &Type) -> Result<FuncType, FuncTypeError> {
        match (input.iter().any(|(_, x)| x.is_empty()), output.is_empty()) {
            (_, true) => Err(FuncTypeError::OutputEmpty),
            (true, _) => Err(FuncTypeError::InputMultipleEmpties),
            (_, _) => Ok(FuncType {
                label: label.into(),
                input: Box::new(input.clone()),
                output: Box::new(output.clone()),
            }),
        }
    }

    pub fn output(&self) -> &Type {
        self.output.borrow()
    }
}

impl PartialEq for FuncType {
    fn eq(&self, other: &Self) -> bool {
        (self.label == other.label) && (self.input == other.input) && (self.output == other.output)
    }
}

#[derive(Clone)]
pub struct FuncSeqType {
    dimension: usize,
    seq: Vec<Type>,
}

impl PartialEq for FuncSeqType {
    fn eq(&self, other: &Self) -> bool {
        if self.seq.len() == other.seq.len() {
            for (x, y) in self.seq.iter().zip(other.seq.iter()) {
                if x != y {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl FuncSeqType {
    pub fn new(types: &[Type]) -> FuncSeqType {
        FuncSeqType {
            dimension: types.len(),
            seq: types.to_vec(),
        }
    }

    pub fn new_unit(ty: &Type) -> FuncSeqType {
        FuncSeqType {
            dimension: 1,
            seq: vec![ty.clone()],
        }
    }
}

impl Type {
    pub fn is_empty(&self) -> bool {
        match &self {
            Type::Empty => true,
            _ => false,
        }
    }

    pub fn dimension(&self) -> usize {
        match &self {
            Type::Empty => 0,
            Type::Function { .. } => 1,
            Type::FuncSeq(s) => s.dimension,
        }
    }

    pub fn iter(&self) -> TypesIter {
        TypesIter::init(&self)
    }

    pub fn new_func(label: &str, in_ty: &Type, out_ty: &Type) -> Type {
        Type::Func(FuncType {
            label: label.into(),
            input: Box::new(in_ty.clone()),
            output: Box::new(out_ty.clone()),
        })
    }

    pub fn new_seq(seq: Vec<Type>) -> Type {
        Type::FuncSeq(FuncSeqType {
            dimension: seq.len(),
            seq,
        })
    }
}

impl From<&Type> for FuncSeqType {
    fn from(ty: &Type) -> Self {
        match ty {
            Type::Empty | Type::Primitive(_) | Type::Func(_) => FuncSeqType::new_unit(ty),
            Type::FuncSeq(st) => st.clone(),
        }
    }
}

pub struct IterStackElem<'a> {
    len: usize,
    ix: usize,
    ty: &'a Type,
}

pub struct TypesIter<'a> {
    stack: Vec<IterStackElem<'a>>,
}

impl<'a> TypesIter<'a> {}

impl<'a> Iterator for TypesIter<'a> {
    type Item = (usize, &'a Type);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(elem) => match (elem.ix == elem.len, elem.ty) {
                (true, _) | (_, Type::Empty) => self.next(),
                (false, Type::Primitive(_)) => Some((self.stack.len(), elem.ty)),
                (false, Type::Func(ft)) => {
                    self.stack.push(IterStackElem {
                        len: ft.output.dimension(),
                        ix: 0,
                        ty: ft.output.borrow(),
                    });
                    Some((self.stack.len() - 1, elem.ty))
                }
                (false, Type::FuncSeq(st)) => {
                    self.stack.push(IterStackElem {
                        len: elem.len,
                        ix: elem.ix + 1,
                        ty: elem.ty,
                    });
                    Some((self.stack.len(), &st.seq[elem.ix]))
                }
            },
            None => None,
        }
    }
}

impl<'a> TypesIter<'a> {
    pub fn init(start: &'a Type) -> TypesIter<'a> {
        TypesIter {
            stack: vec![IterStackElem {
                len: start.dimension(),
                ix: 0,
                ty: start,
            }],
        }
    }
}
