use::druid::Data;

#[derive(Clone, Debug)]
pub enum Type {
    Primitive(String),
    Sequence(TypeSeq),
}

impl Data for Type {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Primitive(x), Type::Primitive(y)) => {
                x == y
            },
            (Type::Primitive(_), Type::Sequence(y)) => {
                if y.len == 1 {
                    y.seq[0] == *self
                } else {
                    false
                }
            },
            (Type::Sequence(x), Type::Primitive(_)) => {
                if x.len == 1 {
                    x.seq[0] == *other
                } else {
                    false
                }
            },
            (Type::Sequence(x), Type::Sequence(y)) => {
                if x.len == y.len {
                    x.seq.iter().zip(y.seq.iter()).all(|(u, v)| u == v)
                } else {
                    false
                }
            }
        }
    }
}

impl From<Vec<Type>> for Type {
    fn from(seq: Vec<Type>) -> Self {
        Type::Sequence(TypeSeq {
            len: seq.len(),
            seq,
        })
    }
}

#[derive(Clone, Debug)]
pub struct TypeSeq {
    len: usize,
    seq: Vec<Type>,
}

impl TypeSeq {
    pub fn iter(&self) -> TypeSeqIter {
        TypeSeqIter {
            ix: 0,
            seq: &self.seq,
        }
    }
}

pub struct TypeSeqIter<'a> {
    ix: usize,
    seq: &'a Vec<Type>,
}

impl<'a> Iterator for TypeSeqIter<'a> {
    type Item = &'a Type;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ix == self.seq.len() {
            None
        } else {
            let r = &self.seq[self.ix];
            self.ix += 1;
            Some(r)
        }
    }
}

impl PartialEq for TypeSeq {
    fn eq(&self, other: &Self) -> bool {
        if self.len == other.len {
            self.seq.iter().zip(other.seq.iter()).all(|(x, y)| x == y)
        } else {
            false
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct FuncType {
    name: String,
    pub(crate) inputs: TypeSeq,
    pub(crate) outputs: TypeSeq,
}

struct IterStackElem<'a> {
    len: usize,
    ix: usize,
    ty: &'a Type,
}

impl<'a> IterStackElem<'a> {
    fn new(ty: &'a Type) -> IterStackElem<'a> {
        IterStackElem { len: 0, ix: 0, ty }
    }
}

pub struct TypeIter<'a> {
    stack: Vec<IterStackElem<'a>>,
}

impl<'a> Iterator for TypeIter<'a> {
    type Item = &'a Type;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(mut elem) => match elem.ty {
                Type::Primitive(_) => Some(elem.ty),
                Type::Sequence(sty) => {
                    if elem.ix == elem.len {
                        self.next()
                    } else {
                        let r = &sty.seq[elem.ix];
                        elem.ix += 1;
                        self.stack.push(elem);
                        self.stack.push(IterStackElem::new(r));
                        Some(r)
                    }
                }
            },
            None => None,
        }
    }
}
