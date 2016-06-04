//! Core types used with SPIR-V instructions

use std::fmt;
use std::fmt::{Display, Formatter};
use super::dis::*;

/// A SPIR-V `<id>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OpId(pub u32);

impl Display for OpId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(&format!("%{}", self.0))
    }
}

/// A SPIR-V `Result <id>`
#[derive(Clone, Debug, PartialEq)]
pub struct ResultId(pub u32);

/// Set of words used to represent a literal constant
pub type LitBytes = Vec<u32>;

/// A String literal
pub type LitString = String;

/// Version for a module
#[derive(Clone, Debug, PartialEq)]
pub struct Version(pub u8, pub u8);

/// Struct to hold type and version for the generator of a module
#[derive(Clone, Debug, PartialEq)]
pub struct Generator {
    pub tool: Tool,
    pub version: u16,
}

/// The tool used to generate a module
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub enum SourceLanguage {
    Unknown,
    Essl,
    Glsl,
    OpenCL_C,
    OpenCL_Cpp,
    Other(u32),
}

impl Display for SourceLanguage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            SourceLanguage::Unknown => "Unknown",
            SourceLanguage::Essl => "ESSL",
            SourceLanguage::Glsl => "GLSL",
            SourceLanguage::OpenCL_C => "OpenCL_C",
            SourceLanguage::OpenCL_Cpp => "OpenCL_CPP",
            SourceLanguage::Other(n) => return write!(f, "Unknown({})", n),
        };
        write!(f, "{}", name)
    }
}

/// Version of the source language
#[derive(Clone, Debug, PartialEq)]
pub struct SourceVersion(pub u32);

impl Display for SourceVersion {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Line number used with OpLine instruction
#[derive(Clone, Debug, PartialEq)]
pub struct Line(pub u32);

/// Column number used with OpLine instruction
#[derive(Clone, Debug, PartialEq)]
pub struct Column(pub u32);

/// Type of decoration to annotate an instruction with
#[derive(Clone, Debug, PartialEq)]
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

impl Display for Decoration {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Decoration::RelaxedPrecision => "RelaxedPrecision",
            Decoration::SpecId(ref n) => return write!(f, "SpecId {}", n),
            Decoration::Block => "Block",
            Decoration::BufferBlock => "BufferBlock",
            Decoration::RowMajor => "RowMajor",
            Decoration::ColMajor => "ColMajor",
            Decoration::ArrayStride(ref n) => return write!(f, "ArrayStride {}", n),
            Decoration::MatrixStride(ref n) => return write!(f, "MatrixStride {}", n),
            Decoration::GlslShared => "GLSLShared",
            Decoration::GlslPacked => "GLSLPacked",
            Decoration::CPacked => "CPacked",
            Decoration::BuiltIn(ref b) => return write!(f, "BuiltIn {}", b),
            Decoration::NoPerspective => "NoPerspective",
            Decoration::Flat => "Flat",
            Decoration::Patch => "Patch",
            Decoration::Centroid => "Centroid",
            Decoration::Sample => "Sample",
            Decoration::Invariant => "Invariant",
            Decoration::Restrict => "Restrict",
            Decoration::Aliased => "Aliased",
            Decoration::Volatile => "Volatile",
            Decoration::Constant => "Constant",
            Decoration::Coherent => "Coherent",
            Decoration::NonWritable => "NonWritable",
            Decoration::NonReadable => "NonReadable",
            Decoration::Uniform => "Uniform",
            Decoration::SaturatedConversion => "SaturatedConversion",
            Decoration::Stream(ref n) => return write!(f, "Stream {}", n),
            Decoration::Location(ref n) => return write!(f, "Location {}", n),
            Decoration::Component(ref n) => return write!(f, "Component {}", n),
            Decoration::Index(ref n) => return write!(f, "Index {}", n),
            Decoration::Binding(ref n) => return write!(f, "Binding {}", n),
            Decoration::DescriptorSet(ref n) => return write!(f, "DescriptorSet {}", n),
            Decoration::Offset(ref n) => return write!(f, "Offset {}", n),
            Decoration::XfbBuffer(ref n) => return write!(f, "XfbBuffer {}", n),
            Decoration::XfbStride(ref n) => return write!(f, "XfbStride {}", n),
            Decoration::FuncParamAttr(ref foa) => return write!(f, "FuncParamAttr {}", foa),
            Decoration::FpRoundingMode(ref rounding_mode) => {
                return write!(f, "FpRoundingMode {}", rounding_mode)
            }
            Decoration::FpFastMathMode(ref fast_math_mode) => {
                return write!(f, "FpFastMathMode {}", fast_math_mode)
            }
            Decoration::LinkageAttributes(ref name, ref lt) => {
                return write!(f, "LinkageAttributes {} {}", name, lt)
            }
            Decoration::NoContraction => "NoContraction",
            Decoration::InputAttachmentIndex(ref n) => {
                return write!(f, "InputAttachmentIndex {}", n)
            }
            Decoration::Alignment(ref n) => return write!(f, "Alignment {}", n),
        };
        write!(f, "{}", name)
    }
}

