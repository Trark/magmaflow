
use spv::*;
use byteorder::{LittleEndian, BigEndian, ByteOrder};

#[derive(Debug, PartialEq)]
pub enum ReadError {
    UnexpectedEndOfStream,
    UnexpectedStreamAlignment,
    BadMagic,
    UnknownVersion(Word),
    UnknownReservedHeaderWord4,
    UnknownOp(u16, u16),
    WrongWordCountForOp,
    InvalidString,

    UnexpectedEndOfInstruction,
    InstructionHadExcessData,

    UnknownAddressingMode(Word),
    UnknownMemoryModel(Word),
    UnknownExecutionModel(Word),
    UnknownExecutionMode(Word),
    UnknownCapability(Word),
    UnknownDecoration(Word),
    UnknownBuiltIn(Word),
    UnknownFpRoundingMode(Word),
    UnknownLinkageType(Word),
    UnknownFunctionParameterAttribute(Word),

    ModuleLayoutMemoryModelMissing,
}

pub type ReadResult<T> = Result<T, ReadError>;

const SPIRV_MAGIC_NUMBER: Word = 0x07230203;
const SPIRV_MAGIC_NUMBER_OTHER_ENDIAN: Word = 0x03022307;

pub fn read_module<'a>(data: &'a [u8]) -> ReadResult<RawModule> {
    let mut stream = Stream::new(data);

    let magic = try!(stream.read_word());
    match magic {
        SPIRV_MAGIC_NUMBER => {}
        SPIRV_MAGIC_NUMBER_OTHER_ENDIAN => stream.invert_endianness(),
        _ => return Err(ReadError::BadMagic),
    }

    let version = try!(stream.read_word());
    match version {
        0x10000 => {}
        v => return Err(ReadError::UnknownVersion(v)),
    }

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
        _ => return Err(ReadError::UnknownReservedHeaderWord4),
    }

    let mut instructions = Vec::new();
    while !stream.is_end() {
        let instr = try!(read_instruction(&mut stream));
        instructions.push(instr);
    }

    Ok(RawModule {
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

    fn read_word(&mut self) -> ReadResult<Word> {
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

fn read_instruction(stream: &mut Stream) -> ReadResult<Op> {
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
        0 => Ok(Op::OpNop(OpNop)),
        3 => read_op_source(&mut im),
        5 => read_op_name(&mut im),
        11 => read_op_ext_inst_import(&mut im),
        14 => read_op_memory_model(&mut im),
        15 => read_op_entry_point(&mut im),
        16 => read_op_execution_mode(&mut im),
        17 => read_op_capability(&mut im),
        19 => read_op_type_void(&mut im),
        20 => read_op_type_bool(&mut im),
        21 => read_op_type_int(&mut im),
        22 => read_op_type_float(&mut im),
        23 => read_op_type_vector(&mut im),
        33 => read_op_type_function(&mut im),
        43 => read_op_constant(&mut im),
        44 => read_op_constant_composite(&mut im),
        54 => read_op_function(&mut im),
        56 => read_op_function_end(&mut im),
        71 => read_op_decorate(&mut im),
        248 => read_op_label(&mut im),
        249 => read_op_branch(&mut im),
        253 => read_op_return(&mut im),
        _ => Err(ReadError::UnknownOp(id, wc)),
    });
    try!(im.finish());
    Ok(inst)
}

struct InstructionMemory<'a> {
    block: &'a [Word],
    position: usize,
}

