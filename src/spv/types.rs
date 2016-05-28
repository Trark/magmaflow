
pub type Word = u32;

#[derive(Debug, PartialEq)]
pub struct OpId(pub Word);

#[derive(Debug, PartialEq)]
pub struct ResultId(pub Word);

pub type WordNumber = Word;
pub type LitNumber = Vec<u32>;
pub type LitBytes = Vec<u32>;
pub type LitString = String;

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(LitString),
    Number(LitNumber),
}

#[derive(Debug, PartialEq)]
pub struct Version(pub u8, pub u8);

#[derive(Debug, PartialEq)]
pub struct Generator {
    pub tool: Tool,
    pub version: u16,
}

#[derive(Debug, PartialEq)]
pub enum Tool {
    KhronosReserved,
    LunarG,
    Valve,
    Codeplay,
    Nvidia,
    Arm,
    KhronosLLvmTranslator,
    KhronosAssembler,
    KhronosGlslang,
    Qualcomm,
    Amd,
    Intel,
    Other(u16),
}

impl Tool {
    pub fn get_vendor(&self) -> Option<&'static str> {
        Some(match *self {
            Tool::KhronosReserved => "Khronos",
            Tool::LunarG => "LunarG",
            Tool::Valve => "Valve",
            Tool::Codeplay => "Codeplay",
            Tool::Nvidia => "NVIDIA",
            Tool::Arm => "ARM",
            Tool::KhronosLLvmTranslator => "Khronos",
            Tool::KhronosAssembler => "Khronos",
            Tool::KhronosGlslang => "Khronos",
            Tool::Qualcomm => "Qualcomm",
            Tool::Amd => "AMD",
            Tool::Intel => "Intel",
            Tool::Other(_) => return None,
        })
    }
    pub fn get_tool(&self) -> Option<&'static str> {
        Some(match *self {
            Tool::KhronosLLvmTranslator => "LLVM/SPIR-V Translator",
            Tool::KhronosAssembler => "SPIR-V Tools Assembler",
            Tool::KhronosGlslang => "Glslang Reference Front End",
            _ => return None,
        })
    }
}

/// Source language the module was created from
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum SourceLanguage {
    Unknown,
    Essl,
    Glsl,
    OpenCL_C,
    OpenCL_Cpp,
    Other(Word),
}

#[derive(Debug, PartialEq)]
pub struct Line(pub u32);

#[derive(Debug, PartialEq)]
pub struct Column(pub u32);

#[derive(Debug, PartialEq)]
pub enum Decoration {
    RelaxedPrecision,
    SpecId(u32),
    Block,
    BufferBlock,
    RowMajor,
    ColMajor,
    ArrayStride(u32),
    MatrixStride(u32),
    GlslShared,
    GlslPacked,
    CPacked,
    BuiltIn(BuiltIn),
    NoPerspective,
    Flat,
    Patch,
    Centroid,
    Sample,
    Invariant,
    Restrict,
    Aliased,
    Volatile,
    Constant,
    Coherent,
    NonWritable,
    NonReadable,
    Uniform,
    SaturatedConversion,
    Stream(u32),
    Location(u32),
    Component(u32),
    Index(u32),
    Binding(u32),
    DescriptorSet(u32),
    Offset(u32),
    XfbBuffer(u32),
    XfbStride(u32),
    FuncParamAttr(FunctionParameterAttribute),
    FpRoundingMode(FpRoundingMode),
    FpFastMathMode(FpFastMathMode),
    LinkageAttributes(LitString, LinkageType),
    NoContraction,
    InputAttachmentIndex(u32),
    Alignment(u32),
}