/// Marks a special built in variable or member
#[derive(Clone, Debug, PartialEq)]
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

impl Display for BuiltIn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            BuiltIn::Position => "Position",
            BuiltIn::PointSize => "PointSize",
            BuiltIn::ClipDistance => "ClipDistance",
            BuiltIn::CullDistance => "CullDistance",
            BuiltIn::VertexId => "VertexId",
            BuiltIn::InstanceId => "InstanceId",
            BuiltIn::PrimitiveId => "PrimitiveId",
            BuiltIn::InvocationId => "InvocationId",
            BuiltIn::Layer => "Layer",
            BuiltIn::ViewportIndex => "ViewportIndex",
            BuiltIn::TessLevelOuter => "TessLevelOuter",
            BuiltIn::TessLevelInner => "TessLevelInner",
            BuiltIn::TessCoord => "TessCoord",
            BuiltIn::PatchVerticies => "PatchVerticies",
            BuiltIn::FragCoord => "FragCoord",
            BuiltIn::PointCoord => "PointCoord",
            BuiltIn::FrontFacing => "FrontFacing",
            BuiltIn::SampleId => "SampleId",
            BuiltIn::SamplePosition => "SamplePosition",
            BuiltIn::SampleMask => "SampleMask",
            BuiltIn::FragDepth => "FragDepth",
            BuiltIn::HelperInvocation => "HelperInvocation",
            BuiltIn::NumWorkgroups => "NumWorkgroups",
            BuiltIn::WorkgroupSize => "WorkgroupSize",
            BuiltIn::WorkgroupId => "WorkgroupId",
            BuiltIn::LocalInvocationId => "LocalInvocationId",
            BuiltIn::GlobalInvocationId => "GlobalInvocationId",
            BuiltIn::LocalInvocationIndex => "LocalInvocationIndex",
            BuiltIn::WorkDim => "WorkDim",
            BuiltIn::GlobalSize => "GlobalSize",
            BuiltIn::EnqueuedWorkgroupSize => "EnqueuedWorkgroupSize",
            BuiltIn::GlobalOffset => "GlobalOffset",
            BuiltIn::GlobalLinearId => "GlobalLinearId",
            BuiltIn::SubgroupSize => "SubgroupSize",
            BuiltIn::SubgroupMaxSize => "SubgroupMaxSize",
            BuiltIn::NumSubgroups => "NumSubgroups",
            BuiltIn::NumEnqueuedSubgroups => "NumEnqueuedSubgroups",
            BuiltIn::SubgroupId => "SubgroupId",
            BuiltIn::SubgroupLocalInvocationId => "SubgroupLocalInvocationId",
            BuiltIn::VertexIndex => "VertexIndex",
            BuiltIn::InstanceIndex => "InstanceIndex",
        };
        write!(f, "{}", name)
    }
}

/// Offset of a member in a type
#[derive(Clone, Debug, PartialEq)]
pub struct MemberIndex(pub u32);

/// The addressing model used by the module
#[derive(Clone, Debug, PartialEq)]
pub enum AddressingModel {
    Logical,
    Physical32,
    Physical64,
}

impl Display for AddressingModel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            AddressingModel::Logical => "Logical",
            AddressingModel::Physical32 => "Physical32",
            AddressingModel::Physical64 => "Physical64",
        };
        write!(f, "{}", name)
    }
}

/// The memory model required by the module
#[derive(Clone, Debug, PartialEq)]
pub enum MemoryModel {
    Simple,
    Glsl450,
    OpenCL,
}

impl Display for MemoryModel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            MemoryModel::Simple => "Simple",
            MemoryModel::Glsl450 => "GLSL450",
            MemoryModel::OpenCL => "OpenCL",
        };
        write!(f, "{}", name)
    }
}

