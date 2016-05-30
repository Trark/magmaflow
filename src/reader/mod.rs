
use spv::*;
use spv::op::*;
use spv::types::*;
use byteorder::{LittleEndian, BigEndian, ByteOrder};

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
}

pub type ReadResult<T> = Result<T, ReadError>;

/// Magic number for a SPIR-V module
const SPIRV_MAGIC_NUMBER: u32 = 0x07230203;
/// Magic number for a SPIR-V module if the endianness is flipped
const SPIRV_MAGIC_NUMBER_OTHER_ENDIAN: u32 = 0x03022307;

pub fn read_module<'a>(data: &'a [u8]) -> ReadResult<RawModule> {
    let mut stream = Stream::new(data);

    let magic = try!(stream.read_word());
    match magic {
        SPIRV_MAGIC_NUMBER => {}
        SPIRV_MAGIC_NUMBER_OTHER_ENDIAN => stream.invert_endianness(),
        _ => return Err(ReadError::BadMagic),
    }

    let ver_word = try!(stream.read_word());
    let ver_high = (ver_word >> 24) as u8;
    let ver_major = ((ver_word >> 16) & 0xF) as u8;
    let ver_minor = ((ver_word >> 8) & 0xF) as u8;
    let ver_low = (ver_word & 0xF) as u8;
    let version = match (ver_high, ver_major, ver_minor, ver_low) {
        (0, 1, 0, 0) => Version(ver_major, ver_minor),
        (v3, v2, v1, v0) => return Err(ReadError::UnknownVersionBytes(v3, v2, v1, v0)),
    };
    match version {
        Version(1, 0) => {}
        v => return Err(ReadError::UnknownVersion(v)),
    };

    let generator_word = try!(stream.read_word());
    let generator_id = (generator_word >> 16) as u16;
    let tool = match generator_id {
        0 => Tool::KhronosReserved,
        1 => Tool::LunarG,
        2 => Tool::Valve,
        3 => Tool::Codeplay,
        4 => Tool::Nvidia,
        5 => Tool::Arm,
        6 => Tool::KhronosLLvmTranslator,
        7 => Tool::KhronosAssembler,
        8 => Tool::KhronosGlslang,
        9 => Tool::Qualcomm,
        10 => Tool::Amd,
        11 => Tool::Intel,
        id => Tool::Other(id),
    };
    let generator_version = (generator_word & 0xFFFF) as u16;
    let generator = Generator {
        tool: tool,
        version: generator_version,
    };

    let bound = try!(stream.read_word());

    let zero = try!(stream.read_word());
    match zero {
        0 => {}
        _ => return Err(ReadError::UnknownReservedHeaderu324),
    }

    let mut instructions = Vec::new();
    while !stream.is_end() {
        let instr = try!(read_instruction(&mut stream));
        instructions.push(instr);
    }

    Ok(RawModule {
        version: version,
        generator: generator,
        bound: bound,
        instructions: instructions,
    })
}

struct Stream<'a> {
    source: &'a [u8],
    is_le: bool,
}

