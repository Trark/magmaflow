
use std;
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
    OpUndef(OpUndef),
    OpSourceContinued(OpSourceContinued),
    OpSource(OpSource),
    OpSourceExtension(OpSourceExtension),
    OpName(OpName),
    OpMemberName(OpMemberName),
    OpString(OpString),
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
    OpBitcast(OpBitcast),
    OpIAdd(OpIAdd),
    OpFAdd(OpFAdd),
    OpISub(OpISub),
    OpFSub(OpFSub),
    OpIMul(OpIMul),
    OpFMul(OpFMul),
    OpUDiv(OpUDiv),
    OpSDiv(OpSDiv),
    OpFDiv(OpFDiv),
    OpUMod(OpUMod),
    OpSRem(OpSRem),
    OpSMod(OpSMod),
    OpFRem(OpFRem),
    OpFMod(OpFMod),
    OpIAddCarry(OpIAddCarry),
    OpISubBorrow(OpISubBorrow),
    OpUMulExtended(OpUMulExtended),
    OpSMulExtended(OpSMulExtended),
    OpBitwiseOr(OpBitwiseOr),
    OpBitwiseXor(OpBitwiseXor),
    OpBitwiseAnd(OpBitwiseAnd),
    OpIEqual(OpIEqual),
    OpINotEqual(OpINotEqual),
    OpUGreaterThan(OpUGreaterThan),
    OpSGreaterThan(OpSGreaterThan),
    OpUGreaterThanEqual(OpUGreaterThanEqual),
    OpSGreaterThanEqual(OpSGreaterThanEqual),
    OpULessThan(OpULessThan),
    OpSLessThan(OpSLessThan),
    OpULessThanEqual(OpULessThanEqual),
    OpSLessThanEqual(OpSLessThanEqual),
    OpFOrdEqual(OpFOrdEqual),
    OpFUnordEqual(OpFUnordEqual),
    OpFOrdNotEqual(OpFOrdNotEqual),
    OpFUnordNotEqual(OpFUnordNotEqual),
    OpFOrdLessThan(OpFOrdLessThan),
    OpFUnordLessThan(OpFUnordLessThan),
    OpFOrdGreaterThan(OpFOrdGreaterThan),
    OpFUnordGreaterThan(OpFUnordGreaterThan),
    OpFOrdLessThanEqual(OpFOrdLessThanEqual),
    OpFUnordLessThanEqual(OpFUnordLessThanEqual),
    OpFOrdGreaterThanEqual(OpFOrdGreaterThanEqual),
    OpFUnordGreaterThanEqual(OpFUnordGreaterThanEqual),
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
            Core::OpUndef(ref op) => Display::fmt(op, f),
            Core::OpSourceContinued(ref op) => Display::fmt(op, f),
            Core::OpSource(ref op) => Display::fmt(op, f),
            Core::OpSourceExtension(ref op) => Display::fmt(op, f),
            Core::OpName(ref op) => Display::fmt(op, f),
            Core::OpMemberName(ref op) => Display::fmt(op, f),
            Core::OpString(ref op) => Display::fmt(op, f),
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
            Core::OpBitcast(ref op) => Display::fmt(op, f),
            Core::OpIAdd(ref op) => Display::fmt(op, f),
            Core::OpFAdd(ref op) => Display::fmt(op, f),
            Core::OpISub(ref op) => Display::fmt(op, f),
            Core::OpFSub(ref op) => Display::fmt(op, f),
            Core::OpIMul(ref op) => Display::fmt(op, f),
            Core::OpFMul(ref op) => Display::fmt(op, f),
            Core::OpUDiv(ref op) => Display::fmt(op, f),
            Core::OpSDiv(ref op) => Display::fmt(op, f),
            Core::OpFDiv(ref op) => Display::fmt(op, f),
            Core::OpUMod(ref op) => Display::fmt(op, f),
            Core::OpSRem(ref op) => Display::fmt(op, f),
            Core::OpSMod(ref op) => Display::fmt(op, f),
            Core::OpFRem(ref op) => Display::fmt(op, f),
            Core::OpFMod(ref op) => Display::fmt(op, f),
            Core::OpIAddCarry(ref op) => Display::fmt(op, f),
            Core::OpISubBorrow(ref op) => Display::fmt(op, f),
            Core::OpUMulExtended(ref op) => Display::fmt(op, f),
            Core::OpSMulExtended(ref op) => Display::fmt(op, f),
            Core::OpBitwiseOr(ref op) => Display::fmt(op, f),
            Core::OpBitwiseXor(ref op) => Display::fmt(op, f),
            Core::OpBitwiseAnd(ref op) => Display::fmt(op, f),
            Core::OpIEqual(ref op) => Display::fmt(op, f),
            Core::OpINotEqual(ref op) => Display::fmt(op, f),
            Core::OpUGreaterThan(ref op) => Display::fmt(op, f),
            Core::OpSGreaterThan(ref op) => Display::fmt(op, f),
            Core::OpUGreaterThanEqual(ref op) => Display::fmt(op, f),
            Core::OpSGreaterThanEqual(ref op) => Display::fmt(op, f),
            Core::OpULessThan(ref op) => Display::fmt(op, f),
            Core::OpSLessThan(ref op) => Display::fmt(op, f),
            Core::OpULessThanEqual(ref op) => Display::fmt(op, f),
            Core::OpSLessThanEqual(ref op) => Display::fmt(op, f),
            Core::OpFOrdEqual(ref op) => Display::fmt(op, f),
            Core::OpFUnordEqual(ref op) => Display::fmt(op, f),
            Core::OpFOrdNotEqual(ref op) => Display::fmt(op, f),
            Core::OpFUnordNotEqual(ref op) => Display::fmt(op, f),
            Core::OpFOrdLessThan(ref op) => Display::fmt(op, f),
            Core::OpFUnordLessThan(ref op) => Display::fmt(op, f),
            Core::OpFOrdGreaterThan(ref op) => Display::fmt(op, f),
            Core::OpFUnordGreaterThan(ref op) => Display::fmt(op, f),
            Core::OpFOrdLessThanEqual(ref op) => Display::fmt(op, f),
            Core::OpFUnordLessThanEqual(ref op) => Display::fmt(op, f),
            Core::OpFOrdGreaterThanEqual(ref op) => Display::fmt(op, f),
            Core::OpFUnordGreaterThanEqual(ref op) => Display::fmt(op, f),
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
    UnknownFpFastMathMode(u32),
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
            UnknownFpFastMathMode(_) => "unknown fp fast math mode",
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

