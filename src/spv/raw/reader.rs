
use std::collections::HashMap;
use spv::ExtInstSet;
use spv::ExtInstBox;
use spv::op::*;
use spv::types::*;
use spv::raw::*;
use byteorder::{LittleEndian, BigEndian, ByteOrder};

/// Magic number for a SPIR-V module
const SPIRV_MAGIC_NUMBER: u32 = 0x07230203;
/// Magic number for a SPIR-V module if the endianness is flipped
const SPIRV_MAGIC_NUMBER_OTHER_ENDIAN: u32 = 0x03022307;

pub fn read_module<'a>(data: &'a [u8],
                       known_inst_sets: Vec<Box<ExtInstSet>>)
                       -> ReadResult<RawModule> {
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
        _ => return Err(ReadError::UnknownReservedSchema),
    }

    let mut instructions = Vec::new();
    let mut bound_inst_sets = HashMap::new();
    while !stream.is_end() {
        let instr = try!(read_instruction(&mut stream, &known_inst_sets, &mut bound_inst_sets));
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

fn read_instruction(stream: &mut Stream,
                    known_inst_sets: &[Box<ExtInstSet>],
                    bound_inst_sets: &mut HashMap<OpId, Box<ExtInstSet>>)
                    -> ReadResult<Core> {
    let head = try!(stream.read_word());
    let id = (head & 0xFFFF) as u16;
    let wc = (head >> 16) as u16;
    let mut words = Vec::with_capacity(wc as usize);
    words.push(head);
    for _ in 1..wc {
        words.push(try!(stream.read_word()));
    }
    let block = MemoryBlock::new(&words[1..words.len()]);
    let read_fn = match id {
        0 => OpNop::read_core,
        1 => OpUndef::read_core,
        2 => OpSourceContinued::read_core,
        3 => OpSource::read_core,
        4 => OpSourceExtension::read_core,
        5 => OpName::read_core,
        6 => OpMemberName::read_core,
        7 => OpString::read_core,
        8 => return Err(ReadError::UnimplementedOp("OpLine")),
        10 => OpExtension::read_core,
        11 => OpExtInstImport::read_core,
        12 => OpExtInst::read_core,
        14 => OpMemoryModel::read_core,
        15 => OpEntryPoint::read_core,
        16 => OpExecutionMode::read_core,
        17 => OpCapability::read_core,
        19 => OpTypeVoid::read_core,
        20 => OpTypeBool::read_core,
        21 => OpTypeInt::read_core,
        22 => OpTypeFloat::read_core,
        23 => OpTypeVector::read_core,
        24 => OpTypeMatrix::read_core,
        25 => OpTypeImage::read_core,
        26 => OpTypeSampler::read_core,
        27 => OpTypeSampledImage::read_core,
        28 => OpTypeArray::read_core,
        29 => OpTypeRuntimeArray::read_core,
        30 => OpTypeStruct::read_core,
        31 => OpTypeOpaque::read_core,
        32 => OpTypePointer::read_core,
        33 => OpTypeFunction::read_core,
        41 => return Err(ReadError::UnimplementedOp("OpConstantTrue")),
        42 => return Err(ReadError::UnimplementedOp("OpConstantFalse")),
        43 => OpConstant::read_core,
        44 => OpConstantComposite::read_core,
        45 => return Err(ReadError::UnimplementedOp("OpConstantSampler")),
        46 => return Err(ReadError::UnimplementedOp("OpConstantNull")),
        48 => return Err(ReadError::UnimplementedOp("OpSpecConstantTrue")),
        49 => return Err(ReadError::UnimplementedOp("OpSpecConstantFalse")),
        50 => return Err(ReadError::UnimplementedOp("OpSpecConstant")),
        51 => return Err(ReadError::UnimplementedOp("OpSpecConstantComposite")),
        52 => return Err(ReadError::UnimplementedOp("OpSpecConstantOp")),
        54 => OpFunction::read_core,
        55 => OpFunctionParameter::read_core,
        56 => OpFunctionEnd::read_core,
        57 => return Err(ReadError::UnimplementedOp("OpFunctionCall")),
        59 => OpVariable::read_core,
        60 => return Err(ReadError::UnimplementedOp("OpImageTexelPointer")),
        61 => OpLoad::read_core,
        62 => OpStore::read_core,
        63 => return Err(ReadError::UnimplementedOp("OpCopyMemory")),
        64 => return Err(ReadError::UnimplementedOp("OpCopyMemorySized")),
        65 => OpAccessChain::read_core,
        66 => return Err(ReadError::UnimplementedOp("OpInBoundsAccessChain")),
        67 => return Err(ReadError::UnimplementedOp("OpPtrAccessChain")),
        68 => return Err(ReadError::UnimplementedOp("OpArrayLength")),
        69 => return Err(ReadError::UnimplementedOp("OpGenericPtrMemSemantics")),
        70 => return Err(ReadError::UnimplementedOp("OpInBoundsPtrAccessChain")),
        71 => OpDecorate::read_core,
        72 => OpMemberDecorate::read_core,
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
        112 => OpConvertUToF::read_core,
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
        128 => OpIAdd::read_core,
        129 => OpFAdd::read_core,
        130 => OpISub::read_core,
        131 => OpFSub::read_core,
        132 => OpIMul::read_core,
        133 => OpFMul::read_core,
        134 => OpUDiv::read_core,
        135 => OpSDiv::read_core,
        136 => OpFDiv::read_core,
        137 => OpUMod::read_core,
        138 => OpSRem::read_core,
        139 => OpSMod::read_core,
        140 => OpFRem::read_core,
        141 => OpFMod::read_core,
        142 => return Err(ReadError::UnimplementedOp("OpVectorTimesScalar")),
        143 => return Err(ReadError::UnimplementedOp("OpMatrixTimesScalar")),
        144 => return Err(ReadError::UnimplementedOp("OpVectorTimesMatrix")),
        145 => return Err(ReadError::UnimplementedOp("OpMatrixTimesVector")),
        146 => return Err(ReadError::UnimplementedOp("OpMatrixTimesMatrix")),
        147 => return Err(ReadError::UnimplementedOp("OpOuterProduct")),
        148 => return Err(ReadError::UnimplementedOp("OpDot")),
        149 => OpIAddCarry::read_core,
        150 => OpISubBorrow::read_core,
        151 => OpUMulExtended::read_core,
        152 => OpSMulExtended::read_core,
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
        170 => OpIEqual::read_core,
        171 => OpINotEqual::read_core,
        172 => OpUGreaterThan::read_core,
        173 => OpSGreaterThan::read_core,
        174 => OpUGreaterThanEqual::read_core,
        175 => OpSGreaterThanEqual::read_core,
        176 => OpULessThan::read_core,
        177 => OpSLessThan::read_core,
        178 => OpULessThanEqual::read_core,
        179 => OpSLessThanEqual::read_core,
        180 => OpFOrdEqual::read_core,
        181 => OpFUnordEqual::read_core,
        182 => OpFOrdNotEqual::read_core,
        183 => OpFUnordNotEqual::read_core,
        184 => OpFOrdLessThan::read_core,
        185 => OpFUnordLessThan::read_core,
        186 => OpFOrdGreaterThan::read_core,
        187 => OpFUnordGreaterThan::read_core,
        188 => OpFOrdLessThanEqual::read_core,
        189 => OpFUnordLessThanEqual::read_core,
        190 => OpFOrdGreaterThanEqual::read_core,
        191 => OpFUnordGreaterThanEqual::read_core,
        194 => return Err(ReadError::UnimplementedOp("OpShiftRightLogical")),
        195 => return Err(ReadError::UnimplementedOp("OpShiftRightArithmetic")),
        196 => return Err(ReadError::UnimplementedOp("OpShiftLeftLogical")),
        197 => OpBitwiseOr::read_core,
        198 => OpBitwiseXor::read_core,
        199 => OpBitwiseAnd::read_core,
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
        245 => OpPhi::read_core,
        246 => OpLoopMerge::read_core,
        247 => OpSelectionMerge::read_core,
        248 => OpLabel::read_core,
        249 => OpBranch::read_core,
        250 => OpBranchConditional::read_core,
        251 => return Err(ReadError::UnimplementedOp("OpSwitch")),
        252 => return Err(ReadError::UnimplementedOp("OpKill")),
        253 => OpReturn::read_core,
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
        317 => return Err(ReadError::UnimplementedOp("OpNoLine")),
        318 => return Err(ReadError::UnimplementedOp("OpAtomicFlagTestAndSet")),
        319 => return Err(ReadError::UnimplementedOp("OpAtomicFlagClear")),
        320 => return Err(ReadError::UnimplementedOp("OpImageSparseRead")),
        _ => return Err(ReadError::UnknownOp(id, wc)),
    };
    let (block, inst) = try!(read_fn(block, known_inst_sets, bound_inst_sets));
    if !block.end() {
        return Err(ReadError::InstructionHadExcessData);
    }
    Ok(inst)
}

trait CoreRead {
    fn read_core<'a>(block: MemoryBlock<'a>,
                     known_inst_sets: &[Box<ExtInstSet>],
                     bound_inst_sets: &mut HashMap<OpId, Box<ExtInstSet>>)
                     -> MemoryBlockResult<'a, Core>;
}

