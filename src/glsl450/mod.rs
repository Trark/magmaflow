
pub mod op;

use std::any::Any;
use std::fmt;
use std::fmt::{Display, Formatter};
use spv::Op;
use spv::ExtInst;
use spv::ExtInstSet;
use spv::raw::MemoryBlock;
use spv::raw::MemoryBlockResult;
use spv::raw::ReadError;
use self::op::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Inst {
    Sin(Sin),
    Cos(Cos),
}

impl ExtInst for Inst {
    fn get_op(&self) -> &Op {
        match *self {
            Inst::Sin(ref op) => op,
            Inst::Cos(ref op) => op,
        }
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn eq(&self, other: &ExtInst) -> bool {
        match other.as_any().downcast_ref::<Inst>() {
            Some(other_glsl450) => PartialEq::eq(self, other_glsl450),
            None => false,
        }
    }
}

impl Display for Inst {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::Inst::*;
        match *self {
            Sin(ref op) => op.fmt(f),
            Cos(ref op) => op.fmt(f),
        }
    }
}

pub struct InstSet;

impl ExtInstSet for InstSet {
    fn get_name(&self) -> &'static str {
        "GLSL.std.450"
    }
    fn read_instruction<'a, 'b>(&'b self,
                                instruction: u32,
                                block: MemoryBlock<'a>)
                                -> MemoryBlockResult<'a, Box<ExtInst>> {
        let (block, inst) = try!(match instruction {
            13 => read_sin(block),
            14 => read_cos(block),
            _ => return Err(ReadError::UnknownExtInstOp(self.get_name(), instruction)),
        });
        Ok((block, Box::new(inst)))
    }
    fn duplicate(&self) -> Box<ExtInstSet> {
        Box::new(InstSet)
    }
}

fn read_sin<'a>(block: MemoryBlock<'a>) -> MemoryBlockResult<'a, Inst> {
    let (block, x) = try!(block.read_op_id());
    Ok((block, Inst::Sin(Sin { x: x })))
}

fn read_cos(block: MemoryBlock) -> MemoryBlockResult<Inst> {
    let (block, x) = try!(block.read_op_id());
    Ok((block, Inst::Cos(Cos { x: x })))
}
