
use spv::op::*;
use spv::types::*;

mod reader;
pub use self::reader::read_module;

/// Raw list of SpirV instructions
///
/// Instructions do not nessessarily pass validation rules.
#[derive(Clone, Debug, PartialEq)]
pub struct RawModule {
    /// Version number of the module
    pub version: Version,
    /// Tool and tool version used to generate the SPIR-V module
    pub generator: Generator,
    /// Bound on the highest id in the module (0 < id < bound)
    pub bound: u32,
    /// List of all instructions.
    pub instructions: Vec<Core>,
}

/// Enumeration of all core instructions (incomplete)
#[derive(Clone, Debug, PartialEq)]
pub enum Core {
    OpNop(OpNop),
    OpSource(OpSource),
    OpName(OpName),
    OpMemberName(OpMemberName),
    OpExtension(OpExtension),
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
    OpFunctionParameter(OpFunctionParameter),
    OpFunctionEnd(OpFunctionEnd),
    OpVariable(OpVariable),
    OpLoad(OpLoad),
    OpStore(OpStore),
    OpAccessChain(OpAccessChain),
    OpDecorate(OpDecorate),
    OpMemberDecorate(OpMemberDecorate),
    OpConvertUToF(OpConvertUToF),
    OpIMul(OpIMul),
    OpUMod(OpUMod),
    OpIEqual(OpIEqual),
    OpPhi(OpPhi),
    OpLoopMerge(OpLoopMerge),
    OpSelectionMerge(OpSelectionMerge),
    OpLabel(OpLabel),
    OpBranch(OpBranch),
    OpBranchConditional(OpBranchConditional),
    OpReturn(OpReturn),
}


#[derive(Debug, PartialEq)]
pub enum ReadError {
    UnexpectedEndOfStream,
    UnexpectedStreamAlignment,
    BadMagic,
    UnknownVersionBytes(u8, u8, u8, u8),
    UnknownVersion(Version),
    UnknownReservedHeaderu324,
    UnknownOp(u16, u16),
    UnimplementedOp(&'static str),
    WrongWordCountForOp,
    InvalidString,

    UnexpectedEndOfInstruction,
    InstructionHadExcessData,

    UnknownAddressingModel(u32),
    UnknownMemoryModel(u32),
    UnknownExecutionModel(u32),
    UnknownExecutionMode(u32),
    UnknownCapability(u32),
    UnknownDecoration(u32),
    UnknownBuiltIn(u32),
    UnknownFpRoundingMode(u32),
    UnknownLinkageType(u32),
    UnknownSignedness(u32),
    UnknownStorageClass(u32),
    UnknownFunctionParameterAttribute(u32),
    UnknownMemoryAccess(u32),
    UnknownDim(u32),
    UnknownDepthStatus(u32),
    UnknownArrayed(u32),
    UnknownMS(u32),
    UnknownSampledStatus(u32),
    UnknownImageFormat(u32),
    UnknownAccessQualifier(u32),
    UnknownLoopControl(u32),
    UnknownSelectionControl(u32),
}

pub type ReadResult<T> = Result<T, ReadError>;