#[derive(Debug, PartialEq)]
pub enum BuiltIn {
    Position,
    PointSize,
    ClipDistance,
    CullDistance,
    VertexId,
    InstanceId,
    PrimitiveId,
    InvocationId,
    Layer,
    ViewportIndex,
    TessLevelOuter,
    TessLevelInner,
    TessCoord,
    PatchVerticies,
    FragCoord,
    PointCoord,
    FrontFacing,
    SampleId,
    SamplePosition,
    SampleMask,
    FragDepth,
    HelperInvocation,
    NumWorkgroups,
    WorkgroupSize,
    WorkgroupId,
    LocalInvocationId,
    GlobalInvocationId,
    LocalInvocationIndex,
    WorkDim,
    GlobalSize,
    EnqueuedWorkgroupSize,
    GlobalOffset,
    GlobalLinearId,
    SubgroupSize,
    SubgroupMaxSize,
    NumSubgroups,
    NumEnqueuedSubgroups,
    SubgroupId,
    SubgroupLocalInvocationId,
    VertexIndex,
    InstanceIndex,
}

#[derive(Debug, PartialEq)]
pub struct MemberIndex(pub u32);

#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Logical,
    Physical32,
    Physical64,
}

#[derive(Debug, PartialEq)]
pub enum MemoryModel {
    Simple,
    Glsl450,
    OpenCL,
}

#[derive(Debug, PartialEq)]
pub enum ExecutionModel {
    Vertex,
    TesselationControl,
    TesselationEvaluation,
    Geometry,
    Fragment,
    GlCompute,
    Kernel,
}

#[derive(Debug, PartialEq)]
pub enum ExecutionMode {
    Invocations(u32),
    SpacingEqual,
    SpacingFractionalEven,
    SpacingFractionalOdd,
    VertexOrderCw,
    VertexOrderCcw,
    PixelCenterInteger,
    OriginUpperLeft,
    OriginLowerLeft,
    EarlyFragmentTests,
    PointMode,
    Xfb,
    DepthReplacing,
    DepthGreater,
    DepthLess,
    DepthUnchanged,
    LocalSize(u32, u32, u32),
    LocalSizeHint(u32, u32, u32),
    InputPoints,
    InputLines,
    InputLinesAdjacency,
    Triangles,
    InputTrianglesAdjacency,
    Quads,
    Isolines,
    OutputVerticies(u32),
    OutputPoints,
    OutputLineStrip,
    OutputTriangleStrip,
    VecTypeHint(OpId),
    ContractionOff,
}

#[derive(Debug, PartialEq)]
pub enum Capability {
    Matrix,
    Shader,
    Geometry,
    Tessellation,
    Addresses,
    Linkage,
    Kernel,
    Vector16,
    Float16Buffer,
    Float16,
    Float64,
    Int64,
    Int64Atomics,
    ImageBasic,
    ImageReadWrite,
    ImageMipmap,
    Pipes,
    Groups,
    DeviceEnqueue,
    LiteralSampler,
    AtomicStorage,
    Int16,
    TessellationPointSize,
    GeometryPointSize,
    ImageGatherExtended,
    StorageImageMultisample,
    UniformBufferArrayDynamicIndexing,
    SampledImageArrayDynamicIndexing,
    StorageBufferArrayDynamicIndexing,
    StorageImageArrayDynamicIndexing,
    ClipDistance,
    CullDistance,
    ImageCubeArray,
    SampleRateShading,
    ImageRect,
    SampledRect,
    GenericPointer,
    Int8,
    InputAttachment,
    SparseResidency,
    MinLod,
    Sampled1D,
    Image1D,
    SampledCubeArray,
    SampledBuffer,
    ImageBuffer,
    ImageMSArray,
    StorageImageExtendedFormats,
    ImageQuery,
    DerivativeControl,
    InterpolationFunction,
    TransformFeedback,
    GeometryStreams,
    StorageImageReadWithoutFormat,
    StorageImageWriteWithoutFormat,
    MultiViewport,
}

#[derive(Debug, PartialEq)]
pub enum StorageClass {
    UniformConstant,
    Input,
    Uniform,
    Output,
    WorkgroupLocal,
    WorkgroupGlobal,
    PrivateGlobal,
    Function,
    Generic,
    Private,
    AtomicCounter,
}

#[derive(Debug, PartialEq)]
pub struct FpFastMathMode {
    pub not_nan: bool,
    pub not_inf: bool,
    pub nsz: bool,
    pub allow_recip: bool,
    pub fast: bool,
}