pub type MemoryBlockResult<'a, T> = ReadResult<(MemoryBlock<'a>, T)>;

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

    pub fn remaining(&self) -> usize {
        self.data.len()
    }

    pub fn end(&self) -> bool {
        self.data.len() == 0
    }

    pub fn read_op_id(self) -> MemoryBlockResult<'a, OpId> {
        let (next, word) = try!(self.read_word());
        Ok((next, OpId(word)))
    }
}

pub trait MemoryBlockRead: Sized {
    fn read(block: MemoryBlock) -> MemoryBlockResult<Self>;
}

impl MemoryBlockRead for OpId {
    fn read(block: MemoryBlock) -> MemoryBlockResult<OpId> {
        let (block, id) = try!(block.read_word());
        Ok((block, OpId(id)))
    }
}

impl MemoryBlockRead for ResultId {
    fn read(block: MemoryBlock) -> MemoryBlockResult<ResultId> {
        let (block, id) = try!(block.read_word());
        Ok((block, ResultId(id)))
    }
}

impl<T: MemoryBlockRead> MemoryBlockRead for Vec<T> {
    fn read(mut block: MemoryBlock) -> MemoryBlockResult<Vec<T>> {
        let item_word_size = std::cmp::min(1, std::mem::size_of::<T>() / 4);
        let assumed_size = block.remaining() / item_word_size;
        let mut items = Vec::with_capacity(assumed_size);
        while block.remaining() > 0 {
            let (next_block, item) = try!(<T as MemoryBlockRead>::read(block));
            items.push(item);
            block = next_block;
        }
        Ok((block, items))
    }
}

impl<T: MemoryBlockRead> MemoryBlockRead for Option<T> {
    fn read(block: MemoryBlock) -> MemoryBlockResult<Option<T>> {
        if block.remaining() > 0 {
            let (block, item) = try!(<T as MemoryBlockRead>::read(block));
            Ok((block, Some(item)))
        } else {
            Ok((block, None))
        }
    }
}

