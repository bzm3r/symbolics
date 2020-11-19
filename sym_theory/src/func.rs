use crate::types::{Type, FuncType, FuncTypeError};

#[derive(Clone)]
pub struct Function {
    ty: FuncType,
    inputs: FuncSeq,
}

#[derive(Clone)]
pub struct FuncSeq {
    ty: Type,
    seq: Vec<Function>,
}

impl FuncSeq {
    pub fn new(fs: Vec<Function>) -> Result<FuncSeq, FuncTypeError> {
        
    }
}

pub enum CreateError {
    InputTypeMismatch,
}

impl Function {
    pub fn new(label: &str, in_ty: Type, out_ty: Type) -> Result<Function, FuncTypeError> {
        Ok(Function {
            ty: FuncType::new(label, &in_ty, &out_ty)?,
            inputs: FuncSeq {
                ty: in_ty.clone(),
                seq: vec![]
            }
        })
    }

    pub fn from_inputs(label: &str, inputs: Vec<Function>, out_ty: Type) -> Result<Function, FuncTypeError> {
        let inputs = FuncSeq::new(inputs)?;
        Ok(Function {
            ty: FuncType::new(label, &in_ty, &out_ty)?,
            inputs,
        })
    }

    pub fn new_const(label: &str, ty: Type) -> Result<Function, FuncTypeError> {
        Function::new(label, Type::Empty, ty)
    }
}
