
use spv::*;
use spv::op::*;
use spv::types::*;
use spv::logical::*;
use super::OpByBlock;

#[derive(Clone, Debug, PartialEq)]
pub enum ValidationError {
    MemoryModelMissing,
    ExpectedOpFunction(usize),
    ExpectedOpFunctionEnd(usize),
    ExpectedBranch(usize),
    UnexpectedInstruction(usize),
}

pub type ValidationResult<T> = Result<T, ValidationError>;

pub fn parse(raw: RawModule) -> ValidationResult<LogicalModule> {
    let group = Into::<OpByBlock>::into;
    let insts_storage = raw.instructions.into_iter().map(group).collect::<Vec<_>>();
    let insts = OpSlice::new(&insts_storage);

    let (capabilities, insts) = try!(read_many(insts, read_capability));
    let (extensions, insts) = try!(read_many(insts, read_extension));
    let (ext_inst_imports, insts) = try!(read_many(insts, read_ext_inst_import));
    let (memory_model, insts) = try!(read_memory_model(insts));
    let (entry_points, insts) = try!(read_many(insts, read_entry_point));
    let (execution_modes, insts) = try!(read_many(insts, read_execution_mode));
    let (debug, insts) = try!(read_many(insts, read_debug));
    let (annotations, insts) = try!(read_many(insts, read_annotation));
    let (globals, insts) = try!(read_many(insts, read_global));
    let (function_declarations, insts) = try!(read_many(insts, read_function_declaration));
    let (function_definitions, insts) = try!(read_many(insts, read_function_definition));

    if insts.get_remaining() > 0 {
        return Err(ValidationError::UnexpectedInstruction(insts.get_slot()));
    }

    Ok(LogicalModule {
        capabilities: capabilities,
        extensions: extensions,
        ext_inst_imports: ext_inst_imports,
        memory_model: memory_model,
        entry_points: entry_points,
        execution_modes: execution_modes,
        debug: debug,
        annotations: annotations,
        globals: globals,
        function_declarations: function_declarations,
        function_definitions: function_definitions,
    })
}

#[derive(Clone, Copy)]
struct OpSlice<'a> {
    insts: &'a [OpByBlock],
    index: usize,
}

impl<'a> OpSlice<'a> {
    fn new(insts: &'a [OpByBlock]) -> OpSlice {
        OpSlice {
            insts: insts,
            index: 0,
        }
    }
    fn first(&self) -> Option<&OpByBlock> {
        if self.index < self.insts.len() {
            Some(&self.insts[self.index])
        } else {
            None
        }
    }
    fn advance(self) -> OpSlice<'a> {
        assert!(self.index < self.insts.len());
        OpSlice {
            insts: self.insts,
            index: self.index + 1,
        }
    }
    fn get_slot(&self) -> usize {
        self.index
    }
    fn get_remaining(&self) -> usize {
        self.insts.len() - self.index
    }
}