impl<'a> Stream<'a> {
    fn new(source: &'a [u8]) -> Stream<'a> {
        Stream {
            source: source,
            is_le: true,
        }
    }

    fn read_word(&mut self) -> ReadResult<u32> {
        if self.source.len() >= 4 {
            let result = if self.is_le {
                LittleEndian::read_u32(self.source)
            } else {
                BigEndian::read_u32(self.source)
            };
            self.source = &self.source[4..];
            Ok(result)
        } else if self.source.len() > 0 {
            Err(ReadError::UnexpectedStreamAlignment)
        } else {
            Err(ReadError::UnexpectedEndOfStream)
        }
    }

    fn is_end(&self) -> bool {
        self.source.len() == 0
    }

    fn invert_endianness(&mut self) {
        self.is_le = !self.is_le;
    }
}

fn read_instruction(stream: &mut Stream) -> ReadResult<Core> {
    let head = try!(stream.read_word());
    let id = (head & 0xFFFF) as u16;
    let wc = (head >> 16) as u16;
    let mut words = Vec::with_capacity(wc as usize);
    words.push(head);
    for _ in 1..wc {
        words.push(try!(stream.read_word()));
    }
    let mut im = InstructionMemory::new(&words[..words.len()]);
    let inst = try!(match id {
        0 => Ok(Core::OpNop(OpNop)),
        1 => return Err(ReadError::UnimplementedOp("OpUndef")),
        2 => return Err(ReadError::UnimplementedOp("OpSourceContinued")),
        3 => read_op_source(&mut im),
        4 => return Err(ReadError::UnimplementedOp("OpSourceExtension")),
        5 => read_op_name(&mut im),
        6 => read_op_member_name(&mut im),
        7 => return Err(ReadError::UnimplementedOp("OpString")),
        8 => return Err(ReadError::UnimplementedOp("OpLine")),
        10 => read_op_extension(&mut im),
        11 => read_op_ext_inst_import(&mut im),
        12 => return Err(ReadError::UnimplementedOp("OpExtInst")),
        14 => read_op_memory_model(&mut im),
        15 => read_op_entry_point(&mut im),
        16 => read_op_execution_mode(&mut im),
        17 => read_op_capability(&mut im),
        19 => read_op_type_void(&mut im),
        20 => read_op_type_bool(&mut im),
        21 => read_op_type_int(&mut im),
        22 => read_op_type_float(&mut im),
        23 => read_op_type_vector(&mut im),
        24 => read_op_type_matrix(&mut im),
        25 => read_op_type_image(&mut im),
        26 => read_op_type_sampler(&mut im),
        27 => read_op_type_sampled_image(&mut im),
        28 => read_op_type_array(&mut im),
        29 => read_op_type_runtime_array(&mut im),
        30 => read_op_type_struct(&mut im),
        31 => read_op_type_opaque(&mut im),
        32 => read_op_type_pointer(&mut im),
        33 => read_op_type_function(&mut im),
        41 => return Err(ReadError::UnimplementedOp("OpConstantTrue")),
        42 => return Err(ReadError::UnimplementedOp("OpConstantFalse")),
        43 => read_op_constant(&mut im),
        44 => read_op_constant_composite(&mut im),
        45 => return Err(ReadError::UnimplementedOp("OpConstantSampler")),
        46 => return Err(ReadError::UnimplementedOp("OpConstantNull")),
        48 => return Err(ReadError::UnimplementedOp("OpSpecConstantTrue")),
        49 => return Err(ReadError::UnimplementedOp("OpSpecConstantFalse")),
        50 => return Err(ReadError::UnimplementedOp("OpSpecConstant")),
        51 => return Err(ReadError::UnimplementedOp("OpSpecConstantComposite")),
        52 => return Err(ReadError::UnimplementedOp("OpSpecConstantOp")),
        54 => read_op_function(&mut im),
        55 => read_op_function_parameter(&mut im),
        56 => read_op_function_end(&mut im),
        57 => return Err(ReadError::UnimplementedOp("OpFunctionCall")),
        59 => read_op_variable(&mut im),
        60 => return Err(ReadError::UnimplementedOp("OpImageTexelPointer")),
        61 => read_op_load(&mut im),
        62 => read_op_store(&mut im),
        63 => return Err(ReadError::UnimplementedOp("OpCopyMemory")),
        64 => return Err(ReadError::UnimplementedOp("OpCopyMemorySized")),
        65 => read_op_access_chain(&mut im),
        66 => return Err(ReadError::UnimplementedOp("OpInBoundsAccessChain")),
        67 => return Err(ReadError::UnimplementedOp("OpPtrAccessChain")),
        68 => return Err(ReadError::UnimplementedOp("OpArrayLength")),
        69 => return Err(ReadError::UnimplementedOp("OpGenericPtrMemSemantics")),
        70 => return Err(ReadError::UnimplementedOp("OpInBoundsPtrAccessChain")),
        71 => read_op_decorate(&mut im),
        72 => read_op_member_decorate(&mut im),
        77 => return Err(ReadError::UnimplementedOp("OpVectorExtractDynamic")),
        78 => return Err(ReadError::UnimplementedOp("OpVectorInsertDynamic")),
        79 => return Err(ReadError::UnimplementedOp("OpVectorShuffle")),
        80 => return Err(ReadError::UnimplementedOp("OpCompositeConstruct")),
        81 => return Err(ReadError::UnimplementedOp("OpCompositeExtract")),
        82 => return Err(ReadError::UnimplementedOp("OpCompositeInsert")),
        83 => return Err(ReadError::UnimplementedOp("OpCopyObject")),
        84 => return Err(ReadError::UnimplementedOp("OpTranspose")),
        86 => return Err(ReadError::UnimplementedOp("OpSampledImage")),
        87 => return Err(ReadError::UnimplementedOp("OpImageSampleImplicitLod")),
        88 => return Err(ReadError::UnimplementedOp("OpImageSampleExplicitLod")),
        89 => return Err(ReadError::UnimplementedOp("OpImageSampleDrefImplicitLod")),
        90 => return Err(ReadError::UnimplementedOp("OpImageSampleDrefExplicitLod")),
        91 => return Err(ReadError::UnimplementedOp("OpImageSampleProjImplicitLod")),
        92 => return Err(ReadError::UnimplementedOp("OpImageSampleProjExplicitLod")),
        93 => return Err(ReadError::UnimplementedOp("OpImageSampleProjDrefImplicitLod")),
        94 => return Err(ReadError::UnimplementedOp("OpImageSampleProjDrefExplicitLod")),
        95 => return Err(ReadError::UnimplementedOp("OpImageFetch")),
        96 => return Err(ReadError::UnimplementedOp("OpImageGather")),
        97 => return Err(ReadError::UnimplementedOp("OpImageDrefGather")),
        98 => return Err(ReadError::UnimplementedOp("OpImageRead")),
        99 => return Err(ReadError::UnimplementedOp("OpImageWrite")),
        100 => return Err(ReadError::UnimplementedOp("OpImage")),
        101 => return Err(ReadError::UnimplementedOp("OpImageQueryFormat")),
        102 => return Err(ReadError::UnimplementedOp("OpImageQueryOrder")),
        103 => return Err(ReadError::UnimplementedOp("OpImageQuerySizeLod")),
        104 => return Err(ReadError::UnimplementedOp("OpImageQuerySize")),
        105 => return Err(ReadError::UnimplementedOp("OpImageQueryLod")),
        106 => return Err(ReadError::UnimplementedOp("OpImageQueryLevels")),
        107 => return Err(ReadError::UnimplementedOp("OpImageQuerySamples")),
        109 => return Err(ReadError::UnimplementedOp("OpConvertFToU")),
        110 => return Err(ReadError::UnimplementedOp("OpConvertFToS")),
        111 => return Err(ReadError::UnimplementedOp("OpConvertSToF")),
        112 => read_op_convert_utof(&mut im),
        113 => return Err(ReadError::UnimplementedOp("OpUConvert")),
        114 => return Err(ReadError::UnimplementedOp("OpSConvert")),
        115 => return Err(ReadError::UnimplementedOp("OpFConvert")),
        116 => return Err(ReadError::UnimplementedOp("OpQuantizeToF16")),
        117 => return Err(ReadError::UnimplementedOp("OpConvertPtrToU")),
        118 => return Err(ReadError::UnimplementedOp("OpSatConvertSToU")),
        119 => return Err(ReadError::UnimplementedOp("OpSatConvertUToS")),
        120 => return Err(ReadError::UnimplementedOp("OpConvertUToPtr")),
        121 => return Err(ReadError::UnimplementedOp("OpPtrCastToGeneric")),
        122 => return Err(ReadError::UnimplementedOp("OpGenericCastToPtr")),
        123 => return Err(ReadError::UnimplementedOp("OpGenericCastToPtrExplicit")),
        124 => return Err(ReadError::UnimplementedOp("OpBitcast")),
        126 => return Err(ReadError::UnimplementedOp("OpSNegate")),
        127 => return Err(ReadError::UnimplementedOp("OpFNegate")),
        128 => return Err(ReadError::UnimplementedOp("OpIAdd")),
        129 => return Err(ReadError::UnimplementedOp("OpFAdd")),
        130 => return Err(ReadError::UnimplementedOp("OpISub")),
        131 => return Err(ReadError::UnimplementedOp("OpFSub")),
        132 => read_op_imul(&mut im),
        133 => return Err(ReadError::UnimplementedOp("OpFMul")),
        134 => return Err(ReadError::UnimplementedOp("OpUDiv")),
        135 => return Err(ReadError::UnimplementedOp("OpSDiv")),
        136 => return Err(ReadError::UnimplementedOp("OpFDiv")),
        137 => return Err(ReadError::UnimplementedOp("OpUMod")),
        138 => return Err(ReadError::UnimplementedOp("OpSRem")),
        139 => return Err(ReadError::UnimplementedOp("OpSMod")),
        140 => return Err(ReadError::UnimplementedOp("OpFRem")),
        141 => return Err(ReadError::UnimplementedOp("OpFMod")),
        142 => return Err(ReadError::UnimplementedOp("OpVectorTimesScalar")),
        143 => return Err(ReadError::UnimplementedOp("OpMatrixTimesScalar")),
        144 => return Err(ReadError::UnimplementedOp("OpVectorTimesMatrix")),
        145 => return Err(ReadError::UnimplementedOp("OpMatrixTimesVector")),
        146 => return Err(ReadError::UnimplementedOp("OpMatrixTimesMatrix")),
        147 => return Err(ReadError::UnimplementedOp("OpOuterProduct")),
        148 => return Err(ReadError::UnimplementedOp("OpDot")),
        149 => return Err(ReadError::UnimplementedOp("OpIAddCarry")),
        150 => return Err(ReadError::UnimplementedOp("OpISubBorrow")),
        151 => return Err(ReadError::UnimplementedOp("OpUMulExtended")),
        152 => return Err(ReadError::UnimplementedOp("OpSMulExtended")),
        154 => return Err(ReadError::UnimplementedOp("OpAny")),
        155 => return Err(ReadError::UnimplementedOp("OpAll")),
        156 => return Err(ReadError::UnimplementedOp("OpIsNan")),
        157 => return Err(ReadError::UnimplementedOp("OpIsInf")),
        158 => return Err(ReadError::UnimplementedOp("OpIsFinite")),
        159 => return Err(ReadError::UnimplementedOp("OpIsNormal")),
        160 => return Err(ReadError::UnimplementedOp("OpSignBitSet")),
        161 => return Err(ReadError::UnimplementedOp("OpLessOrGreater")),
        162 => return Err(ReadError::UnimplementedOp("OpOrdered")),
        163 => return Err(ReadError::UnimplementedOp("OpUnordered")),
        164 => return Err(ReadError::UnimplementedOp("OpLogicalEqual")),
        165 => return Err(ReadError::UnimplementedOp("OpLogicalNotEqual")),
        166 => return Err(ReadError::UnimplementedOp("OpLogicalOr")),
        167 => return Err(ReadError::UnimplementedOp("OpLogicalAnd")),
        168 => return Err(ReadError::UnimplementedOp("OpLogicalNot")),
        169 => return Err(ReadError::UnimplementedOp("OpSelect")),
        170 => return Err(ReadError::UnimplementedOp("OpIEqual")),
        171 => return Err(ReadError::UnimplementedOp("OpINotEqual")),
        172 => return Err(ReadError::UnimplementedOp("OpUGreaterThan")),
        173 => return Err(ReadError::UnimplementedOp("OpSGreaterThan")),
        174 => return Err(ReadError::UnimplementedOp("OpUGreaterThanEqual")),
        175 => return Err(ReadError::UnimplementedOp("OpSGreaterThanEqual")),
        176 => return Err(ReadError::UnimplementedOp("OpULessThan")),
        177 => return Err(ReadError::UnimplementedOp("OpSLessThan")),
        178 => return Err(ReadError::UnimplementedOp("OpULessThanEqual")),
        179 => return Err(ReadError::UnimplementedOp("OpSLessThanEqual")),
        180 => return Err(ReadError::UnimplementedOp("OpFOrdEqual")),
        181 => return Err(ReadError::UnimplementedOp("OpFUnordEqual")),
        182 => return Err(ReadError::UnimplementedOp("OpFOrdNotEqual")),
        183 => return Err(ReadError::UnimplementedOp("OpFUnordNotEqual")),
        184 => return Err(ReadError::UnimplementedOp("OpFOrdLessThan")),
        185 => return Err(ReadError::UnimplementedOp("OpFUnordLessThan")),
        186 => return Err(ReadError::UnimplementedOp("OpFOrdGreaterThan")),
        187 => return Err(ReadError::UnimplementedOp("OpFUnordGreaterThan")),
        188 => return Err(ReadError::UnimplementedOp("OpFOrdLessThanEqual")),
        189 => return Err(ReadError::UnimplementedOp("OpFUnordLessThanEqual")),
        190 => return Err(ReadError::UnimplementedOp("OpFOrdGreaterThanEqual")),
        191 => return Err(ReadError::UnimplementedOp("OpFUnordGreaterThanEqual")),
        194 => return Err(ReadError::UnimplementedOp("OpShiftRightLogical")),
        195 => return Err(ReadError::UnimplementedOp("OpShiftRightArithmetic")),
        196 => return Err(ReadError::UnimplementedOp("OpShiftLeftLogical")),
        197 => return Err(ReadError::UnimplementedOp("OpBitwiseOr")),
        198 => return Err(ReadError::UnimplementedOp("OpBitwiseXor")),
        199 => return Err(ReadError::UnimplementedOp("OpBitwiseAnd")),
        200 => return Err(ReadError::UnimplementedOp("OpNot")),
        201 => return Err(ReadError::UnimplementedOp("OpBitFieldInsert")),
        202 => return Err(ReadError::UnimplementedOp("OpBitFieldSExtract")),
        203 => return Err(ReadError::UnimplementedOp("OpBitFieldUExtract")),
        204 => return Err(ReadError::UnimplementedOp("OpBitReverse")),
        205 => return Err(ReadError::UnimplementedOp("OpBitCount")),
        207 => return Err(ReadError::UnimplementedOp("OpDPdx")),
        208 => return Err(ReadError::UnimplementedOp("OpDPdy")),
        209 => return Err(ReadError::UnimplementedOp("OpFwidth")),
        210 => return Err(ReadError::UnimplementedOp("OpDPdxFine")),
        211 => return Err(ReadError::UnimplementedOp("OpDPdyFine")),
        212 => return Err(ReadError::UnimplementedOp("OpFwidthFine")),
        213 => return Err(ReadError::UnimplementedOp("OpDPdxCoarse")),
        214 => return Err(ReadError::UnimplementedOp("OpDPdyCoarse")),
        215 => return Err(ReadError::UnimplementedOp("OpFwidthCoarse")),
        218 => return Err(ReadError::UnimplementedOp("OpEmitVertex")),
        219 => return Err(ReadError::UnimplementedOp("OpEndPrimitive")),
        220 => return Err(ReadError::UnimplementedOp("OpEmitStreamVertex")),
        221 => return Err(ReadError::UnimplementedOp("OpEndStreamPrimitive")),
        224 => return Err(ReadError::UnimplementedOp("OpControlBarrier")),
        225 => return Err(ReadError::UnimplementedOp("OpMemoryBarrier")),
        227 => return Err(ReadError::UnimplementedOp("OpAtomicLoad")),
        228 => return Err(ReadError::UnimplementedOp("OpAtomicStore")),
        229 => return Err(ReadError::UnimplementedOp("OpAtomicExchange")),
        230 => return Err(ReadError::UnimplementedOp("OpAtomicCompareExchange")),
        231 => return Err(ReadError::UnimplementedOp("OpAtomicCompareExchangeWeak")),
        232 => return Err(ReadError::UnimplementedOp("OpAtomicIIncrement")),
        233 => return Err(ReadError::UnimplementedOp("OpAtomicIDecrement")),
        234 => return Err(ReadError::UnimplementedOp("OpAtomicIAdd")),
        235 => return Err(ReadError::UnimplementedOp("OpAtomicISub")),
        236 => return Err(ReadError::UnimplementedOp("OpAtomicSMin")),
        237 => return Err(ReadError::UnimplementedOp("OpAtomicUMin")),
        238 => return Err(ReadError::UnimplementedOp("OpAtomicSMax")),
        239 => return Err(ReadError::UnimplementedOp("OpAtomicUMax")),
        240 => return Err(ReadError::UnimplementedOp("OpAtomicAnd")),
        241 => return Err(ReadError::UnimplementedOp("OpAtomicOr")),
        242 => return Err(ReadError::UnimplementedOp("OpAtomicXor")),
        245 => return Err(ReadError::UnimplementedOp("OpPhi")),
        246 => return Err(ReadError::UnimplementedOp("OpLoopMerge")),
        247 => return Err(ReadError::UnimplementedOp("OpSelectionMerge")),
        248 => read_op_label(&mut im),
        249 => read_op_branch(&mut im),
        250 => return Err(ReadError::UnimplementedOp("OpBranchConditional")),
        251 => return Err(ReadError::UnimplementedOp("OpSwitch")),
        252 => return Err(ReadError::UnimplementedOp("OpKill")),
        253 => read_op_return(&mut im),
        254 => return Err(ReadError::UnimplementedOp("OpReturnValue")),
        255 => return Err(ReadError::UnimplementedOp("OpUnreachable")),
        256 => return Err(ReadError::UnimplementedOp("OpLifetimeStart")),
        257 => return Err(ReadError::UnimplementedOp("OpLifetimeStop")),
        259 => return Err(ReadError::UnimplementedOp("OpGroupAsyncCopy")),
        260 => return Err(ReadError::UnimplementedOp("OpGroupWaitEvents")),
        261 => return Err(ReadError::UnimplementedOp("OpGroupAll")),
        262 => return Err(ReadError::UnimplementedOp("OpGroupAny")),
        263 => return Err(ReadError::UnimplementedOp("OpGroupBroadcast")),
        264 => return Err(ReadError::UnimplementedOp("OpGroupIAdd")),
        265 => return Err(ReadError::UnimplementedOp("OpGroupFAdd")),
        266 => return Err(ReadError::UnimplementedOp("OpGroupFMin")),
        267 => return Err(ReadError::UnimplementedOp("OpGroupUMin")),
        268 => return Err(ReadError::UnimplementedOp("OpGroupSMin")),
        269 => return Err(ReadError::UnimplementedOp("OpGroupFMax")),
        270 => return Err(ReadError::UnimplementedOp("OpGroupUMax")),
        271 => return Err(ReadError::UnimplementedOp("OpGroupSMax")),
        274 => return Err(ReadError::UnimplementedOp("OpReadPipe")),
        275 => return Err(ReadError::UnimplementedOp("OpWritePipe")),
        276 => return Err(ReadError::UnimplementedOp("OpReservedReadPipe")),
        277 => return Err(ReadError::UnimplementedOp("OpReservedWritePipe")),
        278 => return Err(ReadError::UnimplementedOp("OpReserveReadPipePackets")),
        279 => return Err(ReadError::UnimplementedOp("OpReserveWritePipePackets")),
        280 => return Err(ReadError::UnimplementedOp("OpCommitReadPipe")),
        281 => return Err(ReadError::UnimplementedOp("OpCommitWritePipe")),
        282 => return Err(ReadError::UnimplementedOp("OpIsValidReserveId")),
        283 => return Err(ReadError::UnimplementedOp("OpGetNumPipePackets")),
        284 => return Err(ReadError::UnimplementedOp("OpGetMaxPipePackets")),
        285 => return Err(ReadError::UnimplementedOp("OpGroupReserveReadPipePackets")),
        286 => return Err(ReadError::UnimplementedOp("OpGroupReserveWritePipePackets")),
        287 => return Err(ReadError::UnimplementedOp("OpGroupCommitReadPipe")),
        288 => return Err(ReadError::UnimplementedOp("OpGroupCommitWritePipe")),
        291 => return Err(ReadError::UnimplementedOp("OpEnqueueMarker")),
        292 => return Err(ReadError::UnimplementedOp("OpEnqueueKernel")),
        293 => return Err(ReadError::UnimplementedOp("OpGetKernelNDrangeSubGroupCount")),
        294 => return Err(ReadError::UnimplementedOp("OpGetKernelNDrangeMaxSubGroupSize")),
        295 => return Err(ReadError::UnimplementedOp("OpGetKernelWorkGroupSize")),
        296 => return Err(ReadError::UnimplementedOp("OpGetKernelPreferredWorkGroupSizeMultiple")),
        297 => return Err(ReadError::UnimplementedOp("OpRetainEvent")),
        298 => return Err(ReadError::UnimplementedOp("OpReleaseEvent")),
        299 => return Err(ReadError::UnimplementedOp("OpCreateUserEvent")),
        300 => return Err(ReadError::UnimplementedOp("OpIsValidEvent")),
        301 => return Err(ReadError::UnimplementedOp("OpSetUserEventStatus")),
        302 => return Err(ReadError::UnimplementedOp("OpCaptureEventProfilingInfo")),
        303 => return Err(ReadError::UnimplementedOp("OpGetDefaultQueue")),
        304 => return Err(ReadError::UnimplementedOp("OpBuildNDRange")),
        305 => return Err(ReadError::UnimplementedOp("OpImageSparseSampleImplicitLod")),
        306 => return Err(ReadError::UnimplementedOp("OpImageSparseSampleExplicitLod")),
        307 => return Err(ReadError::UnimplementedOp("OpImageSparseSampleDrefImplicitLod")),
        308 => return Err(ReadError::UnimplementedOp("OpImageSparseSampleDrefExplicitLod")),
        309 => return Err(ReadError::UnimplementedOp("OpImageSparseSampleProjImplicitLod")),
        310 => return Err(ReadError::UnimplementedOp("OpImageSparseSampleProjExplicitLod")),
        311 => return Err(ReadError::UnimplementedOp("OpImageSparseSampleProjDrefImplicitLod")),
        312 => return Err(ReadError::UnimplementedOp("OpImageSparseSampleProjDrefExplicitLod")),
        313 => return Err(ReadError::UnimplementedOp("OpImageSparseFetch")),
        314 => return Err(ReadError::UnimplementedOp("OpImageSparseGather")),
        315 => return Err(ReadError::UnimplementedOp("OpImageSparseDrefGather")),
        316 => return Err(ReadError::UnimplementedOp("OpImageSparseTexelsResident")),
        318 => return Err(ReadError::UnimplementedOp("OpAtomicFlagTestAndSet")),
        319 => return Err(ReadError::UnimplementedOp("OpAtomicFlagClear")),
        320 => return Err(ReadError::UnimplementedOp("OpImageSparseRead")),
        _ => Err(ReadError::UnknownOp(id, wc)),
    });
    try!(im.finish());
    Ok(inst)
}

struct InstructionMemory<'a> {
    block: &'a [u32],
    position: usize,
}