/// The execution model for an entry point into the module
#[derive(Clone, Debug, PartialEq)]
pub enum ExecutionModel {
    Vertex,
    TesselationControl,
    TesselationEvaluation,
    Geometry,
    Fragment,
    GlCompute,
    Kernel,
}

impl Display for ExecutionModel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            ExecutionModel::Vertex => "Vertex",
            ExecutionModel::TesselationControl => "TesselationControl",
            ExecutionModel::TesselationEvaluation => "TesselationEvaluation",
            ExecutionModel::Geometry => "Geometry",
            ExecutionModel::Fragment => "Fragment",
            ExecutionModel::GlCompute => "GLCompute",
            ExecutionModel::Kernel => "Kernel",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, PartialEq)]
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

impl Display for ExecutionMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            ExecutionMode::Invocations(ref n) => return write!(f, "Invocations {}", n),
            ExecutionMode::SpacingEqual => "SpacingEqual",
            ExecutionMode::SpacingFractionalEven => "SpacingFractionalEven",
            ExecutionMode::SpacingFractionalOdd => "SpacingFractionalOdd",
            ExecutionMode::VertexOrderCw => "VertexOrderCw",
            ExecutionMode::VertexOrderCcw => "VertexOrderCcw",
            ExecutionMode::PixelCenterInteger => "PixelCenterInteger",
            ExecutionMode::OriginUpperLeft => "OriginUpperLeft",
            ExecutionMode::OriginLowerLeft => "OriginLowerLeft",
            ExecutionMode::EarlyFragmentTests => "EarlyFragmentTests",
            ExecutionMode::PointMode => "PointMode",
            ExecutionMode::Xfb => "Xfb",
            ExecutionMode::DepthReplacing => "DepthReplacing",
            ExecutionMode::DepthGreater => "DepthGreater",
            ExecutionMode::DepthLess => "DepthLess",
            ExecutionMode::DepthUnchanged => "DepthUnchanged",
            ExecutionMode::LocalSize(ref x, ref y, ref z) => {
                return write!(f, "LocalSize {} {} {}", x, y, z)
            }
            ExecutionMode::LocalSizeHint(ref x, ref y, ref z) => {
                return write!(f, "LocalSizeHint {} {} {}", x, y, z)
            }
            ExecutionMode::InputPoints => "InputPoints",
            ExecutionMode::InputLines => "InputLines",
            ExecutionMode::InputLinesAdjacency => "InputLinesAdjacency",
            ExecutionMode::Triangles => "Triangles",
            ExecutionMode::InputTrianglesAdjacency => "InputTrianglesAdjacency",
            ExecutionMode::Quads => "Quads",
            ExecutionMode::Isolines => "Isolines",
            ExecutionMode::OutputVerticies(ref n) => return write!(f, "OutputVerticies {}", n),
            ExecutionMode::OutputPoints => "OutputPoints",
            ExecutionMode::OutputLineStrip => "OutputLineStrip",
            ExecutionMode::OutputTriangleStrip => "OutputTriangleStrip",
            ExecutionMode::VecTypeHint(ref id) => return write!(f, "VecTypeHint {}", id),
            ExecutionMode::ContractionOff => "ContractionOff",
        };
        write!(f, "{}", name)
    }
}

/// Capability that a module may require
///
/// Many instructions and variants depend on a certain capability
#[derive(Clone, Debug, PartialEq)]
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

