
mod parser;
pub use self::parser::{validate, ValidationError};

mod control_flow;
pub use self::control_flow::{BlockId, ControlFlowChain, ControlFlowFunctionPrinter};
pub use self::control_flow::{ControlType, ControlFlowError, ControlFlowResult};
pub use self::control_flow::find_control_flow;

use super::op::*;
use super::types::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum GroupDebug {
    OpSourceContinued(OpSourceContinued),
    OpSource(OpSource),
    OpSourceExtension(OpSourceExtension),
    OpName(OpName),
    OpMemberName(OpMemberName),
    OpString(OpString),
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
    OpUndef(OpUndef),
    OpExtInst(OpExtInst),
    /// Variables inside blocks must have a storage class of Function
    OpVariable(OpVariable),
    OpLoad(OpLoad),
    OpStore(OpStore),
    OpAccessChain(OpAccessChain),
    OpConvertUToF(OpConvertUToF),
    OpBitcast(OpBitcast),
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

impl fmt::Display for GroupCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Display;
        match *self {
            GroupCode::OpNop(ref op) => Display::fmt(op, f),
            GroupCode::OpUndef(ref op) => Display::fmt(op, f),
            GroupCode::OpExtInst(ref op) => Display::fmt(op, f),
            GroupCode::OpVariable(ref op) => Display::fmt(op, f),
            GroupCode::OpLoad(ref op) => Display::fmt(op, f),
            GroupCode::OpStore(ref op) => Display::fmt(op, f),
            GroupCode::OpAccessChain(ref op) => Display::fmt(op, f),
            GroupCode::OpConvertUToF(ref op) => Display::fmt(op, f),
            GroupCode::OpBitcast(ref op) => Display::fmt(op, f),
            GroupCode::OpIAdd(ref op) => Display::fmt(op, f),
            GroupCode::OpFAdd(ref op) => Display::fmt(op, f),
            GroupCode::OpISub(ref op) => Display::fmt(op, f),
            GroupCode::OpFSub(ref op) => Display::fmt(op, f),
            GroupCode::OpIMul(ref op) => Display::fmt(op, f),
            GroupCode::OpFMul(ref op) => Display::fmt(op, f),
            GroupCode::OpUDiv(ref op) => Display::fmt(op, f),
            GroupCode::OpSDiv(ref op) => Display::fmt(op, f),
            GroupCode::OpFDiv(ref op) => Display::fmt(op, f),
            GroupCode::OpUMod(ref op) => Display::fmt(op, f),
            GroupCode::OpSRem(ref op) => Display::fmt(op, f),
            GroupCode::OpSMod(ref op) => Display::fmt(op, f),
            GroupCode::OpFRem(ref op) => Display::fmt(op, f),
            GroupCode::OpFMod(ref op) => Display::fmt(op, f),
            GroupCode::OpIAddCarry(ref op) => Display::fmt(op, f),
            GroupCode::OpISubBorrow(ref op) => Display::fmt(op, f),
            GroupCode::OpUMulExtended(ref op) => Display::fmt(op, f),
            GroupCode::OpSMulExtended(ref op) => Display::fmt(op, f),
            GroupCode::OpBitwiseOr(ref op) => Display::fmt(op, f),
            GroupCode::OpBitwiseXor(ref op) => Display::fmt(op, f),
            GroupCode::OpBitwiseAnd(ref op) => Display::fmt(op, f),
            GroupCode::OpIEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpINotEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpUGreaterThan(ref op) => Display::fmt(op, f),
            GroupCode::OpSGreaterThan(ref op) => Display::fmt(op, f),
            GroupCode::OpUGreaterThanEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpSGreaterThanEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpULessThan(ref op) => Display::fmt(op, f),
            GroupCode::OpSLessThan(ref op) => Display::fmt(op, f),
            GroupCode::OpULessThanEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpSLessThanEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpFOrdEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpFUnordEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpFOrdNotEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpFUnordNotEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpFOrdLessThan(ref op) => Display::fmt(op, f),
            GroupCode::OpFUnordLessThan(ref op) => Display::fmt(op, f),
            GroupCode::OpFOrdGreaterThan(ref op) => Display::fmt(op, f),
            GroupCode::OpFUnordGreaterThan(ref op) => Display::fmt(op, f),
            GroupCode::OpFOrdLessThanEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpFUnordLessThanEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpFOrdGreaterThanEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpFUnordGreaterThanEqual(ref op) => Display::fmt(op, f),
            GroupCode::OpPhi(ref op) => Display::fmt(op, f),
        }
    }
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

impl fmt::Display for GroupBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Display;
        match *self {
            GroupBranch::OpBranch(ref op) => Display::fmt(op, f),
            GroupBranch::OpBranchConditional(ref op) => Display::fmt(op, f),
            GroupBranch::OpReturn(ref op) => Display::fmt(op, f),
        }
    }
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
