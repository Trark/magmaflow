
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
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

impl Display for RawModule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, "; SPIR-V\n"));
        try!(write!(f, "; Version: {}.{}\n", self.version.0, self.version.1));
        let generator_vendor = self.generator.tool.get_vendor();
        let generator_tool = self.generator.tool.get_tool();
        let gen_name = match (generator_vendor, generator_tool) {
            (Some(vendor), Some(tool)) => format!("{} {}", vendor, tool),
            (Some(vendor), None) => format!("{}", vendor),
            (None, Some(tool)) => format!("{}", tool),
            (None, None) => "Unknown".into(),
        };
        try!(write!(f, "; Generator: {}; {}\n", gen_name, self.generator.version));
        try!(write!(f, "; Bound: {}\n", self.bound));
        // Trivially 0 as we only support loading a RawModule with 0 in slot
        // reserved for schema
        try!(write!(f, "; Schema: 0\n"));
        for inst in &self.instructions {
            try!(write!(f, "{}\n", inst));
        }
        Ok(())
    }
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
    OpExtInst(OpExtInst),
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

impl Display for Core {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Core::OpNop(ref op) => Display::fmt(op, f),
            Core::OpSource(ref op) => Display::fmt(op, f),
            Core::OpName(ref op) => Display::fmt(op, f),
            Core::OpMemberName(ref op) => Display::fmt(op, f),
            Core::OpExtension(ref op) => Display::fmt(op, f),
            Core::OpExtInstImport(ref op) => Display::fmt(op, f),
            Core::OpExtInst(ref op) => Display::fmt(op, f),
            Core::OpMemoryModel(ref op) => Display::fmt(op, f),
            Core::OpEntryPoint(ref op) => Display::fmt(op, f),
            Core::OpExecutionMode(ref op) => Display::fmt(op, f),
            Core::OpCapability(ref op) => Display::fmt(op, f),
            Core::OpTypeVoid(ref op) => Display::fmt(op, f),
            Core::OpTypeBool(ref op) => Display::fmt(op, f),
            Core::OpTypeInt(ref op) => Display::fmt(op, f),
            Core::OpTypeFloat(ref op) => Display::fmt(op, f),
            Core::OpTypeVector(ref op) => Display::fmt(op, f),
            Core::OpTypeMatrix(ref op) => Display::fmt(op, f),
            Core::OpTypeImage(ref op) => Display::fmt(op, f),
            Core::OpTypeSampler(ref op) => Display::fmt(op, f),
            Core::OpTypeSampledImage(ref op) => Display::fmt(op, f),
            Core::OpTypeArray(ref op) => Display::fmt(op, f),
            Core::OpTypeRuntimeArray(ref op) => Display::fmt(op, f),
            Core::OpTypeStruct(ref op) => Display::fmt(op, f),
            Core::OpTypeOpaque(ref op) => Display::fmt(op, f),
            Core::OpTypePointer(ref op) => Display::fmt(op, f),
            Core::OpTypeFunction(ref op) => Display::fmt(op, f),
            Core::OpTypeEvent(ref op) => Display::fmt(op, f),
            Core::OpTypeDeviceEvent(ref op) => Display::fmt(op, f),
            Core::OpTypeQueue(ref op) => Display::fmt(op, f),
            Core::OpTypePipe(ref op) => Display::fmt(op, f),
            Core::OpTypeForwardPointer(ref op) => Display::fmt(op, f),
            Core::OpConstant(ref op) => Display::fmt(op, f),
            Core::OpConstantComposite(ref op) => Display::fmt(op, f),
            Core::OpFunction(ref op) => Display::fmt(op, f),
            Core::OpFunctionParameter(ref op) => Display::fmt(op, f),
            Core::OpFunctionEnd(ref op) => Display::fmt(op, f),
            Core::OpVariable(ref op) => Display::fmt(op, f),
            Core::OpLoad(ref op) => Display::fmt(op, f),
            Core::OpStore(ref op) => Display::fmt(op, f),
            Core::OpAccessChain(ref op) => Display::fmt(op, f),
            Core::OpDecorate(ref op) => Display::fmt(op, f),
            Core::OpMemberDecorate(ref op) => Display::fmt(op, f),
            Core::OpConvertUToF(ref op) => Display::fmt(op, f),
            Core::OpIMul(ref op) => Display::fmt(op, f),
            Core::OpUMod(ref op) => Display::fmt(op, f),
            Core::OpIEqual(ref op) => Display::fmt(op, f),
            Core::OpPhi(ref op) => Display::fmt(op, f),
            Core::OpLoopMerge(ref op) => Display::fmt(op, f),
            Core::OpSelectionMerge(ref op) => Display::fmt(op, f),
            Core::OpLabel(ref op) => Display::fmt(op, f),
            Core::OpBranch(ref op) => Display::fmt(op, f),
            Core::OpBranchConditional(ref op) => Display::fmt(op, f),
            Core::OpReturn(ref op) => Display::fmt(op, f),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ReadError {
    UnexpectedEndOfStream,
    UnexpectedStreamAlignment,
    BadMagic,
    UnknownVersionBytes(u8, u8, u8, u8),
    UnknownVersion(Version),
    UnknownReservedSchema,
    UnknownOp(u16, u16),
    UnimplementedOp(&'static str),
    WrongWordCountForOp,
    InvalidString,

    UnexpectedEndOfInstruction,
    InstructionHadExcessData,

    UnknownInstSet(String),
    UnknownInstSetId(OpId),
    UnknownExtInstOp(&'static str, u32),
    DuplicateResultId(ResultId),
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

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ReadError {
    fn description(&self) -> &str {
        use self::ReadError::*;
        match *self {
            UnexpectedEndOfStream => "unexpected end of stream",
            UnexpectedStreamAlignment => "stream is not word aligned",
            BadMagic => "invalid magic number",
            UnknownVersionBytes(_, _, _, _) => "unknown version bytes",
            UnknownVersion(_) => "unknown version",
            UnknownReservedSchema => "unknown reserved schema field",
            UnknownOp(_, _) => "unknown op",
            UnimplementedOp(_) => "unimplemented op",
            WrongWordCountForOp => "op has invalid word count",
            InvalidString => "invalid string literal",
            UnexpectedEndOfInstruction => "instruction didn't have enough data",
            InstructionHadExcessData => "instruction didn't require all data",
            UnknownInstSet(_) => "unknown instruction set",
            UnknownInstSetId(_) => "extended instruction referenced unknown instruction set",
            UnknownExtInstOp(_, _) => "unknown op in extended instruction set",
            DuplicateResultId(_) => "duplicate result id",
            UnknownAddressingModel(_) => "unknown addressing model",
            UnknownMemoryModel(_) => "unknown memory model",
            UnknownExecutionModel(_) => "unknown execution model",
            UnknownExecutionMode(_) => "unknown execution mode",
            UnknownCapability(_) => "unknown capability",
            UnknownDecoration(_) => "unknown decoration",
            UnknownBuiltIn(_) => "unknown built in",
            UnknownFpRoundingMode(_) => "unknown fp rounding mode",
            UnknownLinkageType(_) => "unknown linkage type",
            UnknownSignedness(_) => "unknown signedness",
            UnknownStorageClass(_) => "unknown storage class",
            UnknownFunctionParameterAttribute(_) => "unknown function parameter attribute",
            UnknownMemoryAccess(_) => "unknown memory access",
            UnknownDim(_) => "unknown image dimension",
            UnknownDepthStatus(_) => "unknown image depth hint",
            UnknownArrayed(_) => "unknown image arrayed status",
            UnknownMS(_) => "unknown multisampled status",
            UnknownSampledStatus(_) => "unknown sampled hint",
            UnknownImageFormat(_) => "unknown image format",
            UnknownAccessQualifier(_) => "unknown access qualifier",
            UnknownLoopControl(_) => "unknown loop control",
            UnknownSelectionControl(_) => "unknown selection control",
        }
    }
}

pub type ReadResult<T> = Result<T, ReadError>;

pub struct MemoryBlock<'a> {
    data: &'a [u32],
}

pub type MemoryBlockResult<'a, T> = Result<(MemoryBlock<'a>, T), ReadError>;

impl<'a> MemoryBlock<'a> {
    fn new(data: &[u32]) -> MemoryBlock {
        MemoryBlock { data: data }
    }

    pub fn read_word(self) -> MemoryBlockResult<'a, u32> {
        if self.data.len() > 0 {
            Ok((MemoryBlock { data: &self.data[1..] }, self.data[0]))
        } else {
            Err(ReadError::UnexpectedEndOfInstruction)
        }
    }

    pub fn end(&self) -> bool {
        self.data.len() == 0
    }

    pub fn read_op_id(self) -> MemoryBlockResult<'a, OpId> {
        let (next, word) = try!(self.read_word());
        Ok((next, OpId(word)))
    }
}
