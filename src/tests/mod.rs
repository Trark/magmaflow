
use spv::*;
use spv::op::*;
use spv::types::*;
use reader;

const NOOP_SPV: &'static [u8] = include_bytes!("noop.spv");
const NOOP_DIS: &'static str = include_str!("noop.dis");
const WRITE_MULTIPLY_SPV: &'static [u8] = include_bytes!("write_multiply.spv");
const WRITE_MULTIPLY_DIS: &'static str = include_str!("write_multiply.dis");

#[test]
fn load_noop() {
    let result = reader::read_module(NOOP_SPV);
    let glsl450 = OpExtInstImport {
        result_id: ResultId(1),
        name: "GLSL.std.450".into(),
    };
    let mem_model = OpMemoryModel {
        addressing_mode: AddressingMode::Logical,
        memory_model: MemoryModel::Glsl450,
    };
    let entry_point = OpEntryPoint {
        execution_model: ExecutionModel::GlCompute,
        entry_point: OpId(4),
        name: "main".into(),
        interface: vec![],
    };
    let local_size = ExecutionMode::LocalSize(32, 32, 32);
    let source = OpSource {
        language: SourceLanguage::Glsl,
        version: SourceVersion(430),
        file: None,
        source: None,
    };
    let workgroup = OpDecorate {
        target: OpId(9),
        decoration: Decoration::BuiltIn(BuiltIn::WorkgroupSize),
    };
    let ty_void = Core::OpTypeVoid(OpTypeVoid { result_id: ResultId(2) });
    let ty_main = Core::OpTypeFunction(OpTypeFunction {
        result_id: ResultId(3),
        return_type: OpId(2),
        parameter_types: vec![],
    });
    let ty_int = Core::OpTypeInt(OpTypeInt {
        result_id: ResultId(6),
        width: 32,
        signedness: Signedness::UnsignedOrNone,
    });
    let ty_vec = Core::OpTypeVector(OpTypeVector {
        result_id: ResultId(7),
        component_type: OpId(6),
        component_count: 3,
    });
    let int_32 = Core::OpConstant(OpConstant {
        result_type: OpId(6),
        result_id: ResultId(8),
        value: vec![32],
    });
    let int_32_x3 = vec![OpId(8), OpId(8), OpId(8)];
    let int_32x3 = Core::OpConstantComposite(OpConstantComposite {
        result_type: OpId(7),
        result_id: ResultId(9),
        constituents: int_32_x3,
    });
    let main = OpFunction {
        result_type: OpId(2),
        result_id: ResultId(4),
        function_control: FunctionControl::default(),
        function_type: OpId(3),
    };
    let instructions = vec![Core::OpCapability(OpCapability { capability: Capability::Shader }),
                            Core::OpExtInstImport(glsl450),
                            Core::OpMemoryModel(mem_model),
                            Core::OpEntryPoint(entry_point),
                            Core::OpExecutionMode(OpExecutionMode {
                                entry_point: OpId(4),
                                mode: local_size,
                            }),
                            Core::OpSource(source),
                            Core::OpName(OpName {
                                target: OpId(4),
                                name: "main".to_string(),
                            }),
                            Core::OpDecorate(workgroup),
                            ty_void,
                            ty_main,
                            ty_int,
                            ty_vec,
                            int_32,
                            int_32x3,
                            Core::OpFunction(main),
                            Core::OpLabel(OpLabel { result_id: ResultId(5) }),
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

#[test]
fn dis_write_multiply() {
    let raw_module = reader::read_module(WRITE_MULTIPLY_SPV)
        .expect("Failed to load write_multiply.spv");
    let disassembly = format!("{}", raw_module);
    for (dis, expect) in disassembly.lines().zip(WRITE_MULTIPLY_DIS.lines()) {
        assert_eq!(dis, expect);
    }
    assert_eq!(WRITE_MULTIPLY_DIS, disassembly);
}