impl Display for Capability {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Capability::Matrix => "Matrix",
            Capability::Shader => "Shader",
            Capability::Geometry => "Geometry",
            Capability::Tessellation => "Tessellation",
            Capability::Addresses => "Addresses",
            Capability::Linkage => "Linkage",
            Capability::Kernel => "Kernel",
            Capability::Vector16 => "Vector16",
            Capability::Float16Buffer => "Float16Buffer",
            Capability::Float16 => "Float16",
            Capability::Float64 => "Float64",
            Capability::Int64 => "Int64",
            Capability::Int64Atomics => "Int64Atomics",
            Capability::ImageBasic => "ImageBasic",
            Capability::ImageReadWrite => "ImageReadWrite",
            Capability::ImageMipmap => "ImageMipmap",
            Capability::Pipes => "Pipes",
            Capability::Groups => "Groups",
            Capability::DeviceEnqueue => "DeviceEnqueue",
            Capability::LiteralSampler => "LiteralSampler",
            Capability::AtomicStorage => "AtomicStorage",
            Capability::Int16 => "Int16",
            Capability::TessellationPointSize => "TessellationPointSize",
            Capability::GeometryPointSize => "GeometryPointSize",
            Capability::ImageGatherExtended => "ImageGatherExtended",
            Capability::StorageImageMultisample => "StorageImageMultisample",
            Capability::UniformBufferArrayDynamicIndexing => "UniformBufferArrayDynamicIndexing",
            Capability::SampledImageArrayDynamicIndexing => "SampledImageArrayDynamicIndexing",
            Capability::StorageBufferArrayDynamicIndexing => "StorageBufferArrayDynamicIndexing",
            Capability::StorageImageArrayDynamicIndexing => "StorageImageArrayDynamicIndexing",
            Capability::ClipDistance => "ClipDistance",
            Capability::CullDistance => "CullDistance",
            Capability::ImageCubeArray => "ImageCubeArray",
            Capability::SampleRateShading => "SampleRateShading",
            Capability::ImageRect => "ImageRect",
            Capability::SampledRect => "SampledRect",
            Capability::GenericPointer => "GenericPointer",
            Capability::Int8 => "Int8",
            Capability::InputAttachment => "InputAttachment",
            Capability::SparseResidency => "SparseResidency",
            Capability::MinLod => "MinLod",
            Capability::Sampled1D => "Sampled1D",
            Capability::Image1D => "Image1D",
            Capability::SampledCubeArray => "SampledCubeArray",
            Capability::SampledBuffer => "SampledBuffer",
            Capability::ImageBuffer => "ImageBuffer",
            Capability::ImageMSArray => "ImageMSArray",
            Capability::StorageImageExtendedFormats => "StorageImageExtendedFormats",
            Capability::ImageQuery => "ImageQuery",
            Capability::DerivativeControl => "DerivativeControl",
            Capability::InterpolationFunction => "InterpolationFunction",
            Capability::TransformFeedback => "TransformFeedback",
            Capability::GeometryStreams => "GeometryStreams",
            Capability::StorageImageReadWithoutFormat => "StorageImageReadWithoutFormat",
            Capability::StorageImageWriteWithoutFormat => "StorageImageWriteWithoutFormat",
            Capability::MultiViewport => "MultiViewport",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Signedness {
    UnsignedOrNone,
    Signed,
}

impl Display for Signedness {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Signedness::UnsignedOrNone => "0",
            Signedness::Signed => "1",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StorageClass {
    UniformConstant,
    Input,
    Uniform,
    Output,
    Workgroup,
    CrossWorkgroup,
    Private,
    Function,
    Generic,
    PushConstant,
    AtomicCounter,
    Image,
}

impl Display for StorageClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            StorageClass::UniformConstant => "UniformConstant",
            StorageClass::Input => "Input",
            StorageClass::Uniform => "Uniform",
            StorageClass::Output => "Output",
            StorageClass::Workgroup => "Workgroup",
            StorageClass::CrossWorkgroup => "CrossWorkgroup",
            StorageClass::Private => "Private",
            StorageClass::Function => "Function",
            StorageClass::Generic => "Generic",
            StorageClass::PushConstant => "PushConstant",
            StorageClass::AtomicCounter => "AtomicCounter",
            StorageClass::Image => "Image",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FpFastMathMode {
    pub not_nan: bool,
    pub not_inf: bool,
    pub nsz: bool,
    pub allow_recip: bool,
    pub fast: bool,
}

impl Display for FpFastMathMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut parts = Vec::new();
        if self.not_nan {
            parts.push("NotNaN");
        }
        if self.not_inf {
            parts.push("NotInf");
        }
        if self.nsz {
            parts.push("NSZ");
        }
        if self.allow_recip {
            parts.push("AllowRecip");
        }
        if self.fast {
            parts.push("Fast");
        }
        if parts.len() == 0 {
            write!(f, "None")
        } else {
            write!(f, "{}", parts.join(" | "))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FpRoundingMode {
    Rte,
    Rtz,
    Rtp,
    Rtn,
}

impl Display for FpRoundingMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            FpRoundingMode::Rte => "RTE",
            FpRoundingMode::Rtz => "RTZ",
            FpRoundingMode::Rtp => "RTP",
            FpRoundingMode::Rtn => "RTN",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LinkageType {
    Export,
    Import,
}

impl Display for LinkageType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            LinkageType::Export => "Export",
            LinkageType::Import => "Import",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, PartialEq)]
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

impl Display for FunctionParameterAttribute {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            FunctionParameterAttribute::Zext => "Zext",
            FunctionParameterAttribute::Sext => "Sext",
            FunctionParameterAttribute::ByVal => "ByVal",
            FunctionParameterAttribute::Sret => "Sret",
            FunctionParameterAttribute::NoAlias => "NoAlias",
            FunctionParameterAttribute::NoCapture => "NoCapture",
            FunctionParameterAttribute::NoWrite => "NoWrite",
            FunctionParameterAttribute::NoReadWrite => "NoReadWrite",
        };
        write!(f, "{}", name)
    }
}

