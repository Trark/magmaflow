
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use spv::op::*;
use spv::types::*;
use spv::raw::*;


impl Display for RawModule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, "; SPIR-V\n"));
        try!(write!(f, "; Version: {}.{}\n", self.version.0, self.version.1));
        let generator_vendor = self.generator.tool.get_vendor();
        let generator_tool = self.generator.tool.get_tool();
        let gen_name = match (generator_vendor, generator_tool) {
            (Some(vendor), Some(tool)) => format!("{} {}", vendor, tool),
            (Some(vendor), None) => format!("{}", vendor),
            (None, Some(tool)) => format!("{}", tool),
            (None, None) => "Unknown".into(),
        };
        try!(write!(f, "; Generator: {}; {}\n", gen_name, self.generator.version));
        try!(write!(f, "; Bound: {}\n", self.bound));
        // Trivially 0 as we only support loading a RawModule with 0 in slot
        // reserved for schema
        try!(write!(f, "; Schema: 0\n"));
        for inst in &self.instructions {
            try!(write!(f, "{}\n", inst));
        }
        Ok(())
    }
}

impl Display for Core {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Core::OpNop(ref op) => Display::fmt(op, f),
            Core::OpSource(ref op) => Display::fmt(op, f),
            Core::OpName(ref op) => Display::fmt(op, f),
            Core::OpMemberName(ref op) => Display::fmt(op, f),
            Core::OpExtension(ref op) => Display::fmt(op, f),
            Core::OpExtInstImport(ref op) => Display::fmt(op, f),
            Core::OpExtInst(ref op) => Display::fmt(op, f),
            Core::OpMemoryModel(ref op) => Display::fmt(op, f),
            Core::OpEntryPoint(ref op) => Display::fmt(op, f),
            Core::OpExecutionMode(ref op) => Display::fmt(op, f),
            Core::OpCapability(ref op) => Display::fmt(op, f),
            Core::OpTypeVoid(ref op) => Display::fmt(op, f),
            Core::OpTypeBool(ref op) => Display::fmt(op, f),
            Core::OpTypeInt(ref op) => Display::fmt(op, f),
            Core::OpTypeFloat(ref op) => Display::fmt(op, f),
            Core::OpTypeVector(ref op) => Display::fmt(op, f),
            Core::OpTypeMatrix(ref op) => Display::fmt(op, f),
            Core::OpTypeImage(ref op) => Display::fmt(op, f),
            Core::OpTypeSampler(ref op) => Display::fmt(op, f),
            Core::OpTypeSampledImage(ref op) => Display::fmt(op, f),
            Core::OpTypeArray(ref op) => Display::fmt(op, f),
            Core::OpTypeRuntimeArray(ref op) => Display::fmt(op, f),
            Core::OpTypeStruct(ref op) => Display::fmt(op, f),
            Core::OpTypeOpaque(ref op) => Display::fmt(op, f),
            Core::OpTypePointer(ref op) => Display::fmt(op, f),
            Core::OpTypeFunction(ref op) => Display::fmt(op, f),
            Core::OpTypeEvent(ref op) => Display::fmt(op, f),
            Core::OpTypeDeviceEvent(ref op) => Display::fmt(op, f),
            Core::OpTypeQueue(ref op) => Display::fmt(op, f),
            Core::OpTypePipe(ref op) => Display::fmt(op, f),
            Core::OpTypeForwardPointer(ref op) => Display::fmt(op, f),
            Core::OpConstant(ref op) => Display::fmt(op, f),
            Core::OpConstantComposite(ref op) => Display::fmt(op, f),
            Core::OpFunction(ref op) => Display::fmt(op, f),
            Core::OpFunctionParameter(ref op) => Display::fmt(op, f),
            Core::OpFunctionEnd(ref op) => Display::fmt(op, f),
            Core::OpVariable(ref op) => Display::fmt(op, f),
            Core::OpLoad(ref op) => Display::fmt(op, f),
            Core::OpStore(ref op) => Display::fmt(op, f),
            Core::OpAccessChain(ref op) => Display::fmt(op, f),
            Core::OpDecorate(ref op) => Display::fmt(op, f),
            Core::OpMemberDecorate(ref op) => Display::fmt(op, f),
            Core::OpConvertUToF(ref op) => Display::fmt(op, f),
            Core::OpIMul(ref op) => Display::fmt(op, f),
            Core::OpUMod(ref op) => Display::fmt(op, f),
            Core::OpIEqual(ref op) => Display::fmt(op, f),
            Core::OpPhi(ref op) => Display::fmt(op, f),
            Core::OpLoopMerge(ref op) => Display::fmt(op, f),
            Core::OpSelectionMerge(ref op) => Display::fmt(op, f),
            Core::OpLabel(ref op) => Display::fmt(op, f),
            Core::OpBranch(ref op) => Display::fmt(op, f),
            Core::OpBranchConditional(ref op) => Display::fmt(op, f),
            Core::OpReturn(ref op) => Display::fmt(op, f),
        }
    }
}

struct Arg<'a, T>(&'a T)
    where T: 'a,
          T: Display;

impl<'a, T> Display for Arg<'a, T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, " "));
        try!(write!(f, "{}", self.0));
        Ok(())
    }
}

struct ArgOpt<'a, T>(&'a Option<T>)
    where T: 'a,
          T: Display;

