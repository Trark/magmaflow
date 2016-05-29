
use super::op::*;
use super::types::*;

#[derive(Clone, Debug, PartialEq)]
pub enum GroupDebug {
    OpSource(OpSource),
    OpName(OpName),
    OpMemberName(OpMemberName),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupAnnotation {
    OpDecorate(OpDecorate),
    OpMemberDecorate(OpMemberDecorate),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupType {
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupConstant {
    OpConstant(OpConstant),
    OpConstantComposite(OpConstantComposite),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupGlobal {
    GroupType(GroupType),
    GroupConstant(GroupConstant),
    /// Variables as globals must have a storage class that is not Function
    OpVariable(OpVariable),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupCode {
    OpNop(OpNop),
    /// Variables inside blocks must have a storage class of Function
    OpVariable(OpVariable),
    OpLoad(OpLoad),
    OpStore(OpStore),
    OpAccessChain(OpAccessChain),
    OpConvertUToF(OpConvertUToF),
    OpIMul(OpIMul),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupBranch {
    OpBranch(OpBranch),
    OpReturn(OpReturn),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub function: OpFunction,
    pub parameters: Vec<OpFunctionParameter>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BasicBlock {
    pub label: OpLabel,
    pub code: Vec<GroupCode>,
    pub branch: GroupBranch,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition {
    pub function: OpFunction,
    pub parameters: Vec<OpFunctionParameter>,
    pub blocks: Vec<BasicBlock>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogicalModule {
    pub capabilities: Vec<Capability>,
    pub extensions: Vec<String>,
    pub ext_inst_imports: Vec<OpExtInstImport>,
    pub memory_model: OpMemoryModel,
    pub entry_points: Vec<OpEntryPoint>,
    pub execution_modes: Vec<OpExecutionMode>,
    pub debug: Vec<GroupDebug>,
    pub annotations: Vec<GroupAnnotation>,
    pub globals: Vec<GroupGlobal>,
    pub function_declarations: Vec<FunctionDeclaration>,
    pub function_definitions: Vec<FunctionDefinition>,
}
