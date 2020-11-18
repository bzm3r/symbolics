use crate::types::Type;

#[derive(Clone)]
pub struct Function {
    ty: Type,
    inputs: FuncSeq,
}

#[derive(Clone)]
pub struct FuncSeq {
    ty: Type,
    seq: Vec<Function>,
}

impl FuncSeq {
    pub fn new(fs: Vec<Function>) -> FuncSeq {
        FuncSeq {
            ty: Type::new_seq(&fs),
            seq: fs,
        }
    }
}

pub enum CreateError {
    InputTypeMismatch,
}

impl Function {
    pub fn new(label: &str, in_ty: Type, out_ty: Type) -> Function {
        Function {
            ty: Type::Function {
                label: String::from(label),
                input: Box::new(in_ty.clone()),
                output: Box::new(out_ty),
            },
            inputs: Function::new_auto_vars(in_ty),
        }
    }

    pub fn new_const(label: &str, ty: Type) -> Function {
        Function::new(label, Type::Empty, ty)
    }

    pub fn new_var(label: &str, ty: Type) -> Function {
        Function::new(label, Type::Unknown, ty)
    }

    pub fn new_auto_vars(ty: Type) -> Function {
        match ty {
            Type::Empty => Function::new("", Type::Empty, Type::Empty),
            Type::Unknown => {}
            Type::Primitive(_) => {}
            Type::Function { .. } => {}
            Type::Sequence(_) => {}
        }
    }

    pub fn with_inputs(mut self, inputs: FuncSeq) -> Result<Function, CreateError> {
        let Type::Function { output, .. } = &self.ty;
        if output == inputs.ty {
            self.inputs = inputs;
            Ok(self)
        } else {
            Err(CreateError::InputTypeMismatch)
        }
    }

    pub fn change_input(&mut self, ix: usize) {
        self.ix ==
    }
}
