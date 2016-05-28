
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

/// Enumeration of all core instructions (incomplete)
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
    OpTypeMatrix(OpTypeMatrix),
    OpTypeImage(OpTypeImage),
    OpTypeSampler(OpTypeSampler),
    OpTypeSampledImage(OpTypeSampledImage),
    OpTypeArray(OpTypeArray),
    OpTypeRuntimeArray(OpTypeRuntimeArray),
    OpTypeStruct(OpTypeStruct),
    OpTypeOpaque(OpTypeOpaque),
    OpTypePointer(OpTypePointer),
    OpTypeFunction(OpTypeFunction),
    OpTypeEvent(OpTypeEvent),
    OpTypeDeviceEvent(OpTypeDeviceEvent),
    OpTypeQueue(OpTypeQueue),
    OpTypePipe(OpTypePipe),
    OpTypeForwardPointer(OpTypeForwardPointer),
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