impl MemoryBlockRead for u32 {
    fn read(block: MemoryBlock) -> MemoryBlockResult<u32> {
        Ok(try!(block.read_word()))
    }
}

impl MemoryBlockRead for LitString {
    fn read(mut block: MemoryBlock) -> MemoryBlockResult<LitString> {
        let mut bytes = Vec::with_capacity(16);
        'null: loop {
            let (next_block, word) = try!(block.read_word());
            block = next_block;
            let chars = [(word & 0xFF) as u8,
                         ((word >> 8) & 0xFF) as u8,
                         ((word >> 16) & 0xFF) as u8,
                         (word >> 24) as u8];
            for c in chars.into_iter() {
                if *c == 0 {
                    break 'null;
                }
                bytes.push(*c);
            }
        }
        let sr = String::from_utf8(bytes);
        match sr {
            Ok(s) => Ok((block, s)),
            Err(_) => Err(ReadError::InvalidString),
        }
    }
}

impl MemoryBlockRead for SourceLanguage {
    fn read(block: MemoryBlock) -> MemoryBlockResult<SourceLanguage> {
        let (block, word) = try!(block.read_word());
        let lang = match word {
            0 => SourceLanguage::Unknown,
            1 => SourceLanguage::Essl,
            2 => SourceLanguage::Glsl,
            3 => SourceLanguage::OpenCL_C,
            4 => SourceLanguage::OpenCL_Cpp,
            id => SourceLanguage::Other(id),
        };
        Ok((block, lang))
    }
}

impl MemoryBlockRead for SourceVersion {
    fn read(block: MemoryBlock) -> MemoryBlockResult<SourceVersion> {
        let (block, word) = try!(block.read_word());
        Ok((block, SourceVersion(word)))
    }
}

impl MemoryBlockRead for MemberIndex {
    fn read(block: MemoryBlock) -> MemoryBlockResult<MemberIndex> {
        let (block, word) = try!(block.read_word());
        Ok((block, MemberIndex(word)))
    }
}

impl MemoryBlockRead for AddressingModel {
    fn read(block: MemoryBlock) -> MemoryBlockResult<AddressingModel> {
        let (block, word) = try!(block.read_word());
        let am = match word {
            0 => AddressingModel::Logical,
            1 => AddressingModel::Physical32,
            2 => AddressingModel::Physical64,
            id => return Err(ReadError::UnknownAddressingModel(id)),
        };
        Ok((block, am))
    }
}

impl MemoryBlockRead for MemoryModel {
    fn read(block: MemoryBlock) -> MemoryBlockResult<MemoryModel> {
        let (block, word) = try!(block.read_word());
        let mm = match word {
            0 => MemoryModel::Simple,
            1 => MemoryModel::Glsl450,
            2 => MemoryModel::OpenCL,
            id => return Err(ReadError::UnknownMemoryModel(id)),
        };
        Ok((block, mm))
    }
}

impl MemoryBlockRead for ExecutionModel {
    fn read(block: MemoryBlock) -> MemoryBlockResult<ExecutionModel> {
        let (block, word) = try!(block.read_word());
        let mm = match word {
            0 => ExecutionModel::Vertex,
            1 => ExecutionModel::TesselationControl,
            2 => ExecutionModel::TesselationEvaluation,
            3 => ExecutionModel::Geometry,
            4 => ExecutionModel::Fragment,
            5 => ExecutionModel::GlCompute,
            6 => ExecutionModel::Kernel,
            id => return Err(ReadError::UnknownExecutionModel(id)),
        };
        Ok((block, mm))
    }
}