macro_rules! def_op_read {
    ($name: ident; $($operand_name: ident)|*) => {
        impl MemoryBlockRead for $name {
            fn read(block: MemoryBlock) -> MemoryBlockResult<Self> {
                $(
                    let (block, $operand_name) = try!(
                        $crate::spv::raw::MemoryBlockRead::read(block)
                    );
                )*
                let op = $name {
                    $(
                        $operand_name: $operand_name
                    ,)*
                };
                Ok((block, op))
            }
        }
        impl CoreRead for $name {
            fn read_core<'a>(block: MemoryBlock<'a>,
                               _: &[Box<ExtInstSet>],
                               _: &mut HashMap<OpId, Box<ExtInstSet>>)
                            -> MemoryBlockResult<'a, Core> {
                let (block, item) = try!(<$name as MemoryBlockRead>::read(block));
                Ok((block, $crate::spv::raw::Core::$name(item)))
            }
        }
    };
}

macro_rules! def_op_read_s1 {
    ($name: ident) => {
        def_op_read!($name; result_type | result_id | operand);
    };
}

macro_rules! def_op_read_s2 {
    ($name: ident) => {
        def_op_read!($name; result_type | result_id | operand1 | operand2);
    };
}

// CoreRead implementations. In same order as standard.

// Miscellaneous Instructions
def_op_read!(OpNop;);
def_op_read!(OpUndef; result_type | result_id);