impl<'a> InstructionMemory<'a> {
    fn new(memory: &'a [u32]) -> InstructionMemory<'a> {
        InstructionMemory {
            block: memory,
            position: 1, // First word is code / length
        }
    }

    fn read_next(&mut self) -> ReadResult<u32> {
        if self.position < self.block.len() {
            let word = self.block[self.position];
            self.position = self.position + 1;
            Ok(word)
        } else {
            Err(ReadError::UnexpectedEndOfInstruction)
        }
    }

    fn get_word_count(&self) -> usize {
        self.block.len()
    }

    fn get_remaining_words(&self) -> usize {
        assert!(self.position <= self.block.len());
        self.block.len() - self.position
    }

    fn finish(self) -> ReadResult<()> {
        if self.block.len() != self.position {
            Err(ReadError::InstructionHadExcessData)
        } else {
            Ok(())
        }
    }
}

fn read_op_id(stream: &mut InstructionMemory) -> ReadResult<OpId> {
    Ok(OpId(try!(stream.read_next())))
}

fn read_result_id(stream: &mut InstructionMemory) -> ReadResult<ResultId> {
    Ok(ResultId(try!(stream.read_next())))
}

fn read_op_id_list(stream: &mut InstructionMemory) -> ReadResult<Vec<OpId>> {
    let rem = stream.get_remaining_words();
    let mut ids = Vec::with_capacity(rem);
    for _ in 0..rem {
        ids.push(read_op_id(stream).expect("Op list read should never pass end"));
    }
    Ok(ids)
}

fn read_lit_number_u32(stream: &mut InstructionMemory) -> ReadResult<u32> {
    let word = try!(stream.read_next());
    Ok(word)
}

fn read_string_literal(stream: &mut InstructionMemory) -> ReadResult<LitString> {
    let mut bytes = Vec::with_capacity(16);
    'null: loop {
        let word = try!(stream.read_next());
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
        Ok(s) => Ok(s),
        Err(_) => Err(ReadError::InvalidString),
    }
}