impl<'a, T> Display for ArgOpt<'a, T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self.0 {
            Some(ref t) => {
                try!(write!(f, "{}", Arg(t)));
            }
            None => {}
        }
        Ok(())
    }
}

struct ArgList<'a, T>(&'a Vec<T>)
    where T: 'a,
          T: Display;

impl<'a, T> Display for ArgList<'a, T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for id in self.0 {
            try!(write!(f, "{}", Arg(id)));
        }
        Ok(())
    }
}

struct ArgString<'a>(&'a str);

impl<'a> Display for ArgString<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, " \"{}\"", self.0)
    }
}

impl Display for OpId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(&format!("%{}", self.0))
    }
}

struct Result<'a>(&'a ResultId);

impl<'a> Display for Result<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:>12} = ", OpId((self.0).0))
    }
}

struct NoResult;

impl Display for NoResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "               ")

    }
}

impl Display for OpNop {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpNop", NoResult)
    }
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

impl Display for SourceVersion {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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

impl Display for OpName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpName{}{}",
               NoResult,
               Arg(&self.target),
               ArgString(&self.name))
    }
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

impl Display for OpExtension {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpExtension{}", NoResult, ArgString(&self.name))
    }
}

impl Display for OpExtInstImport {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpExtInstImport{}",
               Result(&self.result_id),
               ArgString(&self.name))
    }
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

impl Display for OpMemoryModel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpMemoryModel{}{}",
               NoResult,
               Arg(&self.addressing_model),
               Arg(&self.memory_model))
    }
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

impl Display for OpExecutionMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpExecutionMode{}{}",
               NoResult,
               Arg(&self.entry_point),
               Arg(&self.mode))
    }
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

impl Display for OpCapability {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpCapability {}", NoResult, self.capability)
    }
}

impl Display for OpTypeVoid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeVoid", Result(&self.result_id))
    }
}

impl Display for OpTypeBool {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeBool", Result(&self.result_id))
    }
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

impl Display for OpTypeInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeInt{}{}",
               Result(&self.result_id),
               Arg(&self.width),
               Arg(&self.signedness))
    }
}

impl Display for OpTypeFloat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeFloat{}",
               Result(&self.result_id),
               Arg(&self.width))
    }
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



impl Display for OpTypeMatrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeMatrix{}{}",
               Result(&self.result_id),
               Arg(&self.column_type),
               Arg(&self.column_count))
    }
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

impl Display for Arrayed {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Arrayed::False => 0,
            Arrayed::True => 1,
        };
        write!(f, "{}", name)
    }
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

impl Display for OpTypeSampler {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeSampler", Result(&self.result_id))
    }
}

impl Display for OpTypeSampledImage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeSampledImage{}",
               Result(&self.result_id),
               Arg(&self.image_type))
    }
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

impl Display for OpTypeRuntimeArray {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeRuntimeArray{}",
               Result(&self.result_id),
               Arg(&self.element_type))
    }
}

impl Display for OpTypeStruct {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeStruct{}",
               Result(&self.result_id),
               ArgList(&self.member_types))
    }
}

impl Display for OpTypeOpaque {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeOpaque{}",
               Result(&self.result_id),
               ArgString(&self.name))
    }
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

impl Display for OpTypePointer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypePointer{}{}",
               Result(&self.result_id),
               Arg(&self.storage_class),
               Arg(&self.pointed_type))
    }
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

impl Display for OpTypeEvent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeEvent", Result(&self.result_id))
    }
}

impl Display for OpTypeDeviceEvent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeDeviceEvent", Result(&self.result_id))
    }
}

impl Display for OpTypeReserveId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeReserveId", Result(&self.result_id))
    }
}

impl Display for OpTypeQueue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypeQueue", Result(&self.result_id))
    }
}

impl Display for OpTypePipe {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpTypePipe", Result(&self.result_id))
    }
}

impl Display for OpTypeForwardPointer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpTypeForwardPointer{}",
               Arg(&self.pointer_type),
               Arg(&self.storage_class))
    }
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

impl Display for OpConstantComposite {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpConstantComposite{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               ArgList(&self.constituents))
    }
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

impl Display for OpFunctionParameter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpFunctionParameter{}",
               Result(&self.result_id),
               Arg(&self.result_type))
    }
}

impl Display for OpFunctionEnd {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpFunctionEnd", NoResult)
    }
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

impl Display for LinkageType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            LinkageType::Export => "Export",
            LinkageType::Import => "Import",
        };
        write!(f, "{}", name)
    }
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

impl Display for OpDecorate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpDecorate{}{}",
               NoResult,
               Arg(&self.target),
               Arg(&self.decoration))
    }
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

impl Display for OpConvertUToF {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpConvertUToF{}{}",
               Result(&self.result_id),
               Arg(&self.result_type),
               Arg(&self.unsigned_value))
    }
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

impl Display for PhiArg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", Arg(&self.variable), Arg(&self.parent))
    }
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

impl Display for OpSelectionMerge {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{}OpSelectionMerge{}{}",
               NoResult,
               Arg(&self.merge_block),
               Arg(&self.selection_control))
    }
}

impl Display for OpLabel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpLabel", Result(&self.result_id))
    }
}

impl Display for BranchWeights {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", Arg(&self.true_weight), Arg(&self.false_weight))
    }
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

impl Display for OpBranch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpBranch{}", NoResult, Arg(&self.target_label))
    }
}

impl Display for OpReturn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}OpReturn", NoResult)
    }
}