// Debug Instructions
def_op_read!(OpSourceContinued; continued_source);
def_op_read!(OpSource; language | version | file | source);
def_op_read!(OpSourceExtension; extension);
def_op_read!(OpName; target | name);
def_op_read!(OpMemberName; struct_type | member | name);
def_op_read!(OpString; result_id | string);

// Annotation Instructions
def_op_read!(OpDecorate; target | decoration);
def_op_read!(OpMemberDecorate; structure_type | member | decoration);

// Extension Instructions
def_op_read!(OpExtension; name);

impl CoreRead for OpExtInstImport {
    fn read_core<'a>(block: MemoryBlock<'a>,
                     known_inst_sets: &[Box<ExtInstSet>],
                     bound_inst_sets: &mut HashMap<OpId, Box<ExtInstSet>>)
                     -> MemoryBlockResult<'a, Core> {
        let (block, result_id) = try!(ResultId::read(block));
        let (block, name) = try!(LitString::read(block));
        for inst_set in known_inst_sets {
            if inst_set.get_name() == name {
                match bound_inst_sets.insert(OpId(result_id.0), inst_set.duplicate()) {
                    Some(_) => return Err(ReadError::DuplicateResultId(result_id)),
                    None => {}
                }
                let op = Core::OpExtInstImport(OpExtInstImport {
                    result_id: result_id,
                    name: name,
                });
                return Ok((block, op));
            }
        }
        Err(ReadError::UnknownInstSet(name))
    }
}

impl CoreRead for OpExtInst {
    fn read_core<'a>(block: MemoryBlock<'a>,
                     _: &[Box<ExtInstSet>],
                     bound_inst_sets: &mut HashMap<OpId, Box<ExtInstSet>>)
                     -> MemoryBlockResult<'a, Core> {
        let (block, result_type) = try!(OpId::read(block));
        let (block, result_id) = try!(ResultId::read(block));
        let (block, set_id) = try!(OpId::read(block));
        let (block, inst) = try!(u32::read(block));
        match bound_inst_sets.get(&set_id) {
            Some(set) => {
                let (block, extinst) = try!(set.read_instruction(inst, block));
                let op = Core::OpExtInst(OpExtInst {
                    result_type: result_type,
                    result_id: result_id,
                    set: set_id,
                    instruction: ExtInstBox(extinst),
                });
                Ok((block, op))
            }
            None => Err(ReadError::UnknownInstSetId(set_id)),
        }
    }
}

def_op_read!(OpMemoryModel; addressing_model | memory_model);
def_op_read!(OpEntryPoint; execution_model | entry_point | name | interface);
def_op_read!(OpExecutionMode; entry_point | mode);
def_op_read!(OpCapability; capability);