fn read_op_source(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let lang_id = try!(stream.read_next());
    let lang = match lang_id {
        0 => SourceLanguage::Unknown,
        1 => SourceLanguage::Essl,
        2 => SourceLanguage::Glsl,
        3 => SourceLanguage::OpenCL_C,
        4 => SourceLanguage::OpenCL_Cpp,
        id => SourceLanguage::Other(id),
    };
    let version = SourceVersion(try!(read_lit_number_u32(stream)));
    let file_id = if stream.get_remaining_words() > 0 {
        Some(read_op_id(stream).expect("Reading file id after checking bounds"))
    } else {
        None
    };
    let source_name = if stream.get_remaining_words() > 0 {
        Some(read_string_literal(stream).expect("Reading file name after checking bounds"))
    } else {
        None
    };
    Ok(Core::OpSource(OpSource {
        language: lang,
        version: version,
        file: file_id,
        source: source_name,
    }))
}

fn read_op_name(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let id = try!(read_op_id(stream));
    let name = try!(read_string_literal(stream));
    Ok(Core::OpName(OpName {
        target: id,
        name: name,
    }))
}

fn read_op_member_name(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let struct_type = try!(read_op_id(stream));
    let member_index = MemberIndex(try!(read_lit_number_u32(stream)));
    let name = try!(read_string_literal(stream));
    Ok(Core::OpMemberName(OpMemberName {
        struct_type: struct_type,
        member: member_index,
        name: name,
    }))
}