impl MemoryBlockRead for ExecutionMode {
    fn read(block: MemoryBlock) -> MemoryBlockResult<ExecutionMode> {
        let (block, word) = try!(block.read_word());
        let mode = match word {
            0 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, ExecutionMode::Invocations(num)));
            }
            1 => ExecutionMode::SpacingEqual,
            2 => ExecutionMode::SpacingFractionalEven,
            3 => ExecutionMode::SpacingFractionalOdd,
            4 => ExecutionMode::VertexOrderCw,
            5 => ExecutionMode::VertexOrderCcw,
            6 => ExecutionMode::PixelCenterInteger,
            7 => ExecutionMode::OriginUpperLeft,
            8 => ExecutionMode::OriginLowerLeft,
            9 => ExecutionMode::EarlyFragmentTests,
            10 => ExecutionMode::PointMode,
            11 => ExecutionMode::Xfb,
            12 => ExecutionMode::DepthReplacing,
            14 => ExecutionMode::DepthGreater,
            15 => ExecutionMode::DepthLess,
            16 => ExecutionMode::DepthUnchanged,
            17 => {
                let (block, x) = try!(MemoryBlockRead::read(block));
                let (block, y) = try!(MemoryBlockRead::read(block));
                let (block, z) = try!(MemoryBlockRead::read(block));
                return Ok((block, ExecutionMode::LocalSize(x, y, z)));
            }
            18 => {
                let (block, x) = try!(MemoryBlockRead::read(block));
                let (block, y) = try!(MemoryBlockRead::read(block));
                let (block, z) = try!(MemoryBlockRead::read(block));
                return Ok((block, ExecutionMode::LocalSizeHint(x, y, z)));
            }
            19 => ExecutionMode::InputPoints,
            20 => ExecutionMode::InputLines,
            21 => ExecutionMode::InputLinesAdjacency,
            22 => ExecutionMode::Triangles,
            23 => ExecutionMode::InputTrianglesAdjacency,
            24 => ExecutionMode::Quads,
            25 => ExecutionMode::Isolines,
            26 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, ExecutionMode::OutputVerticies(num)));
            }
            27 => ExecutionMode::OutputPoints,
            28 => ExecutionMode::OutputLineStrip,
            29 => ExecutionMode::OutputTriangleStrip,
            30 => {
                let (block, id) = try!(<OpId as MemoryBlockRead>::read(block));
                return Ok((block, ExecutionMode::VecTypeHint(id)));
            }
            31 => ExecutionMode::ContractionOff,
            id => return Err(ReadError::UnknownExecutionMode(id)),
        };
        Ok((block, mode))
    }
}

impl MemoryBlockRead for Capability {
    fn read(block: MemoryBlock) -> MemoryBlockResult<Capability> {
        let (block, word) = try!(block.read_word());
        let capability = match word {
            0 => Capability::Matrix,
            1 => Capability::Shader,
            2 => Capability::Geometry,
            3 => Capability::Tessellation,
            4 => Capability::Addresses,
            5 => Capability::Linkage,
            6 => Capability::Kernel,
            7 => Capability::Vector16,
            8 => Capability::Float16Buffer,
            9 => Capability::Float16,
            10 => Capability::Float64,
            11 => Capability::Int64,
            12 => Capability::Int64Atomics,
            13 => Capability::ImageBasic,
            14 => Capability::ImageReadWrite,
            15 => Capability::ImageMipmap,
            17 => Capability::Pipes,
            18 => Capability::Groups,
            19 => Capability::DeviceEnqueue,
            20 => Capability::LiteralSampler,
            21 => Capability::AtomicStorage,
            22 => Capability::Int16,
            23 => Capability::TessellationPointSize,
            24 => Capability::GeometryPointSize,
            25 => Capability::ImageGatherExtended,
            27 => Capability::StorageImageMultisample,
            28 => Capability::UniformBufferArrayDynamicIndexing,
            29 => Capability::SampledImageArrayDynamicIndexing,
            30 => Capability::StorageBufferArrayDynamicIndexing,
            31 => Capability::StorageImageArrayDynamicIndexing,
            32 => Capability::ClipDistance,
            33 => Capability::CullDistance,
            34 => Capability::ImageCubeArray,
            35 => Capability::SampleRateShading,
            36 => Capability::ImageRect,
            37 => Capability::SampledRect,
            38 => Capability::GenericPointer,
            39 => Capability::Int8,
            40 => Capability::InputAttachment,
            41 => Capability::SparseResidency,
            42 => Capability::MinLod,
            43 => Capability::Sampled1D,
            44 => Capability::Image1D,
            45 => Capability::SampledCubeArray,
            46 => Capability::SampledBuffer,
            47 => Capability::ImageBuffer,
            48 => Capability::ImageMSArray,
            49 => Capability::StorageImageExtendedFormats,
            50 => Capability::ImageQuery,
            51 => Capability::DerivativeControl,
            52 => Capability::InterpolationFunction,
            53 => Capability::TransformFeedback,
            54 => Capability::GeometryStreams,
            55 => Capability::StorageImageReadWithoutFormat,
            56 => Capability::StorageImageWriteWithoutFormat,
            57 => Capability::MultiViewport,
            id => return Err(ReadError::UnknownCapability(id)),
        };
        Ok((block, capability))
    }
}

