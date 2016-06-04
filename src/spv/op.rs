//! Instructions present in the core SPIR-V spec

use std::fmt;
use std::fmt::{Display, Formatter};
use super::types::*;
use super::ExtInstBox;
use super::dis::*;

#[derive(Clone, Debug, PartialEq)]
pub struct OpNop;

impl Display for OpNop {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpNop", NoResult)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpUndef {
    pub result_type: OpId,
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSourceContinued {
    pub continued_source: LitString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSource {
    pub language: SourceLanguage,
    pub version: SourceVersion,
    pub file: Option<OpId>,
    pub source: Option<LitString>,
}

impl Display for OpSource {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f,
                    "{}OpSource{}{}{}{}",
                    NoResult,
                    Arg(&self.language),
                    Arg(&self.version),
                    ArgOpt(&self.file),
                    ArgOpt(&self.source)));
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSourceExtension {
    pub extension: LitString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpName {
    pub target: OpId,
    pub name: LitString,
}

impl Display for OpName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpName{}{}",
               NoResult,
               Arg(&self.target),
               ArgString(&self.name))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemberName {
    pub struct_type: OpId,
    pub member: MemberIndex,
    pub name: LitString,
}

impl Display for OpMemberName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpMemberName{}{}{}",
               NoResult,
               Arg(&self.struct_type),
               Arg(&self.member.0),
               ArgString(&self.name))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpString {
    pub result_id: OpId,
    pub string: LitString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpLine {
    pub file: OpId,
    pub line: Line,
    pub column: Column,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpNoLine;

#[derive(Clone, Debug, PartialEq)]
pub struct OpDecorate {
    pub target: OpId,
    pub decoration: Decoration,
}

impl Display for OpDecorate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpDecorate{}{}",
               NoResult,
               Arg(&self.target),
               Arg(&self.decoration))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemberDecorate {
    pub structure_type: OpId,
    pub member: MemberIndex,
    pub decoration: Decoration,
}

impl Display for OpMemberDecorate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpMemberDecorate{}{}{}",
               NoResult,
               Arg(&self.structure_type),
               Arg(&self.member.0),
               Arg(&self.decoration))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpDecorationGroup {
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupDecorate {
    pub decoration_group: OpId,
    pub targets: Vec<OpId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupMemberDecorate {
    pub decoration_group: OpId,
    pub targets: Vec<(OpId, MemberIndex)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpExtension {
    pub name: LitString,
}

impl Display for OpExtension {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpExtension{}", NoResult, ArgString(&self.name))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpExtInstImport {
    pub result_id: ResultId,
    pub name: LitString,
}

impl Display for OpExtInstImport {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpExtInstImport{}",
               Result(&self.result_id),
               ArgString(&self.name))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpExtInst {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub set: OpId,
    pub instruction: ExtInstBox,
}

impl Display for OpExtInst {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpExtInst{}{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.set),
               Arg(&self.instruction))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemoryModel {
    pub addressing_model: AddressingModel,
    pub memory_model: MemoryModel,
}

impl Display for OpMemoryModel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpMemoryModel{}{}",
               NoResult,
               Arg(&self.addressing_model),
               Arg(&self.memory_model))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpEntryPoint {
    pub execution_model: ExecutionModel,
    pub entry_point: OpId,
    pub name: LitString,
    pub interface: Vec<OpId>,
}

impl Display for OpEntryPoint {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpEntryPoint{}{}{}{}",
               NoResult,
               Arg(&self.execution_model),
               Arg(&self.entry_point),
               ArgString(&self.name),
               ArgList(&self.interface))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpExecutionMode {
    pub entry_point: OpId,
    pub mode: ExecutionMode,
}

impl Display for OpExecutionMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpExecutionMode{}{}",
               NoResult,
               Arg(&self.entry_point),
               Arg(&self.mode))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpCapability {
    pub capability: Capability,
}

impl Display for OpCapability {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpCapability {}", NoResult, self.capability)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeVoid {
    pub result_id: ResultId,
}

impl Display for OpTypeVoid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeVoid", Result(&self.result_id))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeBool {
    pub result_id: ResultId,
}