impl<'a> InstructionMemory<'a> {
    fn new(memory: &'a [Word]) -> InstructionMemory<'a> {
        InstructionMemory {
            block: memory,
            position: 1, // First word is code / length
        }
    }

    fn read_next(&mut self) -> ReadResult<Word> {
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

fn read_lit_number_word(stream: &mut InstructionMemory) -> ReadResult<LitNumber> {
    let word = try!(stream.read_next());
    Ok(vec![word])
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

fn read_op_source(stream: &mut InstructionMemory) -> ReadResult<Op> {
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
    Ok(Op::OpSource(OpSource(lang, version, file_id, source_name)))
}

fn read_op_name(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let id = try!(read_op_id(stream));
    let name = try!(read_string_literal(stream));
    Ok(Op::OpName(OpName(id, name)))
}

fn read_op_ext_inst_import(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_op_id(stream));
    let name = try!(read_string_literal(stream));
    Ok(Op::OpExtInstImport(OpExtInstImport(result_id, name)))
}

fn read_addressing_mode(stream: &mut InstructionMemory) -> ReadResult<AddressingMode> {
    let am = try!(stream.read_next());
    Ok(match am {
        0 => AddressingMode::Logical,
        1 => AddressingMode::Physical32,
        2 => AddressingMode::Physical64,
        id => return Err(ReadError::UnknownAddressingMode(id)),
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

fn read_op_memory_model(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let am = try!(read_addressing_mode(stream));
    let mm = try!(read_memory_model(stream));
    Ok(Op::OpMemoryModel(OpMemoryModel(am, mm)))
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

fn read_op_entry_point(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() < 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let execution_model = try!(read_execution_model(stream));
    let entry_point = try!(read_op_id(stream));
    let name = try!(read_string_literal(stream));
    let forward_defs = try!(read_op_id_list(stream));
    let inst = OpEntryPoint(execution_model, entry_point, name, forward_defs);
    Ok(Op::OpEntryPoint(inst))
}

fn read_op_execution_mode(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let entry_point = try!(read_op_id(stream));
    let execution_mode_id = try!(stream.read_next());
    let mode = match execution_mode_id {
        0 => {
            let num = try!(read_lit_number_word(stream));
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
            let x = try!(read_lit_number_word(stream));
            let y = try!(read_lit_number_word(stream));
            let z = try!(read_lit_number_word(stream));
            ExecutionMode::LocalSize(x, y, z)
        }
        18 => {
            let x = try!(read_lit_number_word(stream));
            let y = try!(read_lit_number_word(stream));
            let z = try!(read_lit_number_word(stream));
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
            let num = try!(read_lit_number_word(stream));
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
    Ok(Op::OpExecutionMode(OpExecutionMode(entry_point, mode)))
}

fn read_op_capability(stream: &mut InstructionMemory) -> ReadResult<Op> {
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
    Ok(Op::OpCapability(OpCapability(capability)))
}

fn read_op_type_void(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    Ok(Op::OpTypeVoid(OpTypeVoid(result_id)))
}

fn read_op_type_bool(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    Ok(Op::OpTypeBool(OpTypeBool(result_id)))
}

fn read_op_type_int(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let width = try!(read_lit_number_word(stream));
    let signedness = try!(read_lit_number_word(stream));
    Ok(Op::OpTypeInt(OpTypeInt(result_id, width, signedness)))
}

fn read_op_type_float(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let width = try!(read_lit_number_word(stream));
    Ok(Op::OpTypeFloat(OpTypeFloat(result_id, width)))
}

fn read_op_type_vector(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 4 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    let component_type = try!(read_op_id(stream));
    let count = try!(read_lit_number_word(stream));
    Ok(Op::OpTypeVector(OpTypeVector(result_id, component_type, count)))
}

fn read_op_type_function(stream: &mut InstructionMemory) -> ReadResult<Op> {
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
    Ok(Op::OpTypeFunction(OpTypeFunction(result_id, return_type, param_types)))
}

fn read_op_constant(stream: &mut InstructionMemory) -> ReadResult<Op> {
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
    let inst = OpConstant(result_type_id, result_id, constant);
    Ok(Op::OpConstant(inst))
}

fn read_op_constant_composite(stream: &mut InstructionMemory) -> ReadResult<Op> {
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
    let inst = OpConstantComposite(result_type_id, result_id, constituents);
    Ok(Op::OpConstantComposite(inst))
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

fn read_op_function(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 5 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_type_id = try!(read_op_id(stream));
    let result_id = try!(read_result_id(stream));
    let function_control = try!(read_function_control(stream));
    let function_type = try!(read_op_id(stream));
    let func = OpFunction(result_type_id, result_id, function_control, function_type);
    Ok(Op::OpFunction(func))
}

fn read_op_function_end(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 1 {
        return Err(ReadError::WrongWordCountForOp);
    }
    Ok(Op::OpFunctionEnd(OpFunctionEnd))
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

fn read_op_decorate(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() < 3 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let id = try!(read_op_id(stream));
    let decorate_id = try!(stream.read_next());
    let decorate = match decorate_id {
        0 => Decoration::RelaxedPrecision,
        1 => Decoration::SpecId(try!(read_lit_number_word(stream))),
        2 => Decoration::Block,
        3 => Decoration::BufferBlock,
        4 => Decoration::RowMajor,
        5 => Decoration::ColMajor,
        6 => Decoration::ArrayStride(try!(read_lit_number_word(stream))),
        7 => Decoration::MatrixStride(try!(read_lit_number_word(stream))),
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
        29 => Decoration::Stream(try!(read_lit_number_word(stream))),
        30 => Decoration::Location(try!(read_lit_number_word(stream))),
        31 => Decoration::Component(try!(read_lit_number_word(stream))),
        32 => Decoration::Index(try!(read_lit_number_word(stream))),
        33 => Decoration::Binding(try!(read_lit_number_word(stream))),
        34 => Decoration::DescriptorSet(try!(read_lit_number_word(stream))),
        35 => Decoration::Offset(try!(read_lit_number_word(stream))),
        36 => Decoration::XfbBuffer(try!(read_lit_number_word(stream))),
        37 => Decoration::XfbStride(try!(read_lit_number_word(stream))),
        38 => Decoration::FuncParamAttr(try!(read_func_param_attr(stream))),
        39 => Decoration::FpRoundingMode(try!(read_fp_rounding_mode(stream))),
        40 => Decoration::FpFastMathMode(try!(read_fp_fast_math_mode(stream))),
        41 => {
            let name = try!(read_string_literal(stream));
            let linkage_type = try!(read_linkage_type(stream));
            Decoration::LinkageAttributes(name, linkage_type)
        }
        42 => Decoration::NoContraction,
        43 => Decoration::InputAttachmentIndex(try!(read_lit_number_word(stream))),
        44 => Decoration::Alignment(try!(read_lit_number_word(stream))),
        id => return Err(ReadError::UnknownDecoration(id)),
    };
    Ok(Op::OpDecorate(OpDecorate(id, decorate)))
}

fn read_op_label(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let result_id = try!(read_result_id(stream));
    Ok(Op::OpLabel(OpLabel(result_id)))
}

fn read_op_branch(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 2 {
        return Err(ReadError::WrongWordCountForOp);
    }
    let label_id = try!(read_result_id(stream));
    Ok(Op::OpLabel(OpLabel(label_id)))
}

fn read_op_return(stream: &mut InstructionMemory) -> ReadResult<Op> {
    if stream.get_word_count() != 1 {
        return Err(ReadError::WrongWordCountForOp);
    }
    Ok(Op::OpReturn(OpReturn))
}
