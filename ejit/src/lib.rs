#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::{Debug, Display};

use libc::c_void;

#[derive(Debug)]
pub enum Error {
    BadNodeRef,
    SizeTooBig,
    TooManyNodes,
    TooManyBBs,
    TooManyFunctions,
}

#[derive(Debug, Clone, Copy)]
pub struct Size(u16);

#[derive(Debug, Clone, Copy)]
pub struct NodeRef(u16);

#[derive(Debug, Clone, Copy)]
pub struct BBRef(u16);

#[derive(Debug, Clone, Copy)]
pub struct FuncRef(u16);

impl TryFrom<usize> for Size {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().map_err(|_| Error::SizeTooBig)?))
    }
}

impl TryFrom<usize> for NodeRef {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().map_err(|_| Error::TooManyNodes)?))
    }
}

impl TryFrom<usize> for BBRef {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().map_err(|_| Error::TooManyBBs)?))
    }
}

impl TryFrom<usize> for FuncRef {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().map_err(|_| Error::TooManyFunctions)?))
    }
}

#[derive(Clone)]
struct Phi {
    bbrefs: Vec<BBRef>,
    noderefs: Vec<NodeRef>,
}

#[derive(Clone)]
pub enum Node {
    Nop,
    Const { bytes: Box<[u8]> },
    // Phi { phi: Box<Phi> },

    Not { size: Size, params: [NodeRef; 1] },
    Load { size: Size, params: [NodeRef; 1] },
    StackAlloc { size: Size, params: [NodeRef; 1] },
    HeapAlloc { size: Size, params: [NodeRef; 1] },
    HeapFree { size: Size, params: [NodeRef; 1] },
    Ret { size: Size, params: [NodeRef; 1] },

    Add { size: Size, params: [NodeRef; 2] },
    Store { size: Size, params: [NodeRef; 2] },
}

#[derive(Clone, Default)]
pub struct BB {
    noderefs: Vec<NodeRef>,
}

#[derive(Clone)]
pub struct EJit {
    functions: Vec<Func>,
}

#[derive(Clone)]
pub struct Program {
    _map: *mut c_void,
}

#[derive(Clone, Default)]
pub struct Func {
    name: String,
    bbs: Vec<BB>,
    nodes: Vec<Node>,
    bbrefs: Vec<BBRef>,
}

impl EJit {
    pub fn new() -> Self {
        EJit {
            functions: Vec::new(),
        }
    }

    pub fn compile(&self) -> Result<Vec<u8>, Error> {
        for f in &self.functions {
            f.compile()?;
        }
        Ok(Vec::new())
    }

    pub fn add_func(&mut self, func: Func) {
        self.functions.push(func);
    }
}

impl Node {
    pub fn size(&self, func: &Func) -> Size {
        use Node::*;
        let res = match &self {
            Nop => 0.try_into().unwrap(),
            Const { bytes } => bytes.len().try_into().unwrap(),
            Add { size, .. }
            | Not { size, .. }
            | Load { size, .. }
            | Store { size, .. }
            | StackAlloc { size, .. }
            | HeapAlloc { size, .. }
            | HeapFree { size, .. }
            | Ret { size, .. } => *size,
        };
        res
    }

    pub fn params(&self, func: &Func) -> &[NodeRef] {
        use Node::*;
        let res = match &self {
            Nop | Const { .. } => &[],

            Add { params, .. } | Store { params, .. } => &params[..],

            Not { params, .. }
            | Load { params, .. }
            | StackAlloc { params, .. }
            | HeapAlloc { params, .. }
            | HeapFree { params, .. }
            | Ret { params, .. } => &params[..],
        };
        res
    }

    pub fn op(&self) -> &'static str {
        use Node::*;
        match &self {
            Nop => "Nop",
            Const { .. } => "Const",
            Add { .. } =>  "Add",
            Not { .. } =>  "Not",
            Load { .. } =>  "Load",
            Store { .. } =>  "Store",
            StackAlloc { .. } =>  "StackAlloc",
            HeapAlloc { .. } =>  "HeapAlloc",
            HeapFree { .. } =>  "HeapFree",
            Ret { .. } =>   "Ret",
        }
    }

}

impl BB {
    fn new() -> Self {
        Self {
            noderefs: Vec::new(),
        }
    }

    fn nodes<'func, 's: 'func>(&'s self, func: &'func Func) -> impl Iterator<Item = &'func Node> {
        self.noderefs.iter().map(|noderef| func.node(*noderef))
    }

    fn noderefs(&self) -> &[NodeRef] {
        &self.noderefs
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.name)?;
        for (bb, bbref) in self.bbs().zip(self.bbrefs()) {
            writeln!(f, "  {:?}:", bbref)?;
            for (n, noderef) in bb.nodes(self).zip(bb.noderefs()) {
                use Node::*;
                let res = match n {
                    Nop => writeln!(f, "    {:?} Nop", noderef)?,
                    Const { bytes } => writeln!(f, "    {:?} Const {:02x?}", noderef, bytes)?,
                    _ => writeln!(f, "    {:?} {} {:?}", noderef, n.op(), n.params(self))?,
                };
            }
        }

        Ok(())
    }
}

impl Func {
    fn new(name: &str) -> Self {
        Func {
            name: name.to_string(),
            bbs: vec![BB::new()],
            nodes: Vec::new(),
            bbrefs: vec![BBRef(0)],
        }
    }

    fn add_node(&mut self, node: Node) -> Result<NodeRef, Error> {
        let noderef = self.nodes.len().try_into()?;
        self.bbs.last_mut().unwrap().noderefs.push(noderef);
        self.nodes.push(node);
        Ok(noderef)
    }

    pub fn u32_le(&mut self, val: u32) -> Result<NodeRef, Error> {
        self.add_node(Node::Const {
            bytes: Box::from(val.to_le_bytes()),
        })
    }

    pub fn ret(&mut self, value: NodeRef) -> Result<NodeRef, Error> {
        let res = self.add_node(Node::Ret {
            size: self.node(value).size(self),
            params: [value],
        });
        self.bbs.push(BB::new());
        res
    }

    fn node(&self, noderef: NodeRef) -> &Node {
        &self.nodes[noderef.0 as usize]
    }

    fn bb(&self, bbref: BBRef) -> &BB {
        &self.bbs[bbref.0 as usize]
    }

    fn compile(&self) -> Result<Vec<u8>, Error> {
        for bb in self.bbs() {

        }
        Ok(Vec::new())
    }

    fn bbs(&self) -> impl Iterator<Item = &BB> {
        self.bbrefs.iter().map(|bbref| self.bb(*bbref))
    }

    fn bbrefs(&self) -> &[BBRef] {
        &self.bbrefs
    }
}

#[test]
fn test_add() -> Result<(), Error> {
    assert_eq!(std::mem::size_of::<Node>(), 24);
    let mut ejit = EJit::new();
    let mut func = Func::new("add");
    let one = func.u32_le(1)?;
    let _bbref = func.ret(one)?;
    println!("{}", func);
    ejit.add_func(func);

    ejit.compile()?;
    Ok(())
}