impl Display for OpTypeBool {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeBool", Result(&self.result_id))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeInt {
    pub result_id: ResultId,
    pub width: u32,
    pub signedness: Signedness,
}

impl Display for OpTypeInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeInt{}{}",
               Result(&self.result_id),
               Arg(&self.width),
               Arg(&self.signedness))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeFloat {
    pub result_id: ResultId,
    pub width: u32,
}

impl Display for OpTypeFloat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeFloat{}",
               Result(&self.result_id),
               Arg(&self.width))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeVector {
    pub result_id: ResultId,
    pub component_type: OpId,
    pub component_count: u32,
}

impl Display for OpTypeVector {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeVector{}{}",
               Result(&self.result_id),
               Arg(&self.component_type),
               Arg(&self.component_count))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeMatrix {
    pub result_id: ResultId,
    pub column_type: OpId,
    pub column_count: u32,
}

impl Display for OpTypeMatrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeMatrix{}{}",
               Result(&self.result_id),
               Arg(&self.column_type),
               Arg(&self.column_count))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeImage {
    pub result_id: ResultId,
    pub sampled_type: OpId,
    pub dim: Dim,
    pub depth: DepthStatus,
    pub arrayed: Arrayed,
    pub ms: MS,
    pub sampled: SampledStatus,
    pub format: ImageFormat,
    pub access_qualifier: Option<AccessQualifier>,
}

impl Display for OpTypeImage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeImage{}{}{}{}{}{}{}{}",
               Result(&self.result_id),
               Arg(&self.sampled_type),
               Arg(&self.dim),
               Arg(&self.depth),
               Arg(&self.arrayed),
               Arg(&self.ms),
               Arg(&self.sampled),
               Arg(&self.format),
               ArgOpt(&self.access_qualifier))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeSampler {
    pub result_id: ResultId,
}

impl Display for OpTypeSampler {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeSampler", Result(&self.result_id))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeSampledImage {
    pub result_id: ResultId,
    pub image_type: OpId,
}

impl Display for OpTypeSampledImage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeSampledImage{}",
               Result(&self.result_id),
               Arg(&self.image_type))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeArray {
    pub result_id: ResultId,
    pub element_type: OpId,
    pub length: OpId,
}

impl Display for OpTypeArray {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeArray{}{}",
               Result(&self.result_id),
               Arg(&self.element_type),
               Arg(&self.length))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeRuntimeArray {
    pub result_id: ResultId,
    pub element_type: OpId,
}

impl Display for OpTypeRuntimeArray {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeRuntimeArray{}",
               Result(&self.result_id),
               Arg(&self.element_type))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeStruct {
    pub result_id: ResultId,
    pub member_types: Vec<OpId>,
}

impl Display for OpTypeStruct {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeStruct{}",
               Result(&self.result_id),
               ArgList(&self.member_types))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeOpaque {
    pub result_id: ResultId,
    pub name: LitString,
}

impl Display for OpTypeOpaque {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeOpaque{}",
               Result(&self.result_id),
               ArgString(&self.name))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypePointer {
    pub result_id: ResultId,
    pub storage_class: StorageClass,
    pub pointed_type: OpId,
}

impl Display for OpTypePointer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypePointer{}{}",
               Result(&self.result_id),
               Arg(&self.storage_class),
               Arg(&self.pointed_type))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeFunction {
    pub result_id: ResultId,
    pub return_type: OpId,
    pub parameter_types: Vec<OpId>,
}

impl Display for OpTypeFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeFunction{}{}",
               Result(&self.result_id),
               Arg(&self.return_type),
               ArgList(&self.parameter_types))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeEvent {
    pub result_id: ResultId,
}

impl Display for OpTypeEvent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeEvent", Result(&self.result_id))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeDeviceEvent {
    pub result_id: ResultId,
}

impl Display for OpTypeDeviceEvent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeDeviceEvent", Result(&self.result_id))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeReserveId {
    pub result_id: ResultId,
}

impl Display for OpTypeReserveId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeReserveId", Result(&self.result_id))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeQueue {
    pub result_id: ResultId,
}