/// The dimension for an image type
#[derive(Clone, Debug, PartialEq)]
pub enum Dim {
    Tex1D,
    Tex2D,
    Tex3D,
    Cube,
    Rect,
    Buffer,
    SubpassData,
}

impl Display for Dim {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Dim::Tex1D => "1D",
            Dim::Tex2D => "2D",
            Dim::Tex3D => "3D",
            Dim::Cube => "Cube",
            Dim::Rect => "Rect",
            Dim::Buffer => "Buffer",
            Dim::SubpassData => "SubpassData",
        };
        write!(f, "{}", name)
    }
}

/// Indicates if it is known if an image is a depth image
#[derive(Clone, Debug, PartialEq)]
pub enum DepthStatus {
    NotDepth,
    Depth,
    NoIndication,
}

impl Display for DepthStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            DepthStatus::NotDepth => 0,
            DepthStatus::Depth => 1,
            DepthStatus::NoIndication => 2,
        };
        write!(f, "{}", name)
    }
}

/// Indicates if an image is an array or not
#[derive(Clone, Debug, PartialEq)]
pub enum Arrayed {
    False,
    True,
}

impl Display for Arrayed {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Arrayed::False => 0,
            Arrayed::True => 1,
        };
        write!(f, "{}", name)
    }
}

/// The multisample state of an image
#[derive(Clone, Debug, PartialEq)]
pub enum MS {
    Single,
    Multi,
}

impl Display for MS {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            MS::Single => 0,
            MS::Multi => 1,
        };
        write!(f, "{}", name)
    }
}

/// Indicates how an image is used with samplers
#[derive(Clone, Debug, PartialEq)]
pub enum SampledStatus {
    RuntimeChoice,
    WithSampler,
    WithoutSampler,
}

impl Display for SampledStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            SampledStatus::RuntimeChoice => 0,
            SampledStatus::WithSampler => 1,
            SampledStatus::WithoutSampler => 2,
        };
        write!(f, "{}", name)
    }
}

/// The format for an image type
#[derive(Clone, Debug, PartialEq)]
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
    R32ui,
    Rgb10a2ui,
    Rg32ui,
    Rg16ui,
    Rg8ui,
    R16ui,
    R8ui,
}