impl MemoryBlockRead for Signedness {
    fn read(block: MemoryBlock) -> MemoryBlockResult<Signedness> {
        let (block, word) = try!(block.read_word());
        let signedness = match word {
            0 => Signedness::UnsignedOrNone,
            1 => Signedness::Signed,
            id => return Err(ReadError::UnknownSignedness(id)),
        };
        Ok((block, signedness))
    }
}

impl MemoryBlockRead for Dim {
    fn read(block: MemoryBlock) -> MemoryBlockResult<Dim> {
        let (block, word) = try!(block.read_word());
        let dim = match word {
            0 => Dim::Tex1D,
            1 => Dim::Tex2D,
            2 => Dim::Tex3D,
            3 => Dim::Cube,
            4 => Dim::Rect,
            5 => Dim::Buffer,
            6 => Dim::SubpassData,
            n => return Err(ReadError::UnknownDim(n)),
        };
        Ok((block, dim))
    }
}

impl MemoryBlockRead for DepthStatus {
    fn read(block: MemoryBlock) -> MemoryBlockResult<DepthStatus> {
        let (block, word) = try!(block.read_word());
        let ds = match word {
            0 => DepthStatus::NotDepth,
            1 => DepthStatus::Depth,
            2 => DepthStatus::NoIndication,
            n => return Err(ReadError::UnknownDepthStatus(n)),
        };
        Ok((block, ds))
    }
}

impl MemoryBlockRead for Arrayed {
    fn read(block: MemoryBlock) -> MemoryBlockResult<Arrayed> {
        let (block, word) = try!(block.read_word());
        let arrayed = match word {
            0 => Arrayed::False,
            1 => Arrayed::True,
            n => return Err(ReadError::UnknownArrayed(n)),
        };
        Ok((block, arrayed))
    }
}

impl MemoryBlockRead for MS {
    fn read(block: MemoryBlock) -> MemoryBlockResult<MS> {
        let (block, word) = try!(block.read_word());
        let ms = match word {
            0 => MS::Single,
            1 => MS::Multi,
            n => return Err(ReadError::UnknownMS(n)),
        };
        Ok((block, ms))
    }
}

impl MemoryBlockRead for SampledStatus {
    fn read(block: MemoryBlock) -> MemoryBlockResult<SampledStatus> {
        let (block, word) = try!(block.read_word());
        let ss = match word {
            0 => SampledStatus::RuntimeChoice,
            1 => SampledStatus::WithSampler,
            2 => SampledStatus::WithoutSampler,
            n => return Err(ReadError::UnknownSampledStatus(n)),
        };
        Ok((block, ss))
    }
}

impl MemoryBlockRead for ImageFormat {
    fn read(block: MemoryBlock) -> MemoryBlockResult<ImageFormat> {
        let (block, word) = try!(block.read_word());
        let format = match word {
            0 => ImageFormat::Unknown,
            1 => ImageFormat::Rgba32f,
            2 => ImageFormat::Rgba16f,
            3 => ImageFormat::R32f,
            4 => ImageFormat::Rgba8,
            5 => ImageFormat::Rgba8Snorm,
            6 => ImageFormat::Rg32f,
            7 => ImageFormat::Rg16f,
            8 => ImageFormat::R11fG11fB10f,
            9 => ImageFormat::R16f,
            10 => ImageFormat::Rgba16,
            11 => ImageFormat::Rgb10A2,
            12 => ImageFormat::Rg16,
            13 => ImageFormat::Rg8,
            14 => ImageFormat::R16,
            15 => ImageFormat::R8,
            16 => ImageFormat::Rgba16Snorm,
            17 => ImageFormat::Rg16Snorm,
            18 => ImageFormat::Rg8Snorm,
            19 => ImageFormat::R16Snorm,
            20 => ImageFormat::R8Snorm,
            21 => ImageFormat::Rgba32i,
            22 => ImageFormat::Rgba16i,
            23 => ImageFormat::Rgba8i,
            24 => ImageFormat::R32i,
            25 => ImageFormat::Rg32i,
            26 => ImageFormat::Rg16i,
            27 => ImageFormat::Rg8i,
            28 => ImageFormat::R16i,
            29 => ImageFormat::R8i,
            30 => ImageFormat::Rgba32ui,
            31 => ImageFormat::Rgba16ui,
            32 => ImageFormat::Rgba8ui,
            33 => ImageFormat::R32ui,
            34 => ImageFormat::Rgb10a2ui,
            35 => ImageFormat::Rg32ui,
            36 => ImageFormat::Rg16ui,
            37 => ImageFormat::Rg8ui,
            38 => ImageFormat::R16ui,
            39 => ImageFormat::R8ui,
            n => return Err(ReadError::UnknownImageFormat(n)),
        };
        Ok((block, format))
    }
}

