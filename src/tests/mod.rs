
use spv::*;
use reader;

const NOOP_SPV: &'static [u8] = include_bytes!("noop.spv");

#[test]
fn load_noop() {
    let result = reader::read_module(NOOP_SPV);
    let mem_model = OpMemoryModel(AddressingMode::Logical, MemoryModel::Glsl450);
    let entry_point = OpEntryPoint(ExecutionModel::GlCompute, OpId(4), "main".into(), vec![]);
    let local_size = ExecutionMode::LocalSize(vec![32], vec![32], vec![32]);
    let source = OpSource(SourceLanguage::Glsl, SourceVersion(430), None, None);
    let workgroup = OpDecorate(OpId(9), Decoration::BuiltIn(BuiltIn::WorkgroupSize));
    let ty_void = Op::OpTypeVoid(OpTypeVoid(ResultId(2)));
    let ty_main = Op::OpTypeFunction(OpTypeFunction(ResultId(3), OpId(2), vec![]));
    let ty_int = Op::OpTypeInt(OpTypeInt(ResultId(6), vec![32], vec![0]));
    let ty_vec = Op::OpTypeVector(OpTypeVector(ResultId(7), OpId(6), vec![3]));
    let int_32 = Op::OpConstant(OpConstant(OpId(6), ResultId(8), vec![32]));
    let int_32_x3 = vec![OpId(8), OpId(8), OpId(8)];
    let int_32x3 = Op::OpConstantComposite(OpConstantComposite(OpId(7), ResultId(9), int_32_x3));
    let main = OpFunction(OpId(2), ResultId(4), FunctionControl::default(), OpId(3));
    let instructions = vec![Op::OpCapability(OpCapability(Capability::Shader)),
                            Op::OpExtInstImport(OpExtInstImport(OpId(1), "GLSL.std.450".into())),
                            Op::OpMemoryModel(mem_model),
                            Op::OpEntryPoint(entry_point),
                            Op::OpExecutionMode(OpExecutionMode(OpId(4), local_size)),
                            Op::OpSource(source),
                            Op::OpName(OpName(OpId(4), "main".to_string())),
                            Op::OpDecorate(workgroup),
                            ty_void,
                            ty_main,
                            ty_int,
                            ty_vec,
                            int_32,
                            int_32x3,
                            Op::OpFunction(main),
                            Op::OpLabel(OpLabel(ResultId(5))),
                            Op::OpReturn(OpReturn),
                            Op::OpFunctionEnd(OpFunctionEnd)];
    let expected = RawModule {
        generator: Generator {
            tool: Tool::KhronosGlslang,
            version: 1,
        },
        bound: 10,
        instructions: instructions,
    };
    assert_eq!(result, Ok(expected));
}
