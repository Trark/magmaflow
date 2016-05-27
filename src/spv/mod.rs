
mod types;
pub use self::types::*;

mod op;
pub use self::op::*;

/// Raw list of SpirV instructions
#[derive(Debug, PartialEq)]
pub struct RawModule {
    pub generator: Generator,
    pub bound: Word,
    pub instructions: Vec<Op>,
}

/// Enumeration of all instructions (incomplete)
#[derive(Debug, PartialEq)]
pub enum Op {
    OpNop(OpNop),
    OpSource(OpSource),
    OpName(OpName),
    OpExtInstImport(OpExtInstImport),
    OpMemoryModel(OpMemoryModel),
    OpEntryPoint(OpEntryPoint),
    OpExecutionMode(OpExecutionMode),
    OpCapability(OpCapability),
    OpTypeVoid(OpTypeVoid),
    OpTypeBool(OpTypeBool),
    OpTypeInt(OpTypeInt),
    OpTypeFloat(OpTypeFloat),
    OpTypeVector(OpTypeVector),
    OpTypeFunction(OpTypeFunction),
    OpConstant(OpConstant),
    OpConstantComposite(OpConstantComposite),
    OpFunction(OpFunction),
    OpFunctionEnd(OpFunctionEnd),
    OpDecorate(OpDecorate),
    OpLabel(OpLabel),
    OpBranch(OpBranch),
    OpReturn(OpReturn),
}