impl MemoryBlockRead for AccessQualifier {
    fn read(block: MemoryBlock) -> MemoryBlockResult<AccessQualifier> {
        let (block, word) = try!(block.read_word());
        let access_qualifier = match word {
            0 => AccessQualifier::ReadOnly,
            1 => AccessQualifier::WriteOnly,
            2 => AccessQualifier::ReadWrite,
            n => return Err(ReadError::UnknownAccessQualifier(n)),
        };
        Ok((block, access_qualifier))
    }
}

impl MemoryBlockRead for StorageClass {
    fn read(block: MemoryBlock) -> MemoryBlockResult<StorageClass> {
        let (block, word) = try!(block.read_word());
        let storage_class = match word {
            0 => StorageClass::UniformConstant,
            1 => StorageClass::Input,
            2 => StorageClass::Uniform,
            3 => StorageClass::Output,
            4 => StorageClass::Workgroup,
            5 => StorageClass::CrossWorkgroup,
            6 => StorageClass::Private,
            7 => StorageClass::Function,
            8 => StorageClass::Generic,
            9 => StorageClass::PushConstant,
            10 => StorageClass::AtomicCounter,
            11 => StorageClass::Image,
            n => return Err(ReadError::UnknownStorageClass(n)),
        };
        Ok((block, storage_class))
    }
}

impl MemoryBlockRead for FunctionControl {
    fn read(block: MemoryBlock) -> MemoryBlockResult<FunctionControl> {
        let (block, value) = try!(block.read_word());
        let function_control = FunctionControl {
            inline: (value & 0x1) != 0,
            dont_inline: (value & 0x2) != 0,
            pure_function: (value & 0x4) != 0,
            const_function: (value & 0x8) != 0,
        };
        Ok((block, function_control))
    }
}

impl MemoryBlockRead for MemoryAccess {
    fn read(block: MemoryBlock) -> MemoryBlockResult<MemoryAccess> {
        let (block, memory_access_word) = try!(block.read_word());
        if (memory_access_word & 0xFFF8) != 0 {
            return Err(ReadError::UnknownMemoryAccess(memory_access_word));
        }
        let memory_access = MemoryAccess {
            volatile: (memory_access_word & 0x1) != 0,
            aligned: (memory_access_word & 0x2) != 0,
            non_temporal: (memory_access_word & 0x4) != 0,
        };
        Ok((block, memory_access))
    }
}