enum PhaseResult<'a, T> {
    Ok(T, OpSlice<'a>),
    Next(OpSlice<'a>),
    Err(ValidationError),
}

fn read_many<T>(insts: OpSlice,
                f: fn(OpSlice) -> PhaseResult<T>)
                -> ValidationResult<(Vec<T>, OpSlice)> {
    fn read_rest<T>(insts: OpSlice,
                    f: fn(OpSlice) -> PhaseResult<T>,
                    mut output: Vec<T>)
                    -> ValidationResult<(Vec<T>, OpSlice)> {
        match f(insts) {
            PhaseResult::Ok(item, next) => {
                output.push(item);
                read_rest(next, f, output)
            }
            PhaseResult::Next(next) => Ok((output, next)),
            PhaseResult::Err(err) => Err(err),
        }
    }
    read_rest(insts, f, Vec::new())
}

fn read_capability(insts: OpSlice) -> PhaseResult<Capability> {
    if let Some(&OpByBlock::OpCapability(ref op)) = insts.first() {
        PhaseResult::Ok(op.capability.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_extension(insts: OpSlice) -> PhaseResult<String> {
    if let Some(&OpByBlock::OpExtension(ref op)) = insts.first() {
        PhaseResult::Ok(op.name.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_ext_inst_import(insts: OpSlice) -> PhaseResult<OpExtInstImport> {
    if let Some(&OpByBlock::OpExtInstImport(ref op)) = insts.first() {
        PhaseResult::Ok(op.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_memory_model(insts: OpSlice) -> ValidationResult<(OpMemoryModel, OpSlice)> {
    if let Some(&OpByBlock::OpMemoryModel(ref op)) = insts.first() {
        Ok((op.clone(), insts.advance()))
    } else {
        Err(ValidationError::MemoryModelMissing)
    }
}

fn read_entry_point(insts: OpSlice) -> PhaseResult<OpEntryPoint> {
    if let Some(&OpByBlock::OpEntryPoint(ref op)) = insts.first() {
        PhaseResult::Ok(op.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_execution_mode(insts: OpSlice) -> PhaseResult<OpExecutionMode> {
    if let Some(&OpByBlock::OpExecutionMode(ref op)) = insts.first() {
        PhaseResult::Ok(op.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_debug(insts: OpSlice) -> PhaseResult<GroupDebug> {
    if let Some(&OpByBlock::GroupDebug(ref op)) = insts.first() {
        PhaseResult::Ok(op.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_annotation(insts: OpSlice) -> PhaseResult<GroupAnnotation> {
    if let Some(&OpByBlock::GroupAnnotation(ref op)) = insts.first() {
        PhaseResult::Ok(op.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_global(insts: OpSlice) -> PhaseResult<GroupGlobal> {
    if let Some(&OpByBlock::GroupGlobal(ref op)) = insts.first() {
        PhaseResult::Ok(op.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_function_parameter(insts: OpSlice) -> PhaseResult<OpFunctionParameter> {
    if let Some(&OpByBlock::OpFunctionParameter(ref op)) = insts.first() {
        PhaseResult::Ok(op.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_function_declaration(insts: OpSlice) -> PhaseResult<FunctionDeclaration> {
    // Preserve start point so we can recurse if we're actually a function definition
    let start_insts = insts;
    if insts.get_remaining() < 2 {
        return PhaseResult::Next(insts);
    }
    if let Some(&OpByBlock::OpFunction(ref op)) = insts.first() {
        match read_many(insts.advance(), read_function_parameter) {
            Ok((params, insts)) => {
                if let Some(&OpByBlock::OpFunctionEnd(_)) = insts.first() {
                    let decl = FunctionDeclaration {
                        function: op.clone(),
                        parameters: params,
                    };
                    PhaseResult::Ok(decl, insts.advance())
                } else {
                    PhaseResult::Next(start_insts)
                }
            }
            Err(err) => PhaseResult::Err(err),
        }
    } else {
        PhaseResult::Err(ValidationError::ExpectedOpFunction(insts.get_slot()))
    }
}

fn read_code(insts: OpSlice) -> PhaseResult<GroupCode> {
    if let Some(&OpByBlock::GroupCode(ref op)) = insts.first() {
        PhaseResult::Ok(op.clone(), insts.advance())
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_branch(insts: OpSlice) -> ValidationResult<(GroupBranch, OpSlice)> {
    if let Some(&OpByBlock::GroupBranch(ref op)) = insts.first() {
        Ok((op.clone(), insts.advance()))
    } else {
        Err(ValidationError::ExpectedBranch(insts.get_slot()))
    }
}

fn read_basic_block(insts: OpSlice) -> PhaseResult<BasicBlock> {
    if let Some(&OpByBlock::OpLabel(ref op)) = insts.first() {
        let label = op.clone();
        let (code, insts) = match read_many(insts.advance(), read_code) {
            Ok((code, insts)) => (code, insts),
            Err(err) => return PhaseResult::Err(err),
        };
        let (branch, insts) = match read_branch(insts) {
            Ok((branch, insts)) => (branch, insts),
            Err(err) => return PhaseResult::Err(err),
        };
        let block = BasicBlock {
            label: label,
            code: code,
            branch: branch,
        };
        PhaseResult::Ok(block, insts)
    } else {
        PhaseResult::Next(insts)
    }
}

fn read_function_definition(insts: OpSlice) -> PhaseResult<FunctionDefinition> {
    if insts.get_remaining() < 2 {
        return PhaseResult::Next(insts);
    }
    if let Some(&OpByBlock::OpFunction(ref op)) = insts.first() {
        match read_many(insts.advance(), read_function_parameter) {
            Ok((params, insts)) => {
                let (blocks, insts) = match read_many(insts, read_basic_block) {
                    Ok((blocks, insts)) => (blocks, insts),
                    Err(err) => return PhaseResult::Err(err),
                };
                if let Some(&OpByBlock::OpFunctionEnd(_)) = insts.first() {
                    let insts = insts.advance();
                    let def = FunctionDefinition {
                        function: op.clone(),
                        parameters: params,
                        blocks: blocks,
                    };
                    PhaseResult::Ok(def, insts)
                } else {
                    PhaseResult::Err(ValidationError::ExpectedOpFunctionEnd(insts.get_slot()))
                }
            }
            Err(err) => PhaseResult::Err(err),
        }
    } else {
        PhaseResult::Err(ValidationError::ExpectedOpFunction(insts.get_slot()))
    }
}