impl Display for ImageFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            ImageFormat::Unknown => "Unknown",
            ImageFormat::Rgba32f => "Rgba32f",
            ImageFormat::Rgba16f => "Rgba16f",
            ImageFormat::R32f => "R32f",
            ImageFormat::Rgba8 => "Rgba8",
            ImageFormat::Rgba8Snorm => "Rgba8Snorm",
            ImageFormat::Rg32f => "Rg32f",
            ImageFormat::Rg16f => "Rg16f",
            ImageFormat::R11fG11fB10f => "R11fG11fB10f",
            ImageFormat::R16f => "R16f",
            ImageFormat::Rgba16 => "Rgba16",
            ImageFormat::Rgb10A2 => "Rgb10A2",
            ImageFormat::Rg16 => "Rg16",
            ImageFormat::Rg8 => "Rg8",
            ImageFormat::R16 => "R16",
            ImageFormat::R8 => "R8",
            ImageFormat::Rgba16Snorm => "Rgba16Snorm",
            ImageFormat::Rg16Snorm => "Rg16Snorm",
            ImageFormat::Rg8Snorm => "Rg8Snorm",
            ImageFormat::R16Snorm => "R16Snorm",
            ImageFormat::R8Snorm => "R8Snorm",
            ImageFormat::Rgba32i => "Rgba32i",
            ImageFormat::Rgba16i => "Rgba16i",
            ImageFormat::Rgba8i => "Rgba8i",
            ImageFormat::R32i => "R32i",
            ImageFormat::Rg32i => "Rg32i",
            ImageFormat::Rg16i => "Rg16i",
            ImageFormat::Rg8i => "Rg8i",
            ImageFormat::R16i => "R16i",
            ImageFormat::R8i => "R8i",
            ImageFormat::Rgba32ui => "Rgba32ui",
            ImageFormat::Rgba16ui => "Rgba16ui",
            ImageFormat::Rgba8ui => "Rgba8ui",
            ImageFormat::R32ui => "R32ui",
            ImageFormat::Rgb10a2ui => "Rgb10a2ui",
            ImageFormat::Rg32ui => "Rg32ui",
            ImageFormat::Rg16ui => "Rg16ui",
            ImageFormat::Rg8ui => "Rg8ui",
            ImageFormat::R16ui => "R16ui",
            ImageFormat::R8ui => "R8ui",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AccessQualifier {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl Display for AccessQualifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            AccessQualifier::ReadOnly => "ReadOnly",
            AccessQualifier::WriteOnly => "WriteOnly",
            AccessQualifier::ReadWrite => "ReadWrite",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SamplerAddressingMode {
    None,
    ClampEdge,
    Clamp,
    Repeat,
    RepeatMirrored,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SamplerParam {
    NonNormalized,
    Normalized,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SamplerFilterMode {
    Nearest,
    Linear,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImageOperands {
    pub bias: Option<OpId>,
    pub lod: Option<OpId>,
    pub grad: Option<(OpId, OpId)>,
    pub const_offset: Option<OpId>,
    pub offset: Option<OpId>,
    pub const_offsets: Option<OpId>,
    pub min_lod: Option<OpId>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MemoryAccess {
    pub volatile: bool,
    pub aligned: bool,
    pub non_temporal: bool,
}

impl Display for MemoryAccess {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut parts = Vec::new();
        if self.volatile {
            parts.push("Volatile");
        }
        if self.aligned {
            parts.push("Aligned");
        }
        if self.non_temporal {
            parts.push("Nontemporal");
        }
        if parts.len() == 0 {
            write!(f, "None")
        } else {
            write!(f, "{}", parts.join(" | "))
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FunctionControl {
    pub inline: bool,
    pub dont_inline: bool,
    pub pure_function: bool,
    pub const_function: bool,
}

impl Display for FunctionControl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut parts = Vec::new();
        if self.inline {
            parts.push("Inline");
        }
        if self.dont_inline {
            parts.push("DontInline");
        }
        if self.pure_function {
            parts.push("Pure");
        }
        if self.const_function {
            parts.push("Const");
        }
        if parts.len() == 0 {
            write!(f, "None")
        } else {
            write!(f, "{}", parts.join(" | "))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoopControl {
    pub unroll: bool,
    pub dont_unroll: bool,
    /// Added in 1.1
    pub dependency_infinite: bool,
    /// Added in 1.1
    pub dependency_length: Option<u32>,
}

impl Display for LoopControl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut parts = Vec::new();
        if self.unroll {
            parts.push("Unroll".to_string());
        }
        if self.dont_unroll {
            parts.push("DontUnroll".to_string());
        }
        if self.dependency_infinite {
            parts.push("DependencyInfinite".to_string());
        }
        if let Some(len) = self.dependency_length {
            parts.push(format!("DependencyLength({})", len));
        }
        if parts.len() == 0 {
            write!(f, "None")
        } else {
            write!(f, "{}", parts.join(" | "))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectionControl {
    pub flatten: bool,
    pub dont_flatten: bool,
}

impl Display for SelectionControl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut parts = Vec::new();
        if self.flatten {
            parts.push("Flatten");
        }
        if self.dont_flatten {
            parts.push("DontFlatten");
        }
        if parts.len() == 0 {
            write!(f, "None")
        } else {
            write!(f, "{}", parts.join(" | "))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhiArg {
    pub variable: OpId,
    pub parent: OpId,
}

impl Display for PhiArg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", Arg(&self.variable), Arg(&self.parent))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchWeights {
    pub true_weight: u32,
    pub false_weight: u32,
}

impl Display for BranchWeights {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", Arg(&self.true_weight), Arg(&self.false_weight))
    }
}

/// An `<id>` that refers to a scope
#[derive(Clone, Debug, PartialEq)]
pub struct ScopeId(pub u32);

/// An `<id>` that refers to memory semantics
#[derive(Clone, Debug, PartialEq)]
pub struct MemorySemanticsId(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub enum GroupOperation {
    Reduce,
    InclusiveScan,
    ExclusiveScan,
}

#[derive(Clone, Debug, PartialEq)]
pub enum KernelEnqueueFlags {
    NoWait,
    WaitKernel,
    WaitWorkGroup,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KernelProfilingInfo {
    cmd_exec_time: bool,
}
