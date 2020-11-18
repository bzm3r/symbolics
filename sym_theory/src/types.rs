use crate::func::CreateError;

#[derive(Clone)]
pub enum Type {
    Empty,
    Unknown,
    Primitive(String),
    Function(FuncType),
    Sequence(SeqType),
}

#[derive(Clone)]
pub struct FuncType {
    label: String,
    input: Box<Type>,
    output: Box<Type>,
}

#[derive(Clone)]
pub struct SeqType {
    seq: Vec<Type>,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (Type::Empty, Type::Empty) => true,
            (Type::Primitive(x), Type::Primitive(y)) => x == y,
            (
                Type::Function {
                    label: x,
                    input: x_in,
                    output: x_out,
                },
                Type::Function {
                    label: y,
                    input: y_in,
                    output: y_out,
                },
            ) => (x == y) && (x_in == y_in) && (x_out == y_out),
            (Type::Sequence(xs), Type::Sequence(ys)) => {
                if xs.len() == ys.len() {
                    for (x, y) in xs.iter().zip(ys.iter()) {
                        if x != y {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
            (_, _) => false,
        }
    }
}

impl Type {
    pub fn new_func(label: &str, input: &Type, output: &Type) -> Type {
        Type::Function {
            label: label.into(),
            input: Box::new(input.clone()),
            output: Box::new(output.clone()),
        }
    }

    pub fn new_seq(types: &[Type]) -> Type {
        Type::Sequence(types.to_vec())
    }

    pub fn dimension(&self) -> usize {
        match &self {
            Type::Empty | Type::Unknown => 0,
            Type::Primitive(_) | Type::Function { .. } => 1,
            Type::Sequence(s) => s.len(),
        }
    }

    pub fn new_prim(label: &str) -> Type {
        Type::Primitive(label.into())
    }

    pub fn verify_func(ty: &Type) -> Result<Ok(), CreateError> {}
}
