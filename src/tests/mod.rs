
use spv::*;
use spv::op::*;
use spv::types::*;
use spv::raw::*;
use spv::logical::*;
use glsl450;

const NOOP_SPV: &'static [u8] = include_bytes!("noop.spv");
const NOOP_DIS: &'static str = include_str!("noop.dis");
const WRITE_MULTIPLY_SPV: &'static [u8] = include_bytes!("write_multiply.spv");
const WRITE_MULTIPLY_DIS: &'static str = include_str!("write_multiply.dis");
const COND_TRIG_SPV: &'static [u8] = include_bytes!("cond_trig.spv");
const COND_TRIG_DIS: &'static str = include_str!("cond_trig.dis");
const NEST_IF_SPV: &'static [u8] = include_bytes!("nest_if.spv");
const NEST_IF_DIS: &'static str = include_str!("nest_if.dis");

fn read(module: &'static [u8]) -> ReadResult<RawModule> {
    let inst_sets: Vec<Box<ExtInstSet>> = vec![Box::new(glsl450::InstSet)];
    read_module(module, inst_sets)
}

#[test]
fn load_noop() {
    let result = read(NOOP_SPV);
    let glsl450 = OpExtInstImport {
        result_id: ResultId(1),
        name: "GLSL.std.450".into(),
    };
    let mem_model = OpMemoryModel {
        addressing_model: AddressingModel::Logical,
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
    let raw_module = read(NOOP_SPV).expect("Failed to load noop.spv");
    let disassembly = format!("{}", raw_module);
    for (dis, expect) in disassembly.lines().zip(NOOP_DIS.lines()) {
        assert_eq!(dis, expect);
    }
    assert_eq!(NOOP_DIS, disassembly);
}

#[test]
fn validate_noop() {
    let raw_module = read(NOOP_SPV).expect("Failed to load noop.spv");
    let module = validate(raw_module);
    module.unwrap();
}

#[test]
fn dis_write_multiply() {
    let raw_module = read(WRITE_MULTIPLY_SPV).expect("Failed to load write_multiply.spv");
    let disassembly = format!("{}", raw_module);
    for (dis, expect) in disassembly.lines().zip(WRITE_MULTIPLY_DIS.lines()) {
        assert_eq!(dis, expect);
    }
    assert_eq!(WRITE_MULTIPLY_DIS, disassembly);
}

#[test]
fn validate_write_multiply() {
    let raw_module = read(WRITE_MULTIPLY_SPV).expect("Failed to load write_multiply.spv");
    let module = validate(raw_module);
    module.unwrap();
}

#[test]
fn dis_cond_trig() {
    let raw_module = read(COND_TRIG_SPV).expect("Failed to load cond_trig.spv");
    let disassembly = format!("{}", raw_module);
    for (dis, expect) in disassembly.lines().zip(COND_TRIG_DIS.lines()) {
        assert_eq!(dis, expect);
    }
    assert_eq!(COND_TRIG_DIS, disassembly);
}

#[test]
fn validate_cond_trig() {
    let raw_module = read(COND_TRIG_SPV).expect("Failed to load cond_trig.spv");
    let module = validate(raw_module);
    module.unwrap();
}

#[test]
fn dis_next_if() {
    let raw_module = read(NEST_IF_SPV).expect("Failed to load nest_if.spv");
    let disassembly = format!("{}", raw_module);
    for (dis, expect) in disassembly.lines().zip(NEST_IF_DIS.lines()) {
        assert_eq!(dis, expect);
    }
    assert_eq!(NEST_IF_DIS, disassembly);
}

#[test]
fn validate_next_if() {
    let raw_module = read(NEST_IF_SPV).expect("Failed to load nest_if.spv");
    let module = validate(raw_module);
    module.unwrap();
}