fn read_op_extension(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let name = try!(read_string_literal(stream));
    Ok(Core::OpExtension(OpExtension { name: name }))
}

fn read_op_ext_inst_import(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let name = try!(read_string_literal(stream));
    Ok(Core::OpExtInstImport(OpExtInstImport {
        result_id: result_id,
        name: name,
    }))
}

fn read_addressing_model(stream: &mut InstructionMemory) -> ReadResult<AddressingModel> {
    let am = try!(stream.read_next());
    Ok(match am {
        0 => AddressingModel::Logical,
        1 => AddressingModel::Physical32,
        2 => AddressingModel::Physical64,
        id => return Err(ReadError::UnknownAddressingModel(id)),
    })
}

fn read_memory_model(stream: &mut InstructionMemory) -> ReadResult<MemoryModel> {
    let mm = try!(stream.read_next());
    Ok(match mm {
        0 => MemoryModel::Simple,
        1 => MemoryModel::Glsl450,
        2 => MemoryModel::OpenCL,
        id => return Err(ReadError::UnknownMemoryModel(id)),
    })
}

fn read_op_memory_model(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let am = try!(read_addressing_model(stream));
    let mm = try!(read_memory_model(stream));
    Ok(Core::OpMemoryModel(OpMemoryModel {
        addressing_model: am,
        memory_model: mm,
    }))
}

