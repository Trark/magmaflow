
use spv::*;
use spv::op::*;
use spv::types::*;
use spv::raw::*;
use glsl450;

fn read(module: &'static [u8]) -> ReadResult<RawModule> {
    let inst_sets: Vec<Box<ExtInstSet>> = vec![Box::new(glsl450::InstSet)];
    read_module(module, inst_sets)
}

#[test]
fn load_noop() {
    let result = read(include_bytes!("noop.spv"));
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

macro_rules! def_test {
    ($name: ident) => {
        mod $name {

            use super::read;
            use spv::logical::validate;

            const SPV: &'static [u8] = include_bytes!(concat!(stringify!($name), ".spv"));
            const DIS: &'static str = include_str!(concat!(stringify!($name), ".dis"));

            #[test]
            fn disassemble() {
                let raw_module = read(SPV).expect("Failed to load spv");
                let disassembly = format!("{}", raw_module);
                for (dis, expect) in disassembly.lines().zip(DIS.lines()) {
                    assert_eq!(dis, expect);
                }
                assert_eq!(DIS, disassembly);
            }

            #[test]
            fn logical_pass() {
                let raw_module = read(SPV).expect("Failed to load spv");
                let module = validate(raw_module);
                module.unwrap();
            }
        }
    }
}

def_test!(noop);
def_test!(write_multiply);
def_test!(cond_trig);
def_test!(nest_if);
