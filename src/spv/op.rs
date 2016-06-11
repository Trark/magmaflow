//! Instructions present in the core SPIR-V spec

use std::fmt;
use std::fmt::{Display, Formatter};
use super::types::*;
use super::ExtInstBox;
use super::dis::*;

#[derive(Clone, Debug, PartialEq)]
pub struct OpNop;

def_op_display!(OpNop;);

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

def_op_display!(OpSource; language | version | file | source);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSourceExtension {
    pub extension: LitString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpName {
    pub target: OpId,
    pub name: LitString,
}

def_op_display!(OpName; target | name);

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemberName {
    pub struct_type: OpId,
    pub member: MemberIndex,
    pub name: LitString,
}

def_op_display!(OpMemberName; struct_type | member | name);

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

def_op_display!(OpDecorate; target | decoration);

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemberDecorate {
    pub structure_type: OpId,
    pub member: MemberIndex,
    pub decoration: Decoration,
}

def_op_display!(OpMemberDecorate; structure_type | member | decoration);

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

def_op_display!(OpExtension; name);

#[derive(Clone, Debug, PartialEq)]
pub struct OpExtInstImport {
    pub result_id: ResultId,
    pub name: LitString,
}

def_op_display!(OpExtInstImport; result_id = name);

#[derive(Clone, Debug, PartialEq)]
pub struct OpExtInst {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub set: OpId,
    pub instruction: ExtInstBox,
}

def_op_display!(OpExtInst; result_id = result_type | set | instruction);

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemoryModel {
    pub addressing_model: AddressingModel,
    pub memory_model: MemoryModel,
}

def_op_display!(OpMemoryModel; addressing_model | memory_model);

#[derive(Clone, Debug, PartialEq)]
pub struct OpEntryPoint {
    pub execution_model: ExecutionModel,
    pub entry_point: OpId,
    pub name: LitString,
    pub interface: Vec<OpId>,
}

def_op_display!(OpEntryPoint; execution_model | entry_point | name | interface);

#[derive(Clone, Debug, PartialEq)]
pub struct OpExecutionMode {
    pub entry_point: OpId,
    pub mode: ExecutionMode,
}

def_op_display!(OpExecutionMode; entry_point | mode);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCapability {
    pub capability: Capability,
}

def_op_display!(OpCapability; capability);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeVoid {
    pub result_id: ResultId,
}

def_op_display!(OpTypeVoid; result_id =);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeBool {
    pub result_id: ResultId,
}

def_op_display!(OpTypeBool; result_id =);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeInt {
    pub result_id: ResultId,
    pub width: u32,
    pub signedness: Signedness,
}

def_op_display!(OpTypeInt; result_id = width | signedness);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeFloat {
    pub result_id: ResultId,
    pub width: u32,
}

def_op_display!(OpTypeFloat; result_id = width);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeVector {
    pub result_id: ResultId,
    pub component_type: OpId,
    pub component_count: u32,
}

def_op_display!(OpTypeVector; result_id = component_type | component_count);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeMatrix {
    pub result_id: ResultId,
    pub column_type: OpId,
    pub column_count: u32,
}

def_op_display!(OpTypeMatrix; result_id = column_type | column_count);

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

def_op_display!(OpTypeImage; result_id =
    sampled_type |
    dim |
    depth |
    arrayed |
    ms |
    sampled |
    format |
    access_qualifier
);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeSampler {
    pub result_id: ResultId,
}

def_op_display!(OpTypeSampler; result_id =);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeSampledImage {
    pub result_id: ResultId,
    pub image_type: OpId,
}

def_op_display!(OpTypeSampledImage; result_id = image_type);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeArray {
    pub result_id: ResultId,
    pub element_type: OpId,
    pub length: OpId,
}

def_op_display!(OpTypeArray; result_id = element_type | length);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeRuntimeArray {
    pub result_id: ResultId,
    pub element_type: OpId,
}

def_op_display!(OpTypeRuntimeArray; result_id = element_type);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeStruct {
    pub result_id: ResultId,
    pub member_types: Vec<OpId>,
}

def_op_display!(OpTypeStruct; result_id = member_types);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeOpaque {
    pub result_id: ResultId,
    pub name: LitString,
}

def_op_display!(OpTypeOpaque; result_id = name);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypePointer {
    pub result_id: ResultId,
    pub storage_class: StorageClass,
    pub pointed_type: OpId,
}

def_op_display!(OpTypePointer; result_id = storage_class | pointed_type);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeFunction {
    pub result_id: ResultId,
    pub return_type: OpId,
    pub parameter_types: Vec<OpId>,
}

def_op_display!(OpTypeFunction; result_id = return_type | parameter_types);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeEvent {
    pub result_id: ResultId,
}

def_op_display!(OpTypeEvent; result_id =);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeDeviceEvent {
    pub result_id: ResultId,
}

def_op_display!(OpTypeDeviceEvent; result_id =);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeReserveId {
    pub result_id: ResultId,
}

def_op_display!(OpTypeReserveId; result_id =);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeQueue {
    pub result_id: ResultId,
}

def_op_display!(OpTypeQueue; result_id =);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypePipe {
    pub result_id: ResultId,
    pub access_qualifier: AccessQualifier,
}

def_op_display!(OpTypePipe; result_id = access_qualifier);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeForwardPointer {
    pub pointer_type: OpId,
    pub storage_class: StorageClass,
}

def_op_display!(OpTypeForwardPointer; pointer_type | storage_class);

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

def_op_display!(OpConstant; result_id = result_type | value);

#[derive(Clone, Debug, PartialEq)]
pub struct OpConstantComposite {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub constituents: Vec<OpId>,
}

def_op_display!(OpConstantComposite; result_id = result_type | constituents);

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

def_op_display!(OpVariable; result_id = result_type | storage_class | initializer);

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

def_op_display!(OpLoad; result_id = result_type | pointer | memory_access);

#[derive(Clone, Debug, PartialEq)]
pub struct OpStore {
    pub pointer: OpId,
    pub object: OpId,
    pub memory_access: Option<MemoryAccess>,
}

def_op_display!(OpStore; pointer | object | memory_access);

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

def_op_display!(OpAccessChain; result_id = result_type | base | indexes);

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

def_op_display!(OpFunction; result_id = result_type | function_control | function_type);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunctionParameter {
    pub result_type: OpId,
    pub result_id: ResultId,
}

def_op_display!(OpFunctionParameter; result_id = result_type);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunctionEnd;

def_op_display!(OpFunctionEnd;);

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

def_op_display!(OpConvertUToF; result_id = result_type | unsigned_value);

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

def_op_display_s2!(OpIMul);

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

def_op_display_s2!(OpUMod);

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

def_op_display_s2!(OpIEqual);

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

def_op_display!(OpPhi; result_id = result_type | variables);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLoopMerge {
    pub merge_block: OpId,
    pub continue_target: OpId,
    pub loop_control: LoopControl,
}

def_op_display!(OpLoopMerge; merge_block | continue_target | loop_control);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSelectionMerge {
    pub merge_block: OpId,
    pub selection_control: SelectionControl,
}

def_op_display!(OpSelectionMerge; merge_block | selection_control);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLabel {
    pub result_id: ResultId,
}

def_op_display!(OpLabel; result_id =);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBranch {
    pub target_label: OpId,
}

def_op_display!(OpBranch; target_label);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBranchConditional {
    pub condition: OpId,
    pub true_label: OpId,
    pub false_label: OpId,
    pub weights: Option<BranchWeights>,
}

def_op_display!(OpBranchConditional; condition | true_label | false_label | weights);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSwitch(pub OpId, pub OpId, pub Vec<(LitBytes, OpId)>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpKill;

#[derive(Clone, Debug, PartialEq)]
pub struct OpReturn;

def_op_display!(OpReturn;);

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