fn read_execution_model(stream: &mut InstructionMemory) -> ReadResult<ExecutionModel> {
    let em = try!(stream.read_next());
    Ok(match em {
        0 => ExecutionModel::Vertex,
        1 => ExecutionModel::TesselationControl,
        2 => ExecutionModel::TesselationEvaluation,
        3 => ExecutionModel::Geometry,
        4 => ExecutionModel::Fragment,
        5 => ExecutionModel::GlCompute,
        6 => ExecutionModel::Kernel,
        id => return Err(ReadError::UnknownExecutionModel(id)),
    })
}

fn read_op_entry_point(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let execution_model = try!(read_execution_model(stream));
    let entry_point = try!(read_op_id(stream));
    let name = try!(read_string_literal(stream));
    let forward_defs = try!(read_op_id_list(stream));
    let inst = OpEntryPoint {
        execution_model: execution_model,
        entry_point: entry_point,
        name: name,
        interface: forward_defs,
    };
    Ok(Core::OpEntryPoint(inst))
}

fn read_op_execution_mode(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let entry_point = try!(read_op_id(stream));
    let execution_mode_id = try!(stream.read_next());
    let mode = match execution_mode_id {
        0 => {
            let num = try!(read_lit_number_u32(stream));
            ExecutionMode::Invocations(num)
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
            let x = try!(read_lit_number_u32(stream));
            let y = try!(read_lit_number_u32(stream));
            let z = try!(read_lit_number_u32(stream));
            ExecutionMode::LocalSize(x, y, z)
        }
        18 => {
            let x = try!(read_lit_number_u32(stream));
            let y = try!(read_lit_number_u32(stream));
            let z = try!(read_lit_number_u32(stream));
            ExecutionMode::LocalSizeHint(x, y, z)
        }
        19 => ExecutionMode::InputPoints,
        20 => ExecutionMode::InputLines,
        21 => ExecutionMode::InputLinesAdjacency,
        22 => ExecutionMode::Triangles,
        23 => ExecutionMode::InputTrianglesAdjacency,
        24 => ExecutionMode::Quads,
        25 => ExecutionMode::Isolines,
        26 => {
            let num = try!(read_lit_number_u32(stream));
            ExecutionMode::OutputVerticies(num)
        }
        27 => ExecutionMode::OutputPoints,
        28 => ExecutionMode::OutputLineStrip,
        29 => ExecutionMode::OutputTriangleStrip,
        30 => {
            let id = try!(read_op_id(stream));
            ExecutionMode::VecTypeHint(id)
        }
        31 => ExecutionMode::ContractionOff,
        id => return Err(ReadError::UnknownExecutionMode(id)),
    };
    Ok(Core::OpExecutionMode(OpExecutionMode {
        entry_point: entry_point,
        mode: mode,
    }))
}