impl MemoryBlockRead for BuiltIn {
    fn read(block: MemoryBlock) -> MemoryBlockResult<BuiltIn> {
        let (block, word) = try!(block.read_word());
        let builtin = match word {
            0 => BuiltIn::Position,
            1 => BuiltIn::PointSize,
            3 => BuiltIn::ClipDistance,
            4 => BuiltIn::CullDistance,
            5 => BuiltIn::VertexId,
            6 => BuiltIn::InstanceId,
            7 => BuiltIn::PrimitiveId,
            8 => BuiltIn::InvocationId,
            9 => BuiltIn::Layer,
            10 => BuiltIn::ViewportIndex,
            11 => BuiltIn::TessLevelOuter,
            12 => BuiltIn::TessLevelInner,
            13 => BuiltIn::TessCoord,
            14 => BuiltIn::PatchVerticies,
            15 => BuiltIn::FragCoord,
            16 => BuiltIn::PointCoord,
            17 => BuiltIn::FrontFacing,
            18 => BuiltIn::SampleId,
            19 => BuiltIn::SamplePosition,
            20 => BuiltIn::SampleMask,
            22 => BuiltIn::FragDepth,
            23 => BuiltIn::HelperInvocation,
            24 => BuiltIn::NumWorkgroups,
            25 => BuiltIn::WorkgroupSize,
            26 => BuiltIn::WorkgroupId,
            27 => BuiltIn::LocalInvocationId,
            28 => BuiltIn::GlobalInvocationId,
            29 => BuiltIn::LocalInvocationIndex,
            30 => BuiltIn::WorkDim,
            31 => BuiltIn::GlobalSize,
            32 => BuiltIn::EnqueuedWorkgroupSize,
            33 => BuiltIn::GlobalOffset,
            34 => BuiltIn::GlobalLinearId,
            36 => BuiltIn::SubgroupSize,
            37 => BuiltIn::SubgroupMaxSize,
            38 => BuiltIn::NumSubgroups,
            39 => BuiltIn::NumEnqueuedSubgroups,
            40 => BuiltIn::SubgroupId,
            41 => BuiltIn::SubgroupLocalInvocationId,
            42 => BuiltIn::VertexIndex,
            43 => BuiltIn::InstanceIndex,
            id => return Err(ReadError::UnknownBuiltIn(id)),
        };
        Ok((block, builtin))
    }
}

impl MemoryBlockRead for FunctionParameterAttribute {
    fn read(block: MemoryBlock) -> MemoryBlockResult<FunctionParameterAttribute> {
        let (block, word) = try!(block.read_word());
        let function_parameter_attribute = match word {
            0 => FunctionParameterAttribute::Zext,
            1 => FunctionParameterAttribute::Sext,
            2 => FunctionParameterAttribute::ByVal,
            3 => FunctionParameterAttribute::Sret,
            4 => FunctionParameterAttribute::NoAlias,
            5 => FunctionParameterAttribute::NoCapture,
            6 => FunctionParameterAttribute::NoWrite,
            7 => FunctionParameterAttribute::NoReadWrite,
            id => return Err(ReadError::UnknownFunctionParameterAttribute(id)),
        };
        Ok((block, function_parameter_attribute))
    }
}

impl MemoryBlockRead for FpRoundingMode {
    fn read(block: MemoryBlock) -> MemoryBlockResult<FpRoundingMode> {
        let (block, word) = try!(block.read_word());
        let rounding_mode = match word {
            0 => FpRoundingMode::Rte,
            1 => FpRoundingMode::Rtz,
            2 => FpRoundingMode::Rtp,
            3 => FpRoundingMode::Rtn,
            id => return Err(ReadError::UnknownFpRoundingMode(id)),
        };
        Ok((block, rounding_mode))
    }
}

impl MemoryBlockRead for FpFastMathMode {
    fn read(block: MemoryBlock) -> MemoryBlockResult<FpFastMathMode> {
        let (block, id) = try!(block.read_word());
        if (id & 0xFFC0) != 0 {
            return Err(ReadError::UnknownFpFastMathMode(id));
        }
        let fast_math_mode = FpFastMathMode {
            not_nan: id & 0x1 != 0,
            not_inf: id & 0x2 != 0,
            nsz: id & 0x4 != 0,
            allow_recip: id & 0x8 != 0,
            fast: id & 0x10 != 0,
        };
        Ok((block, fast_math_mode))
    }
}

impl MemoryBlockRead for LinkageType {
    fn read(block: MemoryBlock) -> MemoryBlockResult<LinkageType> {
        let (block, word) = try!(block.read_word());
        let lt = match word {
            0 => LinkageType::Export,
            1 => LinkageType::Import,
            id => return Err(ReadError::UnknownLinkageType(id)),
        };
        Ok((block, lt))
    }
}

