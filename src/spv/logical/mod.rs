
mod parser;
pub use self::parser::{validate, ValidationError};

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
    OpExtInst(OpExtInst),
    /// Variables inside blocks must have a storage class of Function
    OpVariable(OpVariable),
    OpLoad(OpLoad),
    OpStore(OpStore),
    OpAccessChain(OpAccessChain),
    OpConvertUToF(OpConvertUToF),
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupMerge {
    OpLoopMerge(OpLoopMerge),
    OpSelectionMerge(OpSelectionMerge),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupBranch {
    OpBranch(OpBranch),
    OpBranchConditional(OpBranchConditional),
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
    pub merge: Option<GroupMerge>,
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