impl Display for OpTypeQueue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeQueue", Result(&self.result_id))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypePipe {
    pub result_id: ResultId,
    pub access_qualifier: AccessQualifier,
}

impl Display for OpTypePipe {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypePipe", Result(&self.result_id))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeForwardPointer {
    pub pointer_type: OpId,
    pub storage_class: StorageClass,
}

impl Display for OpTypeForwardPointer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeForwardPointer{}",
               Arg(&self.pointer_type),
               Arg(&self.storage_class))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConstantTrue {
    pub result_type: OpId,
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConstantFalse {
    pub result_type: OpId,
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConstant {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub value: LitBytes,
}

impl Display for OpConstant {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpConstant{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               ArgList(&self.value))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConstantComposite {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub constituents: Vec<OpId>,
}

impl Display for OpConstantComposite {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpConstantComposite{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               ArgList(&self.constituents))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConstantSampler {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub addressing_mode: SamplerAddressingMode,
    pub param: SamplerParam,
    pub filter_mode: SamplerFilterMode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConstantNull {
    pub result_type: OpId,
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSpecConstantTrue {
    pub result_type: OpId,
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSpecConstantFalse {
    pub result_type: OpId,
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSpecConstant {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub value: LitBytes,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSpecConstantComposite {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub constituents: Vec<OpId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpVariable {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub storage_class: StorageClass,
    pub initializer: Option<OpId>,
}

impl Display for OpVariable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpVariable{}{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.storage_class),
               ArgOpt(&self.initializer))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageTexelPointer {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub image: OpId,
    pub coordinate: OpId,
    pub sample: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpLoad {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub pointer: OpId,
    pub memory_access: Option<MemoryAccess>,
}

impl Display for OpLoad {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpLoad{}{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.pointer),
               ArgOpt(&self.memory_access))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpStore {
    pub pointer: OpId,
    pub object: OpId,
    pub memory_access: Option<MemoryAccess>,
}

impl Display for OpStore {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpStore{}{}{}",
               NoResult,
               Arg(&self.pointer),
               Arg(&self.object),
               ArgOpt(&self.memory_access))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpCopyMemory {
    pub target: OpId,
    pub source: OpId,
    pub memory_access: Option<MemoryAccess>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpCopyMemorySized {
    pub target: OpId,
    pub source: OpId,
    pub size: OpId,
    pub memory_access: Option<MemoryAccess>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpAccessChain {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub base: OpId,
    pub indexes: Vec<OpId>,
}

impl Display for OpAccessChain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpAccessChain{}{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.base),
               ArgList(&self.indexes))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpInBoundsAccessChain {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub base: OpId,
    pub indexes: Vec<OpId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpPtrAccessChain {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub base: OpId,
    pub element: OpId,
    pub indexes: Vec<OpId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpArrayLength {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub structure: OpId,
    pub array_member: MemberIndex,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpGenericPtrMemSemantics {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub pointer: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpInBoundsPtrAccessChain {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub base: OpId,
    pub element: OpId,
    pub indexes: Vec<OpId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunction {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub function_control: FunctionControl,
    pub function_type: OpId,
}

impl Display for OpFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpFunction{}{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.function_control),
               Arg(&self.function_type))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunctionParameter {
    pub result_type: OpId,
    pub result_id: ResultId,
}

impl Display for OpFunctionParameter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpFunctionParameter{}",
               Result(&self.result_id),
               Arg(&self.result_type))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunctionEnd;

impl Display for OpFunctionEnd {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpFunctionEnd", NoResult)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunctionCall {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub function: OpId,
    pub arguments: Vec<OpId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSampledImage {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub image: OpId,
    pub sampler: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSampledImplicitLod(pub OpId,
                                     pub ResultId,
                                     pub OpId,
                                     pub OpId,
                                     pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSampledExplicitLod(pub OpId, pub ResultId, pub OpId, pub OpId, pub ImageOperands);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSampleDrefImplicitLod(pub OpId,
                                        pub ResultId,
                                        pub OpId,
                                        pub OpId,
                                        pub OpId,
                                        pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSampleDrefExplicitLod(pub OpId,
                                        pub ResultId,
                                        pub OpId,
                                        pub OpId,
                                        pub OpId,
                                        pub ImageOperands);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSampleProjImplicitLod(pub OpId,
                                        pub ResultId,
                                        pub OpId,
                                        pub OpId,
                                        pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSampleProjExplicitLod(pub OpId,
                                        pub ResultId,
                                        pub OpId,
                                        pub OpId,
                                        pub ImageOperands);


#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSampleProjDrefImplicitLod(pub OpId,
                                            pub ResultId,
                                            pub OpId,
                                            pub OpId,
                                            pub OpId,
                                            pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSampleProjDrefExplicitLod(pub OpId,
                                            pub ResultId,
                                            pub OpId,
                                            pub OpId,
                                            pub OpId,
                                            pub ImageOperands);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageFetch(pub OpId, pub ResultId, pub OpId, pub OpId, pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageGather(pub OpId,
                         pub ResultId,
                         pub OpId,
                         pub OpId,
                         pub OpId,
                         pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageDrefGather(pub OpId,
                             pub ResultId,
                             pub OpId,
                             pub OpId,
                             pub OpId,
                             pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageRead(pub OpId, pub ResultId, pub OpId, pub OpId, pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageWrite(pub OpId, pub OpId, pub OpId, pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImage(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageQueryFormat(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageQueryOrder(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageQuerySizeLod(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageQuerySize(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageQueryLod(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageQueryLevels(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageQuerySamples(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseSampleImplicitLod(pub OpId,
                                          pub ResultId,
                                          pub OpId,
                                          pub OpId,
                                          pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseSampleExplicitLod(pub OpId,
                                          pub ResultId,
                                          pub OpId,
                                          pub OpId,
                                          pub ImageOperands);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseSampleDrefImplicitLod(pub OpId,
                                              pub ResultId,
                                              pub OpId,
                                              pub OpId,
                                              pub OpId,
                                              pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseSampleDrefExplicitLod(pub OpId,
                                              pub ResultId,
                                              pub OpId,
                                              pub OpId,
                                              pub OpId,
                                              pub ImageOperands);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseSampleProjImplicitLod(pub OpId,
                                              pub ResultId,
                                              pub OpId,
                                              pub OpId,
                                              pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseSampleProjExplicitLod(pub OpId,
                                              pub ResultId,
                                              pub OpId,
                                              pub OpId,
                                              pub ImageOperands);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseSampleProjDrefImplicitLod(pub OpId,
                                                  pub ResultId,
                                                  pub OpId,
                                                  pub OpId,
                                                  pub OpId,
                                                  pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseSampleProjDrefExplicitLod(pub OpId,
                                                  pub ResultId,
                                                  pub OpId,
                                                  pub OpId,
                                                  pub OpId,
                                                  pub ImageOperands);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseFetch(pub OpId,
                              pub ResultId,
                              pub OpId,
                              pub OpId,
                              pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseGather(pub OpId,
                               pub ResultId,
                               pub OpId,
                               pub OpId,
                               pub OpId,
                               pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseDrefGather(pub OpId,
                                   pub ResultId,
                                   pub OpId,
                                   pub OpId,
                                   pub OpId,
                                   pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseTexelsResident(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageSparseRead(pub OpId, pub ResultId, pub OpId, pub OpId, pub Option<ImageOperands>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertFToU {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub float_value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertFToS {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub float_value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertSToF {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub signed_value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertUToF {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub unsigned_value: OpId,
}

impl Display for OpConvertUToF {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpConvertUToF{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.unsigned_value))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpUConvert {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub unsigned_value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSConvert {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub signed_value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFConvert {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub float_value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpQuantizeToF16 {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertPtrToU {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub pointer: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSatConvertSToU {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub signed_value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSatConvertUToS {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub unsigned_value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertUToPtr {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub integer_value: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpPtrCastToGeneric {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub pointer: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpGenericCastToPtr {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub pointer: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpGenericCastToPtrExplicit {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub pointer: OpId,
    pub storage_class: StorageClass,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitcast {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorExtractDynamic {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub vector: OpId,
    pub index: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorInsertDynamic {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub vector: OpId,
    pub component: OpId,
    pub index: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorShuffle {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
    pub components: Vec<MemberIndex>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpCompositeConstruct {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub constituents: Vec<MemberIndex>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpCompositeExtract {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub composite: OpId,
    pub indexes: Vec<MemberIndex>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpCompositeInsert {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub object: OpId,
    pub composite: OpId,
    pub indexes: Vec<MemberIndex>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpCopyObject {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTranspose {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub matrix: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSNegate {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFNegate {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpIAdd {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFAdd {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpISub {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFSub {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpIMul {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

impl Display for OpIMul {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpIMul{}{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.operand1),
               Arg(&self.operand2))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFMul {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpUDiv {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSDiv {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFDiv {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpUMod {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

impl Display for OpUMod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpUMod{}{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.operand1),
               Arg(&self.operand2))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSRem {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSMod {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFRem {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFMod {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorTimesScalar {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub vector: OpId,
    pub scalar: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpMatrixTimesScalar {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub matrix: OpId,
    pub scalar: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorTimesMatrix {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub vector: OpId,
    pub matrix: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpMatrixTimesVector {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub matrix: OpId,
    pub vector: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpMatrixTimesMatrix {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub left_matrix: OpId,
    pub right_matrix: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpOuterProduct {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub vector1: OpId,
    pub vector2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpDot {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub vector1: OpId,
    pub vector2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpIAddCarry {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpISubBorrow {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpUMulExtended {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSMulExtended {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpShiftRightLogical {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub base: OpId,
    pub shift: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpShiftRightArithmetic {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub base: OpId,
    pub shift: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpShiftLeftLogical {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub base: OpId,
    pub shift: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitwiseOr {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitwiseXor {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitwiseAnd {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpNot {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitFieldInsert(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitFieldSExtract(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitFieldUExtract(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitReverse(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitCount(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAny(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAll(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIsNan(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIsInf(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIsFinite(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIsNormal(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSignBitSet(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLessOrGreater(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpOrdered(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpUnordered(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLogicalEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLogicalNotEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLogicalOr(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLogicalAnd(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLogicalNot(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSelect(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

impl Display for OpIEqual {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpIEqual{}{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.operand1),
               Arg(&self.operand2))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpINotEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpUGreaterThan {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSGreaterThan {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpUGreaterThanEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSGreaterThanEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpULessThan {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSLessThan {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpULessThanEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSLessThanEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdNotEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordNotEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdLessThan {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordLessThan {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdGreaterThan {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordGreaterThan {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdLessThanEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordLessThanEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdGreaterThanEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordGreaterThanEqual {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub operand1: OpId,
    pub operand2: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpDPdx(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpDPdy(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFwidth(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpDPdxFine(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpDPdyFine(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFwidthFine(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpDPdxCoarse(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpDPdyCoarse(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFwidthCoarse(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpPhi {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub variables: Vec<PhiArg>,
}

impl Display for OpPhi {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpPhi{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               ArgList(&self.variables))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpLoopMerge {
    pub merge_block: OpId,
    pub continue_target: OpId,
    pub loop_control: LoopControl,
}

impl Display for OpLoopMerge {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpLoopMerge{}{}{}",
               NoResult,
               Arg(&self.merge_block),
               Arg(&self.continue_target),
               Arg(&self.loop_control))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSelectionMerge {
    pub merge_block: OpId,
    pub selection_control: SelectionControl,
}

impl Display for OpSelectionMerge {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpSelectionMerge{}{}",
               NoResult,
               Arg(&self.merge_block),
               Arg(&self.selection_control))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpLabel {
    pub result_id: ResultId,
}

impl Display for OpLabel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpLabel", Result(&self.result_id))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpBranch {
    pub target_label: OpId,
}

impl Display for OpBranch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpBranch{}", NoResult, Arg(&self.target_label))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpBranchConditional {
    pub condition: OpId,
    pub true_label: OpId,
    pub false_label: OpId,
    pub weights: Option<BranchWeights>,
}

impl Display for OpBranchConditional {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpBranchConditional{}{}{}{}",
               NoResult,
               Arg(&self.condition),
               Arg(&self.true_label),
               Arg(&self.false_label),
               ArgOpt(&self.weights))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpSwitch(pub OpId, pub OpId, pub Vec<(LitBytes, OpId)>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpKill;

#[derive(Clone, Debug, PartialEq)]
pub struct OpReturn;

impl Display for OpReturn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpReturn", NoResult)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpReturnValue(pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpUnreachable;

#[derive(Clone, Debug, PartialEq)]
pub struct OpLifetimeStart(pub OpId, pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLifetimeStop(pub OpId, pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicLoad(pub OpId, pub ResultId, pub OpId, pub ScopeId, pub MemorySemanticsId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicStore(pub OpId, pub ScopeId, pub MemorySemanticsId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicExchange(pub OpId,
                            pub ResultId,
                            pub OpId,
                            pub ScopeId,
                            pub MemorySemanticsId,
                            pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicCompareExchange(pub OpId,
                                   pub ResultId,
                                   pub OpId,
                                   pub ScopeId,
                                   pub MemorySemanticsId,
                                   pub MemorySemanticsId,
                                   pub OpId,
                                   pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicCompareExchangeWeak(pub OpId,
                                       pub ResultId,
                                       pub OpId,
                                       pub ScopeId,
                                       pub MemorySemanticsId,
                                       pub MemorySemanticsId,
                                       pub OpId,
                                       pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicIIncrement(pub OpId,
                              pub ResultId,
                              pub OpId,
                              pub ScopeId,
                              pub MemorySemanticsId,
                              pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicIDecrement(pub OpId,
                              pub ResultId,
                              pub OpId,
                              pub ScopeId,
                              pub MemorySemanticsId,
                              pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicIAdd(pub OpId,
                        pub ResultId,
                        pub OpId,
                        pub ScopeId,
                        pub MemorySemanticsId,
                        pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicISub(pub OpId,
                        pub ResultId,
                        pub OpId,
                        pub ScopeId,
                        pub MemorySemanticsId,
                        pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicSMin(pub OpId,
                        pub ResultId,
                        pub OpId,
                        pub ScopeId,
                        pub MemorySemanticsId,
                        pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicUMin(pub OpId,
                        pub ResultId,
                        pub OpId,
                        pub ScopeId,
                        pub MemorySemanticsId,
                        pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicSMax(pub OpId,
                        pub ResultId,
                        pub OpId,
                        pub ScopeId,
                        pub MemorySemanticsId,
                        pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicUMax(pub OpId,
                        pub ResultId,
                        pub OpId,
                        pub ScopeId,
                        pub MemorySemanticsId,
                        pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicAnd(pub OpId,
                       pub ResultId,
                       pub OpId,
                       pub ScopeId,
                       pub MemorySemanticsId,
                       pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicOr(pub OpId,
                      pub ResultId,
                      pub OpId,
                      pub ScopeId,
                      pub MemorySemanticsId,
                      pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicXor(pub OpId,
                       pub ResultId,
                       pub OpId,
                       pub ScopeId,
                       pub MemorySemanticsId,
                       pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicFlagTestAndSet(pub OpId,
                                  pub ResultId,
                                  pub OpId,
                                  pub ScopeId,
                                  pub MemorySemanticsId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAtomicFlagClear(pub OpId, pub ScopeId, pub MemorySemanticsId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpEmitVertex;

#[derive(Clone, Debug, PartialEq)]
pub struct OpEndPrimitive;

#[derive(Clone, Debug, PartialEq)]
pub struct OpEmitStreamVertex(pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpEndStreamPrimitive(pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpControlBarrier(pub ScopeId, pub ScopeId, pub MemorySemanticsId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemoryBarrier(pub ScopeId, pub MemorySemanticsId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpNamedBarrierInitialize(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemoryNamedBarrier(pub OpId, pub ScopeId, pub MemorySemanticsId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupAsyncCopy(pub OpId,
                            pub ResultId,
                            pub ScopeId,
                            pub OpId,
                            pub OpId,
                            pub OpId,
                            pub OpId,
                            pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupWaitEvents(pub ScopeId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupAll(pub OpId, pub ResultId, pub ScopeId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupAny(pub OpId, pub ResultId, pub ScopeId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupBroadcast(pub OpId, pub ResultId, pub ScopeId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupIAdd(pub OpId, pub ResultId, pub ScopeId, pub GroupOperation, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupFAdd(pub OpId, pub ResultId, pub ScopeId, pub GroupOperation, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupFMin(pub OpId, pub ResultId, pub ScopeId, pub GroupOperation, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupUMin(pub OpId, pub ResultId, pub ScopeId, pub GroupOperation, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupSMin(pub OpId, pub ResultId, pub ScopeId, pub GroupOperation, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupFMax(pub OpId, pub ResultId, pub ScopeId, pub GroupOperation, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupUMax(pub OpId, pub ResultId, pub ScopeId, pub GroupOperation, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupSMax(pub OpId, pub ResultId, pub ScopeId, pub GroupOperation, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpEnqueueMarker(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpEnqueueKernel(pub OpId,
                           pub ResultId,
                           pub OpId,
                           pub OpId,
                           pub OpId,
                           pub OpId,
                           pub OpId,
                           pub OpId,
                           pub OpId,
                           pub OpId,
                           pub OpId,
                           pub OpId,
                           pub Vec<(OpId, OpId)>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGetKernelNDrangeSubGroupCount(pub OpId,
                                           pub ResultId,
                                           pub OpId,
                                           pub OpId,
                                           pub OpId,
                                           pub OpId,
                                           pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGetKernelNDrangeMaxSubGroupSize(pub OpId,
                                             pub ResultId,
                                             pub OpId,
                                             pub OpId,
                                             pub OpId,
                                             pub OpId,
                                             pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGetKernelWorkGroupSize(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGetKernelPreferredWorkGroupSizeMultiple(pub OpId,
                                                     pub ResultId,
                                                     pub OpId,
                                                     pub OpId,
                                                     pub OpId,
                                                     pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpRetainEvent(pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpReleaseEvent(pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCreateUserEvent(pub OpId, pub ResultId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIsValidEvent(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSetUserEventStatus(pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCaptureEventProfilingInfo(pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGetDefaultQueue(pub OpId, pub ResultId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBuildNDRange(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGetKernelLocalSizeForSubgroupCount(pub OpId,
                                                pub ResultId,
                                                pub OpId,
                                                pub OpId,
                                                pub OpId,
                                                pub OpId,
                                                pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGetKernelMaxNumSubgroups(pub OpId,
                                      pub ResultId,
                                      pub OpId,
                                      pub OpId,
                                      pub OpId,
                                      pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpReadPipe(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpWritePipe(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpReservedReadPipe(pub OpId,
                              pub ResultId,
                              pub OpId,
                              pub OpId,
                              pub OpId,
                              pub OpId,
                              pub OpId,
                              pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpReservedWritePipe(pub OpId,
                               pub ResultId,
                               pub OpId,
                               pub OpId,
                               pub OpId,
                               pub OpId,
                               pub OpId,
                               pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpReserveReadPipePackets(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpReserveWritePipePackets(pub OpId,
                                     pub ResultId,
                                     pub OpId,
                                     pub OpId,
                                     pub OpId,
                                     pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCommitReadPipe(pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCommitWritePipe(pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIsValidReserveId(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGetNumPipePackets(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGetMaxPipePackets(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupReserveReadPipePackets(pub OpId,
                                         pub ResultId,
                                         pub ScopeId,
                                         pub OpId,
                                         pub OpId,
                                         pub OpId,
                                         pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupReserveWritePipePackets(pub OpId,
                                          pub ResultId,
                                          pub ScopeId,
                                          pub OpId,
                                          pub OpId,
                                          pub OpId,
                                          pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupCommitReadPipe(pub ScopeId, pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGroupCommitWritePipe(pub ScopeId, pub OpId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpConstantPipeStorage(pub OpId, pub ResultId, pub u32, pub u32, pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCreatePipeFromPipeStorage(pub OpId, pub ResultId, pub OpId);