impl MemoryBlockRead for Decoration {
    fn read(block: MemoryBlock) -> MemoryBlockResult<Decoration> {
        let (block, word) = try!(block.read_word());
        let decoration = match word {
            0 => Decoration::RelaxedPrecision,
            1 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::SpecId(num)));
            }
            2 => Decoration::Block,
            3 => Decoration::BufferBlock,
            4 => Decoration::RowMajor,
            5 => Decoration::ColMajor,
            6 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::ArrayStride(num)));
            }
            7 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::MatrixStride(num)));
            }
            8 => Decoration::GlslShared,
            9 => Decoration::GlslPacked,
            10 => Decoration::CPacked,
            11 => {
                let (block, num) = try!(<BuiltIn as MemoryBlockRead>::read(block));
                return Ok((block, Decoration::BuiltIn(num)));
            }
            13 => Decoration::NoPerspective,
            14 => Decoration::Flat,
            15 => Decoration::Patch,
            16 => Decoration::Centroid,
            17 => Decoration::Sample,
            18 => Decoration::Invariant,
            19 => Decoration::Restrict,
            20 => Decoration::Aliased,
            21 => Decoration::Volatile,
            22 => Decoration::Constant,
            23 => Decoration::Coherent,
            24 => Decoration::NonWritable,
            25 => Decoration::NonReadable,
            26 => Decoration::Uniform,
            28 => Decoration::SaturatedConversion,
            29 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::Stream(num)));
            }
            30 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::Location(num)));
            }
            31 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::Component(num)));
            }
            32 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::Index(num)));
            }
            33 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::Binding(num)));
            }
            34 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::DescriptorSet(num)));
            }
            35 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::Offset(num)));
            }
            36 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::XfbBuffer(num)));
            }
            37 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::XfbStride(num)));
            }
            38 => {
                let (block, item) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::FuncParamAttr(item)));
            }
            39 => {
                let (block, item) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::FpRoundingMode(item)));
            }
            40 => {
                let (block, item) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::FpFastMathMode(item)));
            }
            41 => {
                let (block, name) = try!(MemoryBlockRead::read(block));
                let (block, linkage_type) = try!(<LinkageType as MemoryBlockRead>::read(block));
                return Ok((block, Decoration::LinkageAttributes(name, linkage_type)));
            }
            42 => Decoration::NoContraction,
            43 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::InputAttachmentIndex(num)));
            }
            44 => {
                let (block, num) = try!(MemoryBlockRead::read(block));
                return Ok((block, Decoration::Alignment(num)));
            }
            id => return Err(ReadError::UnknownDecoration(id)),
        };
        Ok((block, decoration))
    }
}

impl MemoryBlockRead for LoopControl {
    fn read(block: MemoryBlock) -> MemoryBlockResult<LoopControl> {
        let (block, word) = try!(block.read_word());
        if word & 0xFFF0 != 0 {
            return Err(ReadError::UnknownLoopControl(word));
        }
        let dependency_length_val = (word & 0x8) != 0;
        let (block, dependency_length) = if dependency_length_val {
            let (block, num) = try!(block.read_word());
            (block, Some(num))
        } else {
            (block, None)
        };
        let loop_control = LoopControl {
            unroll: (word & 0x1) != 0,
            dont_unroll: (word & 0x2) != 0,
            dependency_infinite: (word & 0x4) != 0,
            dependency_length: dependency_length,
        };
        Ok((block, loop_control))
    }
}

impl MemoryBlockRead for SelectionControl {
    fn read(block: MemoryBlock) -> MemoryBlockResult<SelectionControl> {
        let (block, word) = try!(block.read_word());
        if word & 0xFFFC != 0 {
            return Err(ReadError::UnknownSelectionControl(word));
        }
        let selection_control = SelectionControl {
            flatten: (word & 0x1) != 0,
            dont_flatten: (word & 0x2) != 0,
        };
        Ok((block, selection_control))
    }
}

impl MemoryBlockRead for BranchWeights {
    fn read(block: MemoryBlock) -> MemoryBlockResult<BranchWeights> {
        let (block, w1) = try!(block.read_word());
        let (block, w2) = try!(block.read_word());
        let branch_weights = BranchWeights {
            true_weight: w1,
            false_weight: w2,
        };
        Ok((block, branch_weights))
    }
}

impl MemoryBlockRead for PhiArg {
    fn read(block: MemoryBlock) -> MemoryBlockResult<PhiArg> {
        let (block, variable) = try!(MemoryBlockRead::read(block));
        let (block, parent) = try!(MemoryBlockRead::read(block));
        let phi_arg = PhiArg {
            variable: variable,
            parent: parent,
        };
        Ok((block, phi_arg))
    }
}
