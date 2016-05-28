
mod types;
pub use self::types::*;

mod op;
pub use self::op::*;

mod dis;

/// Raw list of SpirV instructions
#[derive(Clone, Debug, PartialEq)]
pub struct RawModule {
    pub version: Version,
    pub generator: Generator,
    pub bound: Word,
    pub instructions: Vec<Core>,
}

/// Enumeration of all core nstructions (incomplete)
#[derive(Clone, Debug, PartialEq)]
pub enum Core {
    OpNop(OpNop),
    OpSource(OpSource),
    OpName(OpName),
    OpMemberName(OpMemberName),
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
    OpTypeRuntimeArray(OpTypeRuntimeArray),
    OpTypeStruct(OpTypeStruct),
    OpTypePointer(OpTypePointer),
    OpTypeFunction(OpTypeFunction),
    OpConstant(OpConstant),
    OpConstantComposite(OpConstantComposite),
    OpFunction(OpFunction),
    OpFunctionEnd(OpFunctionEnd),
    OpVariable(OpVariable),
    OpLoad(OpLoad),
    OpStore(OpStore),
    OpAccessChain(OpAccessChain),
    OpDecorate(OpDecorate),
    OpMemberDecorate(OpMemberDecorate),
    OpConvertUToF(OpConvertUToF),
    OpIMul(OpIMul),
    OpLabel(OpLabel),
    OpBranch(OpBranch),
    OpReturn(OpReturn),
}
