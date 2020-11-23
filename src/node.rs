use druid::im::Vector;
use druid::{EventCtx, Env, Data};

#[derive(Clone, Data)]
pub struct Node {
    pub children: Vector<Node>, // "x", "y"
    pub root: String, // "AND"
}

impl Node {
    pub fn new(root: &str) -> Node {
        Node {
            root: String::from(root),
            children: Vector::new(),
        }
    }

    pub fn set_children(mut self, children: Vec<Node>) -> Self {
        self.children = Vector::from(children);
        self
    }
    
    // pub fn new(roots: Vec<&str>) -> Node {
    //     let mut children = Vector::new();
    //     for rs in roots.iter().skip(1) {
    //         children.push_back(Node {
    //             children: Vector::new(),
    //             root: String::from(rs),
    //         })
    //     }
    //     Node {
    //         children,
    //         root: match roots.len() {
    //             0 => String::from(""),
    //             _ => String::from(roots[0]),
    //         },
    //     }
    // }

    pub fn iter(&self) -> Iter {
        Iter { node: self, ix_stack: vec![(0, 0)] }
    }

    pub fn click(ctx: &mut EventCtx, data: &mut Node, env: &Env) {
        println!("{}", &data.root)
    }

    pub fn data_len(&self) -> usize {
        let mut len = 1;
        for child in self.children.iter() {
            len += child.data_len();
        }
        len
    }
}

pub struct Iter<'a> {
    node: &'a Node,
    ix_stack: Vec<(usize, usize)>,
}

/// We need to iterate through this "recursively", and keep track of the 
/// "indent" (level of recursion), so that we can use that for display purposes. 
impl<'a> Iterator for Iter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ix_stack.pop() {
            Some((indent, ix)) => {
                if ix
            }
            None => None,
        }
    }
}