// Type-Declaration Instructions
def_op_read!(OpTypeVoid; result_id);
def_op_read!(OpTypeBool; result_id);
def_op_read!(OpTypeInt; result_id | width | signedness);
def_op_read!(OpTypeFloat; result_id | width);
def_op_read!(OpTypeVector; result_id | component_type | component_count);
def_op_read!(OpTypeMatrix; result_id | column_type | column_count);
def_op_read!(OpTypeImage;
    result_id |
    sampled_type |
    dim |
    depth |
    arrayed |
    ms |
    sampled |
    format |
    access_qualifier
);
def_op_read!(OpTypeSampler; result_id);
def_op_read!(OpTypeSampledImage; result_id | image_type);
def_op_read!(OpTypeArray; result_id | element_type | length);
def_op_read!(OpTypeRuntimeArray; result_id | element_type);
def_op_read!(OpTypeStruct; result_id | member_types);
def_op_read!(OpTypeOpaque; result_id | name);
def_op_read!(OpTypePointer; result_id | storage_class | pointed_type);
def_op_read!(OpTypeFunction; result_id | return_type | parameter_types);

// Constant-Creation Instructions
def_op_read!(OpConstant; result_type | result_id | value);
def_op_read!(OpConstantComposite; result_type | result_id | constituents);

// Function Instructions
def_op_read!(OpFunction; result_type | result_id | function_control | function_type);
def_op_read!(OpFunctionParameter; result_type | result_id);
def_op_read!(OpFunctionEnd;);

// Memory Instructions
def_op_read!(OpVariable; result_type | result_id | storage_class | initializer);

def_op_read!(OpLoad; result_type | result_id | pointer | memory_access);
def_op_read!(OpStore; pointer | object |  memory_access);

def_op_read!(OpAccessChain; result_type | result_id | base | indexes);

// Conversion Instructions
def_op_read!(OpConvertUToF; result_type | result_id | unsigned_value);

// Arithmetic Instructions
def_op_read_s2!(OpIAdd);
def_op_read_s2!(OpFAdd);
def_op_read_s2!(OpISub);
def_op_read_s2!(OpFSub);
def_op_read_s2!(OpIMul);
def_op_read_s2!(OpFMul);
def_op_read_s2!(OpUDiv);
def_op_read_s2!(OpSDiv);
def_op_read_s2!(OpFDiv);
def_op_read_s2!(OpUMod);
def_op_read_s2!(OpSRem);
def_op_read_s2!(OpSMod);
def_op_read_s2!(OpFRem);
def_op_read_s2!(OpFMod);

def_op_read_s2!(OpIAddCarry);
def_op_read_s2!(OpISubBorrow);
def_op_read_s2!(OpUMulExtended);
def_op_read_s2!(OpSMulExtended);

// Bit Instructions
def_op_read_s2!(OpBitwiseOr);
def_op_read_s2!(OpBitwiseXor);
def_op_read_s2!(OpBitwiseAnd);

// Relational and Logical Instructions
def_op_read_s2!(OpIEqual);
def_op_read_s2!(OpINotEqual);
def_op_read_s2!(OpUGreaterThan);
def_op_read_s2!(OpSGreaterThan);
def_op_read_s2!(OpUGreaterThanEqual);
def_op_read_s2!(OpSGreaterThanEqual);
def_op_read_s2!(OpULessThan);
def_op_read_s2!(OpSLessThan);
def_op_read_s2!(OpULessThanEqual);
def_op_read_s2!(OpSLessThanEqual);
def_op_read_s2!(OpFOrdEqual);
def_op_read_s2!(OpFUnordEqual);
def_op_read_s2!(OpFOrdNotEqual);
def_op_read_s2!(OpFUnordNotEqual);
def_op_read_s2!(OpFOrdLessThan);
def_op_read_s2!(OpFUnordLessThan);
def_op_read_s2!(OpFOrdGreaterThan);
def_op_read_s2!(OpFUnordGreaterThan);
def_op_read_s2!(OpFOrdLessThanEqual);
def_op_read_s2!(OpFUnordLessThanEqual);
def_op_read_s2!(OpFOrdGreaterThanEqual);
def_op_read_s2!(OpFUnordGreaterThanEqual);

// Control-Flow Instructions
def_op_read!(OpPhi; result_type | result_id | variables);
def_op_read!(OpLoopMerge; merge_block | continue_target | loop_control);
def_op_read!(OpSelectionMerge; merge_block | selection_control);
def_op_read!(OpLabel; result_id);
def_op_read!(OpBranch; target_label);
def_op_read!(OpBranchConditional; condition | true_label | false_label | weights);

def_op_read!(OpReturn;);
