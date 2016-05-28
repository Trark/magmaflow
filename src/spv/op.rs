
use super::types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct OpNop;

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

#[derive(Clone, Debug, PartialEq)]
pub struct OpSourceExtension {
    pub extension: LitString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpName {
    pub target: OpId,
    pub name: LitString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemberName {
    pub struct_type: OpId,
    pub member: u32,
    pub name: LitString,
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

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemberDecorate {
    pub structure_type: OpId,
    pub member: MemberIndex,
    pub decoration: Decoration,
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

#[derive(Clone, Debug, PartialEq)]
pub struct OpExtInstImport {
    pub result_id: ResultId,
    pub name: LitString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpExtInst {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub set: OpId,
    pub instruction: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpMemoryModel {
    pub addressing_mode: AddressingMode,
    pub memory_model: MemoryModel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpEntryPoint {
    pub execution_model: ExecutionModel,
    pub entry_point: OpId,
    pub name: LitString,
    pub interface: Vec<OpId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpExecutionMode {
    pub entry_point: OpId,
    pub mode: ExecutionMode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpCapability {
    pub capability: Capability,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeVoid {
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeBool {
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeInt {
    pub result_id: ResultId,
    pub width: u32,
    pub signedness: Signedness,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeFloat {
    pub result_id: ResultId,
    pub width: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeVector {
    pub result_id: ResultId,
    pub component_type: OpId,
    pub component_count: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeMatrix {
    pub result_type: ResultId,
    pub column_type: OpId,
    pub column_count: u32,
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

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeSampler {
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeSampledImage {
    pub result_id: ResultId,
    pub image_type: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeArray {
    pub result_id: ResultId,
    pub element_type: OpId,
    pub length: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeRuntimeArray {
    pub result_id: ResultId,
    pub element_type: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeStruct {
    pub result_id: ResultId,
    pub member_types: Vec<OpId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeOpaque {
    pub result_id: ResultId,
    pub name: LitString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypePointer {
    pub result_id: ResultId,
    pub storage_class: StorageClass,
    pub pointed_type: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeFunction {
    pub result_id: ResultId,
    pub return_type: OpId,
    pub parameter_types: Vec<OpId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeEvent {
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeDeviceEvent {
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeReserveId {
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeQueue {
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypePipe {
    pub result_id: ResultId,
    pub access_qualifier: AccessQualifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpTypeForwardPointer {
    pub pointer_type: OpId,
    pub storage_class: StorageClass,
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

#[derive(Clone, Debug, PartialEq)]
pub struct OpConstantComposite {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub constituents: Vec<OpId>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct OpImageTexelPointer(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLoad(pub OpId, pub ResultId, pub OpId, pub Option<MemoryAccess>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpStore(pub OpId, pub OpId, pub Option<MemoryAccess>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCopyMemory(pub OpId, pub OpId, pub Option<MemoryAccess>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCopyMemorySized(pub OpId, pub OpId, pub OpId, pub Option<MemoryAccess>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpAccessChain(pub OpId, pub ResultId, pub OpId, pub Vec<OpId>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpInBoundsAccessChain(pub OpId, pub ResultId, pub OpId, pub Vec<OpId>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpPtrAccessChain(pub OpId, pub ResultId, pub OpId, pub OpId, pub Vec<OpId>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpArrayLength(pub OpId, pub ResultId, pub OpId, pub MemberIndex);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGenericPtrMemSemantics(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpInBoundsPtrAccessChain(pub OpId, pub ResultId, pub OpId, pub OpId, pub Vec<OpId>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunction {
    pub result_type: OpId,
    pub result_id: ResultId,
    pub function_control: FunctionControl,
    pub function_type: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunctionParameter {
    pub result_type: OpId,
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunctionEnd;

#[derive(Clone, Debug, PartialEq)]
pub struct OpFunctionCall(pub OpId, pub ResultId, pub OpId, pub Vec<OpId>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSampledImage(pub OpId, pub ResultId, pub OpId, pub OpId);

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
pub struct OpConvertFToU(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertFToS(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertSToF(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertUToF(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpUConvert(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSConvert(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFConvert(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpQuantizeToF16(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertPtrToU(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSatConvertSToU(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSatConvertUToS(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpConvertUToPtr(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpPtrCastToGeneric(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGenericCastToPtr(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpGenericCastToPtrExplicit(pub OpId, pub ResultId, pub OpId, pub StorageClass);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitcast(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorExtractDynamic(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorInsertDynamic(pub OpId, pub ResultId, pub OpId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorShuffle(pub OpId, pub ResultId, pub OpId, pub OpId, pub Vec<MemberIndex>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCompositeConstruct(pub OpId, pub ResultId, pub OpId, pub Vec<OpId>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCompositeExtract(pub OpId, pub ResultId, pub OpId, pub Vec<MemberIndex>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCompositeInsert(pub OpId, pub ResultId, pub OpId, pub OpId, pub Vec<MemberIndex>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpCopyObject(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpTranspose(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSNegate(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFNegate(pub OpId, pub ResultId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIAdd(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFAdd(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpISub(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFSub(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIMul(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFMul(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpUDiv(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSDiv(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFDiv(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpUMod(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSRem(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSMod(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFRem(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFMod(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorTimesScalar(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpMatrixTimesScalar(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpVectorTimesMatrix(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpMatrixTimesVector(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpMatrixTimesMatrix(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpOuterProduct(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpDot(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpIAddCarry(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpISubBorrow(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpUMulExtended(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSMulExtended(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpShiftRightLogical(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpShiftRightArithmetic(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpShiftLeftLogical(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitwiseOr(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitwiseXor(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpBitwiseAnd(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpNot(pub OpId, pub ResultId, pub OpId);

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
pub struct OpIEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpINotEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpUGreaterThan(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSGreaterThan(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpUGreaterThanEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSGreaterThanEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpULessThan(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSLessThan(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpULessThanEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSLessThanEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdNotEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordNotEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdLessThan(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordLessThan(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdGreaterThan(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordGreaterThan(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdLessThanEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordLessThanEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFOrdGreaterThanEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

#[derive(Clone, Debug, PartialEq)]
pub struct OpFUnordGreaterThanEqual(pub OpId, pub ResultId, pub OpId, pub OpId);

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
pub struct OpPhi(pub OpId, pub ResultId, pub Vec<OpId>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLoopMerge(pub OpId, pub OpId, pub LoopControl);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSelectionMerge(pub OpId, pub SelectionControl);

#[derive(Clone, Debug, PartialEq)]
pub struct OpLabel {
    pub result_id: ResultId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpBranch {
    pub target_label: OpId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpBranchConditional(pub OpId, pub OpId, pub OpId, pub Option<(u32, u32)>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpSwitch(pub OpId, pub OpId, pub Vec<(LitBytes, OpId)>);

#[derive(Clone, Debug, PartialEq)]
pub struct OpKill;

#[derive(Clone, Debug, PartialEq)]
pub struct OpReturn;

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
