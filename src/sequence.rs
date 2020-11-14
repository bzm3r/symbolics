pub type Arity = u8;
pub type Symbol = String;

pub enum FnRule {
    Transforms(Vec<(Shape, Shape)>),
    Table(Vec<(Sequence, Sequence)>),
    Program(fn(Sequence) -> Sequence),
}

pub struct FnShape {
    pub arity: Arity,
    pub symbol: Symbol,
    pub ty: Vec<Symbol>,
}

pub struct Function {
    pub shape: FnShape,
    pub defn: Vec<FnRule>,
}

pub struct Shape {
    root: FnShape,
    tys: Vec<Symbol>,
}

pub struct Concrete {
    shape: Shape,
    root: Function,
    elements: Vec<Box<Sequence>>,
}

pub enum Sequence {
    Abstract(Shape),
    Concrete(Concrete)
}