fn read_op_capability(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let capability_id = try!(stream.read_next());
    let capability = match capability_id {
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
    Ok(Core::OpCapability(OpCapability { capability: capability }))
}

fn read_op_type_void(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    Ok(Core::OpTypeVoid(OpTypeVoid { result_id: result_id }))
}

fn read_op_type_bool(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    Ok(Core::OpTypeBool(OpTypeBool { result_id: result_id }))
}

fn read_op_type_int(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let width = try!(read_lit_number_u32(stream));
    let signedness_u32 = try!(read_lit_number_u32(stream));
    let signedness = match signedness_u32 {
        0 => Signedness::UnsignedOrNone,
        1 => Signedness::Signed,
        n => return Err(ReadError::UnknownSignedness(n)),
    };
    Ok(Core::OpTypeInt(OpTypeInt {
        result_id: result_id,
        width: width,
        signedness: signedness,
    }))
}

fn read_op_type_float(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let width = try!(read_lit_number_u32(stream));
    Ok(Core::OpTypeFloat(OpTypeFloat {
        result_id: result_id,
        width: width,
    }))
}

fn read_op_type_vector(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let component_type = try!(read_op_id(stream));
    let count = try!(read_lit_number_u32(stream));
    Ok(Core::OpTypeVector(OpTypeVector {
        result_id: result_id,
        component_type: component_type,
        component_count: count,
    }))
}

fn read_op_type_matrix(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let column_type = try!(read_op_id(stream));
    let column_count = try!(read_lit_number_u32(stream));
    Ok(Core::OpTypeMatrix(OpTypeMatrix {
        result_id: result_id,
        column_type: column_type,
        column_count: column_count,
    }))
}

fn read_dim(stream: &mut InstructionMemory) -> ReadResult<Dim> {
    Ok(match try!(stream.read_next()) {
        0 => Dim::Tex1D,
        1 => Dim::Tex2D,
        2 => Dim::Tex3D,
        3 => Dim::Cube,
        4 => Dim::Rect,
        5 => Dim::Buffer,
        6 => Dim::SubpassData,
        n => return Err(ReadError::UnknownDim(n)),
    })
}

fn read_depth_status(stream: &mut InstructionMemory) -> ReadResult<DepthStatus> {
    Ok(match try!(stream.read_next()) {
        0 => DepthStatus::NotDepth,
        1 => DepthStatus::Depth,
        2 => DepthStatus::NoIndication,
        n => return Err(ReadError::UnknownDepthStatus(n)),
    })
}

fn read_arrayed(stream: &mut InstructionMemory) -> ReadResult<Arrayed> {
    Ok(match try!(stream.read_next()) {
        0 => Arrayed::False,
        1 => Arrayed::True,
        n => return Err(ReadError::UnknownArrayed(n)),
    })
}

fn read_ms(stream: &mut InstructionMemory) -> ReadResult<MS> {
    Ok(match try!(stream.read_next()) {
        0 => MS::Single,
        1 => MS::Multi,
        n => return Err(ReadError::UnknownMS(n)),
    })
}

fn read_sampled_status(stream: &mut InstructionMemory) -> ReadResult<SampledStatus> {
    Ok(match try!(stream.read_next()) {
        0 => SampledStatus::RuntimeChoice,
        1 => SampledStatus::WithSampler,
        2 => SampledStatus::WithoutSampler,
        n => return Err(ReadError::UnknownSampledStatus(n)),
    })
}

fn read_image_format(stream: &mut InstructionMemory) -> ReadResult<ImageFormat> {
    Ok(match try!(stream.read_next()) {
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
    })
}

fn read_access_qualifier(stream: &mut InstructionMemory) -> ReadResult<AccessQualifier> {
    Ok(match try!(stream.read_next()) {
        0 => AccessQualifier::ReadOnly,
        1 => AccessQualifier::WriteOnly,
        2 => AccessQualifier::ReadWrite,
        n => return Err(ReadError::UnknownAccessQualifier(n)),
    })
}

fn read_op_type_image(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 9 && stream.get_word_count() != 10 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let sampled_type = try!(read_op_id(stream));
    let dim = try!(read_dim(stream));
    let depth = try!(read_depth_status(stream));
    let arrayed = try!(read_arrayed(stream));
    let ms = try!(read_ms(stream));
    let sampled = try!(read_sampled_status(stream));
    let image_format = try!(read_image_format(stream));
    let access_qualifier = if stream.get_remaining_words() > 0 {
        Some(try!(read_access_qualifier(stream)))
    } else {
        None
    };
    Ok(Core::OpTypeImage(OpTypeImage {
        result_id: result_id,
        sampled_type: sampled_type,
        dim: dim,
        depth: depth,
        arrayed: arrayed,
        ms: ms,
        sampled: sampled,
        format: image_format,
        access_qualifier: access_qualifier,
    }))
}

fn read_op_type_sampler(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    Ok(Core::OpTypeSampler(OpTypeSampler { result_id: result_id }))
}

fn read_op_type_sampled_image(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let image_type = try!(read_op_id(stream));
    Ok(Core::OpTypeSampledImage(OpTypeSampledImage {
        result_id: result_id,
        image_type: image_type,
    }))
}

fn read_op_type_array(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let element_type = try!(read_op_id(stream));
    let length = try!(read_op_id(stream));
    Ok(Core::OpTypeArray(OpTypeArray {
        result_id: result_id,
        element_type: element_type,
        length: length,
    }))
}



fn read_op_type_runtime_array(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let element_type = try!(read_op_id(stream));
    Ok(Core::OpTypeRuntimeArray(OpTypeRuntimeArray {
        result_id: result_id,
        element_type: element_type,
    }))
}

fn read_op_type_struct(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let member_types = try!(read_op_id_list(stream));
    Ok(Core::OpTypeStruct(OpTypeStruct {
        result_id: result_id,
        member_types: member_types,
    }))
}

fn read_op_type_opaque(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let name = try!(read_string_literal(stream));
    Ok(Core::OpTypeOpaque(OpTypeOpaque {
        result_id: result_id,
        name: name,
    }))
}

fn read_storage_class(stream: &mut InstructionMemory) -> ReadResult<StorageClass> {
    Ok(match try!(stream.read_next()) {
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
    })
}

fn read_op_type_pointer(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let storage_class = try!(read_storage_class(stream));
    let pointed_type = try!(read_op_id(stream));
    Ok(Core::OpTypePointer(OpTypePointer {
        result_id: result_id,
        storage_class: storage_class,
        pointed_type: pointed_type,
    }))
}

fn read_op_type_function(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let return_type = try!(read_op_id(stream));
    let rem = stream.get_remaining_words();
    let mut param_types = Vec::with_capacity(rem);
    for _ in 0..rem {
        param_types.push(read_op_id(stream).expect("Reading function type arguments"))
    }
    Ok(Core::OpTypeFunction(OpTypeFunction {
        result_id: result_id,
        return_type: return_type,
        parameter_types: param_types,
    }))
}

fn read_op_constant(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type_id = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    let rem = stream.get_remaining_words();
    let mut constant = Vec::with_capacity(rem);
    for _ in 0..rem {
        constant.push(stream.read_next().expect("Reading OpConstant constant"))
    }
    Ok(Core::OpConstant(OpConstant {
        result_type: result_type_id,
        result_id: result_id,
        value: constant,
    }))
}

fn read_op_constant_composite(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type_id = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    let rem = stream.get_remaining_words();
    let mut constituents = Vec::with_capacity(rem);
    for _ in 0..rem {
        constituents.push(read_op_id(stream).expect("Reading OpConstantComposite arguments"))
    }
    Ok(Core::OpConstantComposite(OpConstantComposite {
        result_type: result_type_id,
        result_id: result_id,
        constituents: constituents,
    }))
}

fn read_function_control(stream: &mut InstructionMemory) -> ReadResult<FunctionControl> {
    let value = try!(stream.read_next());
    Ok(FunctionControl {
        inline: (value & 0x1) != 0,
        dont_inline: (value & 0x2) != 0,
        pure_function: (value & 0x4) != 0,
        const_function: (value & 0x8) != 0,
    })
}

fn read_op_function(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 5 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type_id = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    let function_control = try!(read_function_control(stream));
    let function_type = try!(read_op_id(stream));
    let func = OpFunction {
        result_type: result_type_id,
        result_id: result_id,
        function_control: function_control,
        function_type: function_type,
    };
    Ok(Core::OpFunction(func))
}

fn read_op_function_parameter(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    Ok(Core::OpFunctionParameter(OpFunctionParameter {
        result_type: result_type,
        result_id: result_id,
    }))
}

fn read_op_function_end(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 1 {
        return Err(ReadError::WrongWordCountForOp);
    }
    Ok(Core::OpFunctionEnd(OpFunctionEnd))
}

fn read_op_variable(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 4 && stream.get_word_count() != 5 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    let storage_class = try!(read_storage_class(stream));
    let init = if stream.get_word_count() == 5 {
        Some(try!(read_op_id(stream)))
    } else {
        None
    };
    Ok(Core::OpVariable(OpVariable {
        result_type: result_type,
        result_id: result_id,
        storage_class: storage_class,
        initializer: init,
    }))
}

fn read_memory_access_opt(stream: &mut InstructionMemory) -> ReadResult<Option<MemoryAccess>> {
    Ok(if stream.get_remaining_words() == 0 {
        None
    } else {
        let memory_access_word = try!(stream.read_next());
        if (memory_access_word & 0xFFF8) != 0 {
            return Err(ReadError::UnknownMemoryAccess(memory_access_word));
        }
        Some(MemoryAccess {
            volatile: (memory_access_word & 0x1) != 0,
            aligned: (memory_access_word & 0x2) != 0,
            non_temporal: (memory_access_word & 0x4) != 0,
        })
    })
}

fn read_op_load(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 4 && stream.get_word_count() != 5 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    let pointer = try!(read_op_id(stream));
    let memory_access = try!(read_memory_access_opt(stream));
    Ok(Core::OpLoad(OpLoad {
        result_type: result_type,
        result_id: result_id,
        pointer: pointer,
        memory_access: memory_access,
    }))
}

fn read_op_store(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let pointer = try!(read_op_id(stream));
    let object = try!(read_op_id(stream));
    let memory_access = try!(read_memory_access_opt(stream));
    Ok(Core::OpStore(OpStore {
        pointer: pointer,
        object: object,
        memory_access: memory_access,
    }))
}

fn read_op_access_chain(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    let base = try!(read_op_id(stream));
    let indexes = try!(read_op_id_list(stream));
    Ok(Core::OpAccessChain(OpAccessChain {
        result_type: result_type,
        result_id: result_id,
        base: base,
        indexes: indexes,
    }))
}

fn read_builtin(stream: &mut InstructionMemory) -> ReadResult<BuiltIn> {
    let builtin_id = try!(stream.read_next());
    Ok(match builtin_id {
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
    })
}

fn read_func_param_attr(stream: &mut InstructionMemory) -> ReadResult<FunctionParameterAttribute> {
    let id = try!(stream.read_next());
    Ok(match id {
        0 => FunctionParameterAttribute::Zext,
        1 => FunctionParameterAttribute::Sext,
        2 => FunctionParameterAttribute::ByVal,
        3 => FunctionParameterAttribute::Sret,
        4 => FunctionParameterAttribute::NoAlias,
        5 => FunctionParameterAttribute::NoCapture,
        6 => FunctionParameterAttribute::NoWrite,
        7 => FunctionParameterAttribute::NoReadWrite,
        id => return Err(ReadError::UnknownFunctionParameterAttribute(id)),
    })
}

fn read_fp_rounding_mode(stream: &mut InstructionMemory) -> ReadResult<FpRoundingMode> {
    let id = try!(stream.read_next());
    Ok(match id {
        0 => FpRoundingMode::Rte,
        1 => FpRoundingMode::Rtz,
        2 => FpRoundingMode::Rtp,
        3 => FpRoundingMode::Rtn,
        id => return Err(ReadError::UnknownFpRoundingMode(id)),
    })
}

fn read_fp_fast_math_mode(stream: &mut InstructionMemory) -> ReadResult<FpFastMathMode> {
    let id = try!(stream.read_next());
    Ok(FpFastMathMode {
        not_nan: id & 0x1 != 0,
        not_inf: id & 0x2 != 0,
        nsz: id & 0x4 != 0,
        allow_recip: id & 0x9 != 0,
        fast: id & 0x10 != 0,
    })
}

fn read_linkage_type(stream: &mut InstructionMemory) -> ReadResult<LinkageType> {
    let id = try!(stream.read_next());
    Ok(match id {
        0 => LinkageType::Export,
        1 => LinkageType::Import,
        id => return Err(ReadError::UnknownLinkageType(id)),
    })
}

fn read_decoration(stream: &mut InstructionMemory) -> ReadResult<Decoration> {
    Ok(match try!(stream.read_next()) {
        0 => Decoration::RelaxedPrecision,
        1 => Decoration::SpecId(try!(read_lit_number_u32(stream))),
        2 => Decoration::Block,
        3 => Decoration::BufferBlock,
        4 => Decoration::RowMajor,
        5 => Decoration::ColMajor,
        6 => Decoration::ArrayStride(try!(read_lit_number_u32(stream))),
        7 => Decoration::MatrixStride(try!(read_lit_number_u32(stream))),
        8 => Decoration::GlslShared,
        9 => Decoration::GlslPacked,
        10 => Decoration::CPacked,
        11 => Decoration::BuiltIn(try!(read_builtin(stream))),
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
        29 => Decoration::Stream(try!(read_lit_number_u32(stream))),
        30 => Decoration::Location(try!(read_lit_number_u32(stream))),
        31 => Decoration::Component(try!(read_lit_number_u32(stream))),
        32 => Decoration::Index(try!(read_lit_number_u32(stream))),
        33 => Decoration::Binding(try!(read_lit_number_u32(stream))),
        34 => Decoration::DescriptorSet(try!(read_lit_number_u32(stream))),
        35 => Decoration::Offset(try!(read_lit_number_u32(stream))),
        36 => Decoration::XfbBuffer(try!(read_lit_number_u32(stream))),
        37 => Decoration::XfbStride(try!(read_lit_number_u32(stream))),
        38 => Decoration::FuncParamAttr(try!(read_func_param_attr(stream))),
        39 => Decoration::FpRoundingMode(try!(read_fp_rounding_mode(stream))),
        40 => Decoration::FpFastMathMode(try!(read_fp_fast_math_mode(stream))),
        41 => {
            let name = try!(read_string_literal(stream));
            let linkage_type = try!(read_linkage_type(stream));
            Decoration::LinkageAttributes(name, linkage_type)
        }
        42 => Decoration::NoContraction,
        43 => Decoration::InputAttachmentIndex(try!(read_lit_number_u32(stream))),
        44 => Decoration::Alignment(try!(read_lit_number_u32(stream))),
        id => return Err(ReadError::UnknownDecoration(id)),
    })
}

fn read_op_decorate(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let id = try!(read_op_id(stream));
    let decorate = try!(read_decoration(stream));
    Ok(Core::OpDecorate(OpDecorate {
        target: id,
        decoration: decorate,
    }))
}

fn read_op_member_decorate(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() < 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let structure_type = try!(read_op_id(stream));
    let member = MemberIndex(try!(read_lit_number_u32(stream)));
    let decorate = try!(read_decoration(stream));
    Ok(Core::OpMemberDecorate(OpMemberDecorate {
        structure_type: structure_type,
        member: member,
        decoration: decorate,
    }))
}

fn read_op_convert_utof(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    let unsigned_value = try!(read_op_id(stream));
    Ok(Core::OpConvertUToF(OpConvertUToF {
        result_type: result_type,
        result_id: result_id,
        unsigned_value: unsigned_value,
    }))
}

fn read_op_imul(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 5 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    let operand1 = try!(read_op_id(stream));
    let operand2 = try!(read_op_id(stream));
    Ok(Core::OpIMul(OpIMul {
        result_type: result_type,
        result_id: result_id,
        operand1: operand1,
        operand2: operand2,
    }))
}

fn read_op_label(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    Ok(Core::OpLabel(OpLabel { result_id: result_id }))
}

fn read_op_branch(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let label_id = try!(read_op_id(stream));
    Ok(Core::OpBranch(OpBranch { target_label: label_id }))
}

fn read_op_return(stream: &mut InstructionMemory) -> ReadResult<Core> {
    if stream.get_word_count() != 1 {
        return Err(ReadError::WrongWordCountForOp);
    }
    Ok(Core::OpReturn(OpReturn))
}
