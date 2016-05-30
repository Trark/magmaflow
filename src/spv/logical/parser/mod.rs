
mod parser;
pub use self::parser::{validate, ValidationError};

use spv::op::*;
use spv::types::*;
use spv::raw::*;
use super::GroupDebug;
use super::GroupAnnotation;
use super::GroupType;
use super::GroupConstant;
use super::GroupGlobal;
use super::GroupCode;
use super::GroupMerge;
use super::GroupBranch;

/// Instructions grouped by where they fit in the logical module layout
#[derive(Clone, Debug, PartialEq)]
enum OpByBlock {
    OpCapability(OpCapability),
    OpExtension(OpExtension),
    OpExtInstImport(OpExtInstImport),
    OpMemoryModel(OpMemoryModel),
    OpEntryPoint(OpEntryPoint),
    OpExecutionMode(OpExecutionMode),
    GroupDebug(GroupDebug),
    GroupAnnotation(GroupAnnotation),
    GroupGlobal(GroupGlobal),
    OpFunction(OpFunction),
    OpFunctionParameter(OpFunctionParameter),
    OpFunctionEnd(OpFunctionEnd),
    OpLabel(OpLabel),
    GroupCode(GroupCode),
    GroupMerge(GroupMerge),
    GroupBranch(GroupBranch),
}

impl From<GroupDebug> for OpByBlock {
    fn from(inst: GroupDebug) -> OpByBlock {
        OpByBlock::GroupDebug(inst)
    }
}

impl From<GroupAnnotation> for OpByBlock {
    fn from(inst: GroupAnnotation) -> OpByBlock {
        OpByBlock::GroupAnnotation(inst)
    }
}

impl From<GroupGlobal> for OpByBlock {
    fn from(inst: GroupGlobal) -> OpByBlock {
        OpByBlock::GroupGlobal(inst)
    }
}

impl From<GroupType> for OpByBlock {
    fn from(inst: GroupType) -> OpByBlock {
        OpByBlock::GroupGlobal(GroupGlobal::GroupType(inst))
    }
}

impl From<GroupConstant> for OpByBlock {
    fn from(inst: GroupConstant) -> OpByBlock {
        OpByBlock::GroupGlobal(GroupGlobal::GroupConstant(inst))
    }
}

impl From<GroupCode> for OpByBlock {
    fn from(inst: GroupCode) -> OpByBlock {
        OpByBlock::GroupCode(inst)
    }
}

impl From<GroupMerge> for OpByBlock {
    fn from(inst: GroupMerge) -> OpByBlock {
        OpByBlock::GroupMerge(inst)
    }
}

impl From<GroupBranch> for OpByBlock {
    fn from(inst: GroupBranch) -> OpByBlock {
        OpByBlock::GroupBranch(inst)
    }
}

impl From<OpVariable> for OpByBlock {
    fn from(op: OpVariable) -> OpByBlock {
        match op.storage_class {
            StorageClass::Function => GroupCode::OpVariable(op).into(),
            _ => GroupGlobal::OpVariable(op).into(),
        }
    }
}

impl From<Core> for OpByBlock {
    fn from(inst: Core) -> OpByBlock {
        use spv::raw::Core::*;
        match inst {
            OpNop(op) => GroupCode::OpNop(op).into(),
            OpSource(op) => GroupDebug::OpSource(op).into(),
            OpName(op) => GroupDebug::OpName(op).into(),
            OpMemberName(op) => GroupDebug::OpMemberName(op).into(),
            OpExtension(op) => OpByBlock::OpExtension(op),
            OpExtInstImport(op) => OpByBlock::OpExtInstImport(op),
            OpMemoryModel(op) => OpByBlock::OpMemoryModel(op),
            OpEntryPoint(op) => OpByBlock::OpEntryPoint(op),
            OpExecutionMode(op) => OpByBlock::OpExecutionMode(op),
            OpCapability(op) => OpByBlock::OpCapability(op),
            OpTypeVoid(op) => GroupType::OpTypeVoid(op).into(),
            OpTypeBool(op) => GroupType::OpTypeBool(op).into(),
            OpTypeInt(op) => GroupType::OpTypeInt(op).into(),
            OpTypeFloat(op) => GroupType::OpTypeFloat(op).into(),
            OpTypeVector(op) => GroupType::OpTypeVector(op).into(),
            OpTypeMatrix(op) => GroupType::OpTypeMatrix(op).into(),
            OpTypeImage(op) => GroupType::OpTypeImage(op).into(),
            OpTypeSampler(op) => GroupType::OpTypeSampler(op).into(),
            OpTypeSampledImage(op) => GroupType::OpTypeSampledImage(op).into(),
            OpTypeArray(op) => GroupType::OpTypeArray(op).into(),
            OpTypeRuntimeArray(op) => GroupType::OpTypeRuntimeArray(op).into(),
            OpTypeStruct(op) => GroupType::OpTypeStruct(op).into(),
            OpTypeOpaque(op) => GroupType::OpTypeOpaque(op).into(),
            OpTypePointer(op) => GroupType::OpTypePointer(op).into(),
            OpTypeFunction(op) => GroupType::OpTypeFunction(op).into(),
            OpTypeEvent(op) => GroupType::OpTypeEvent(op).into(),
            OpTypeDeviceEvent(op) => GroupType::OpTypeDeviceEvent(op).into(),
            OpTypeQueue(op) => GroupType::OpTypeQueue(op).into(),
            OpTypePipe(op) => GroupType::OpTypePipe(op).into(),
            OpTypeForwardPointer(op) => GroupType::OpTypeForwardPointer(op).into(),
            OpConstant(op) => GroupConstant::OpConstant(op).into(),
            OpConstantComposite(op) => GroupConstant::OpConstantComposite(op).into(),
            OpFunction(op) => OpByBlock::OpFunction(op),
            OpFunctionParameter(op) => OpByBlock::OpFunctionParameter(op),
            OpFunctionEnd(op) => OpByBlock::OpFunctionEnd(op),
            OpVariable(op) => op.into(),
            OpLoad(op) => GroupCode::OpLoad(op).into(),
            OpStore(op) => GroupCode::OpStore(op).into(),
            OpAccessChain(op) => GroupCode::OpAccessChain(op).into(),
            OpDecorate(op) => GroupAnnotation::OpDecorate(op).into(),
            OpMemberDecorate(op) => GroupAnnotation::OpMemberDecorate(op).into(),
            OpConvertUToF(op) => GroupCode::OpConvertUToF(op).into(),
            OpIMul(op) => GroupCode::OpIMul(op).into(),
            OpUMod(op) => GroupCode::OpUMod(op).into(),
            OpIEqual(op) => GroupCode::OpIEqual(op).into(),
            OpPhi(op) => GroupCode::OpPhi(op).into(),
            OpLoopMerge(op) => GroupMerge::OpLoopMerge(op).into(),
            OpSelectionMerge(op) => GroupMerge::OpSelectionMerge(op).into(),
            OpLabel(op) => OpByBlock::OpLabel(op),
            OpBranch(op) => GroupBranch::OpBranch(op).into(),
            OpBranchConditional(op) => GroupBranch::OpBranchConditional(op).into(),
            OpReturn(op) => GroupBranch::OpReturn(op).into(),
        }
    }
}
