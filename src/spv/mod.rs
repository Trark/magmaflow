
#[macro_use]
mod dis;

pub mod types;
pub mod op;
pub mod raw;
pub mod logical;

use std::any::Any;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use self::raw::MemoryBlockResult;
use self::raw::MemoryBlock;

/// An instruction
pub trait Op: Debug + Display {
    /// Returns the name of the instruction to use in disassembly
    fn get_name(&self) -> &'static str;
}

/// An instruction from an extended instruction set
///
/// This is expected to itself be an enum variant in the instruction set
pub trait ExtInst: Any + ExtInstClone + Debug + Display {
    /// Returns the op in the extended instruction
    fn get_op(&self) -> &Op;

    fn as_any(&self) -> &Any;

    fn eq(&self, other: &ExtInst) -> bool;
}

/// Helper trait to implement box cloning for all ExtInst
pub trait ExtInstClone {
    fn clone_box(&self) -> Box<ExtInst>;
}

impl<T> ExtInstClone for T
    where T: 'static + ExtInst + Clone
{
    fn clone_box(&self) -> Box<ExtInst> {
        Box::new(self.clone())
    }
}

impl Clone for Box<ExtInst> {
    fn clone(&self) -> Box<ExtInst> {
        self.clone_box()
    }
}

/// Helper to own an ExtInst while providing equality tests
#[derive(Clone, Debug)]
pub struct ExtInstBox(Box<ExtInst>);

impl PartialEq<ExtInstBox> for ExtInstBox {
    fn eq(&self, other: &ExtInstBox) -> bool {
        ExtInst::eq(self.0.as_ref(), other.0.as_ref())
    }
}

impl AsRef<ExtInst> for ExtInstBox {
    fn as_ref(&self) -> &ExtInst {
        self.0.as_ref()
    }
}

impl Display for ExtInstBox {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self.as_ref(), f)
    }
}

impl dis::DisplayArgType for ExtInstBox {}

/// Represents an object that controls an extended instruction set
pub trait ExtInstSet {
    /// Returns the name of the instruction set as seen in OpExtInstImport instructions
    fn get_name(&self) -> &'static str;

    /// Attempts to read an instruction from the instruction set
    fn read_instruction<'a, 'b>(&'b self,
                                instruction: u32,
                                block: MemoryBlock<'a>)
                                -> MemoryBlockResult<'a, Box<ExtInst>>;

    /// Clones the instruction set handler
    fn duplicate(&self) -> Box<ExtInstSet>;
}