#[derive(Debug, PartialEq)]
pub enum FpRoundingMode {
    Rte,
    Rtz,
    Rtp,
    Rtn,
}

#[derive(Debug, PartialEq)]
pub enum LinkageType {
    Export,
    Import,
}

#[derive(Debug, PartialEq)]
pub enum FunctionParameterAttribute {
    Zext,
    Sext,
    ByVal,
    Sret,
    NoAlias,
    NoCapture,
    NoWrite,
    NoReadWrite,
}

#[derive(Debug, PartialEq)]
pub enum Dim {
    Tex1D,
    Tex2D,
    Tex3D,
    Cube,
    Rect,
    Buffer,
    SubpassData,
}

#[derive(Debug, PartialEq)]
pub enum DepthStatus {
    NotDepth,
    Depth,
    NoIndication,
}

#[derive(Debug, PartialEq)]
pub enum Arrayed {
    False,
    True,
}

#[derive(Debug, PartialEq)]
pub enum MS {
    Single,
    Multi,
}

#[derive(Debug, PartialEq)]
pub enum SampledStatus {
    RuntimeChoice,
    WithSampler,
    WithoutSampler,
}

#[derive(Debug, PartialEq)]
pub enum ImageFormat {
    Unknown,
    Rgba32f,
    Rgba16f,
    R32f,
    Rgba8,
    Rgba8Snorm,
    Rg32f,
    Rg16f,
    R11fG11fB10f,
    R16f,
    Rgba16,
    Rgb10A2,
    Rg16,
    Rg8,
    R16,
    R8,
    Rgba16Snorm,
    Rg16Snorm,
    Rg8Snorm,
    R16Snorm,
    R8Snorm,
    Rgba32i,
    Rgba16i,
    Rgba8i,
    R32i,
    Rg32i,
    Rg16i,
    Rg8i,
    R16i,
    R8i,
    Rgba32ui,
    Rgba16ui,
    Rgba8ui,
    Rgb10a2ui,
    Rg32ui,
    Rg16ui,
    Rg8ui,
    R16ui,
    R8ui,
}

#[derive(Debug, PartialEq)]
pub enum AccessQualifier {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Debug, PartialEq)]
pub enum SamplerAddressingMode {
    None,
    ClampEdge,
    Clamp,
    Repeat,
    RepeatMirrored,
}

#[derive(Debug, PartialEq)]
pub enum SamplerFilterMode {
    Nearest,
    Linear,
}

#[derive(Debug, PartialEq)]
pub struct ImageOperands {
    pub bias: Option<OpId>,
    pub lod: Option<OpId>,
    pub grad: Option<(OpId, OpId)>,
    pub const_offset: Option<OpId>,
    pub offset: Option<OpId>,
    pub const_offsets: Option<OpId>,
    pub min_lod: Option<OpId>,
}

#[derive(Debug, PartialEq)]
pub struct MemoryAccess {
    volatile: bool,
    aligned: bool,
    non_temporal: bool,
}

#[derive(Debug, Default, PartialEq)]
pub struct FunctionControl {
    pub inline: bool,
    pub dont_inline: bool,
    pub pure_function: bool,
    pub const_function: bool,
}

#[derive(Debug, PartialEq)]
pub struct LoopControl {
    pub unroll: bool,
    pub dont_unroll: bool,
    pub dependency_infinite: bool,
    pub dependency_length: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub struct SelectionControl {
    flatten: bool,
    dont_flatten: bool,
}

#[derive(Debug, PartialEq)]
pub struct ScopeId(pub Word);

#[derive(Debug, PartialEq)]
pub struct MemorySemanticsId(pub Word);

#[derive(Debug, PartialEq)]
pub enum GroupOperation {
    Reduce,
    InclusiveScan,
    ExclusiveScan,
}

#[derive(Debug, PartialEq)]
pub enum KernelEnqueueFlags {
    NoWait,
    WaitKernel,
    WaitWorkGroup,
}

#[derive(Debug, PartialEq)]
pub struct KernelProfilingInfo {
    cmd_exec_time: bool,
}
