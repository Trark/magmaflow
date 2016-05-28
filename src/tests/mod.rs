
use spv::*;
use reader;

const NOOP_SPV: &'static [u8] = include_bytes!("noop.spv");
const NOOP_DIS: &'static str = include_str!("noop.dis");

#[test]
fn load_noop() {
    let result = reader::read_module(NOOP_SPV);
    let glsl450 = OpExtInstImport(ResultId(1), "GLSL.std.450".into());
    let mem_model = OpMemoryModel(AddressingMode::Logical, MemoryModel::Glsl450);
    let entry_point = OpEntryPoint(ExecutionModel::GlCompute, OpId(4), "main".into(), vec![]);
    let local_size = ExecutionMode::LocalSize(32, 32, 32);
    let source = OpSource(SourceLanguage::Glsl, SourceVersion(430), None, None);
    let workgroup = OpDecorate(OpId(9), Decoration::BuiltIn(BuiltIn::WorkgroupSize));
    let ty_void = Core::OpTypeVoid(OpTypeVoid(ResultId(2)));
    let ty_main = Core::OpTypeFunction(OpTypeFunction(ResultId(3), OpId(2), vec![]));
    let ty_int = Core::OpTypeInt(OpTypeInt(ResultId(6), 32, 0));
    let ty_vec = Core::OpTypeVector(OpTypeVector(ResultId(7), OpId(6), 3));
    let int_32 = Core::OpConstant(OpConstant(OpId(6), ResultId(8), vec![32]));
    let int_32_x3 = vec![OpId(8), OpId(8), OpId(8)];
    let int_32x3 = Core::OpConstantComposite(OpConstantComposite(OpId(7), ResultId(9), int_32_x3));
    let main = OpFunction(OpId(2), ResultId(4), FunctionControl::default(), OpId(3));
    let instructions = vec![Core::OpCapability(OpCapability(Capability::Shader)),
                            Core::OpExtInstImport(glsl450),
                            Core::OpMemoryModel(mem_model),
                            Core::OpEntryPoint(entry_point),
                            Core::OpExecutionMode(OpExecutionMode(OpId(4), local_size)),
                            Core::OpSource(source),
                            Core::OpName(OpName(OpId(4), "main".to_string())),
                            Core::OpDecorate(workgroup),
                            ty_void,
                            ty_main,
                            ty_int,
                            ty_vec,
                            int_32,
                            int_32x3,
                            Core::OpFunction(main),
                            Core::OpLabel(OpLabel(ResultId(5))),
                            Core::OpReturn(OpReturn),
                            Core::OpFunctionEnd(OpFunctionEnd)];
    let expected = RawModule {
        version: Version(1, 0),
        generator: Generator {
            tool: Tool::KhronosGlslang,
            version: 1,
        },
        bound: 10,
        instructions: instructions,
    };
    assert_eq!(result, Ok(expected));
}

#[test]
fn dis_noop() {
    let raw_module = reader::read_module(NOOP_SPV).expect("Failed to load noop.spv");
    let disassembly = format!("{}", raw_module);
    for (dis, expect) in disassembly.lines().zip(NOOP_DIS.lines()) {
        assert_eq!(dis, expect);
    }
    assert_eq!(NOOP_DIS, disassembly);

}
