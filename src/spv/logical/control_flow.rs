
use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt;

pub use super::super::types::*;
pub use super::super::op::*;
pub use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(u32);

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", OpId(self.0))
    }
}

#[derive(Debug)]
pub enum ControlFlowChain {
    Atom(BlockId),
    Block(Vec<ControlFlowChain>),
    If(BlockId, Box<ControlFlowChain>, SelectionControl, Option<BranchWeights>),
    IfElse(BlockId,
           Box<ControlFlowChain>,
           Box<ControlFlowChain>,
           SelectionControl,
           Option<BranchWeights>),
}

impl fmt::Display for ControlFlowChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}

impl ControlFlowChain {
    /// Get block contents and optimize the structure
    fn block(mut chains: Vec<ControlFlowChain>) -> ControlFlowChain {
        match chains.len() {
            0 => {
                // Empty block will be internal parse state
                // Just return and it will be dealt with later
                ControlFlowChain::Block(chains)
            }
            1 => {
                // If we only have on chain then return that instead
                chains.pop().unwrap()
            }
            _ => {
                // Else merge everything to be simpler
                let mut merged = Vec::with_capacity(chains.len());
                for chain in chains.into_iter() {
                    match chain {
                        ControlFlowChain::Block(mut more) => {
                            // Other blocks get their elements pulled into us
                            // to remove nexted Blocks
                            merged.append(&mut more);
                        }
                        _ => {
                            // Other nodes just get added
                            merged.push(chain);
                        }
                    }
                }
                ControlFlowChain::Block(merged)
            }
        }
    }

    /// Emit If or IfElse depending on the branch
    fn conditional(id: BlockId,
                   true_chain: Box<ControlFlowChain>,
                   false_chain: Box<ControlFlowChain>,
                   selection_control: SelectionControl,
                   weights: Option<BranchWeights>)
                   -> ControlFlowChain {
        match *false_chain {
            ControlFlowChain::Block(ref block) if block.len() == 0 => {
                ControlFlowChain::If(id, true_chain, selection_control, weights)
            }
            _ => ControlFlowChain::IfElse(id, true_chain, false_chain, selection_control, weights),
        }
    }

    fn fmt_indent(&self, f: &mut fmt::Formatter, indent: u32) -> fmt::Result {
        let write_indent = |f: &mut fmt::Formatter| -> fmt::Result {
            for _ in 0..indent {
                try!(f.write_str("    "))
            }
            Ok(())
        };
        match *self {
            ControlFlowChain::Atom(ref id) => {
                try!(write_indent(f));
                writeln!(f, "{}", id)
            }
            ControlFlowChain::Block(ref block) => {
                for cfc in block {
                    try!(cfc.fmt_indent(f, indent))
                }
                Ok(())
            }
            ControlFlowChain::If(ref id, ref left, ref hint, ref weights) => {
                try!(write_indent(f));
                try!(write!(f, "if {} ", id));
                if SelectionControl::default() != *hint {
                    try!(write!(f, " [{}]", hint));
                }
                if let Some(ref weights) = *weights {
                    try!(write!(f, " [{}]", weights));
                }
                try!(writeln!(f, "{{"));
                try!(left.fmt_indent(f, indent + 1));
                try!(write_indent(f));
                writeln!(f, "}}")
            }
            ControlFlowChain::IfElse(ref id, ref left, ref right, ref hint, ref weights) => {
                try!(write_indent(f));
                try!(write!(f, "if {} ", id));
                if SelectionControl::default() != *hint {
                    try!(write!(f, " [{}]", hint));
                }
                if let Some(ref weights) = *weights {
                    try!(write!(f, " [{}]", weights));
                }
                try!(writeln!(f, "{{"));
                try!(left.fmt_indent(f, indent + 1));
                try!(write_indent(f));
                try!(writeln!(f, "}} else {{"));
                try!(right.fmt_indent(f, indent + 1));
                try!(write_indent(f));
                writeln!(f, "}}")
            }
        }
    }
}

#[derive(Debug)]
pub enum ControlType {
    If,
    IfElse,
    While,
}

#[derive(Debug)]
pub enum ControlFlowError {
    DuplicateBlockId(BlockId),
    NoBlocks,
    UnknownBlockId(BlockId),
    CouldNotPredictConverge(ControlType, BlockId),
    InvalidConvergePrediction(ControlType, BlockId),
    Failed,
}

pub type ControlFlowResult<T> = Result<T, ControlFlowError>;

type BlockMap<'a> = HashMap<BlockId, &'a BasicBlock>;

pub fn find_control_flow(decl: FunctionDefinition) -> ControlFlowResult<ControlFlowChain> {

    let mut start_opt = None;

    let block_map: BlockMap = {
        let mut map = HashMap::new();
        for block in &decl.blocks {
            let id = BlockId(block.label.result_id.0);
            if let None = start_opt {
                start_opt = Some(id);
            }
            if let Some(_) = map.insert(id, block) {
                return Err(ControlFlowError::DuplicateBlockId(id));
            }
        }
        map
    };

    let start = match start_opt {
        Some(id) => id,
        None => return Err(ControlFlowError::NoBlocks),
    };

    let (chain, next) = try!(search_block(start, &HashSet::new(), &block_map));

    match next {
        Continue::Return => Ok(chain),
        _ => Err(ControlFlowError::Failed),
    }
}

#[derive(Debug, PartialEq)]
enum Continue {
    Next(BlockId),
    Return,
}

fn search_block(id: BlockId,
                backtrack: &HashSet<BlockId>,
                block_map: &BlockMap)
                -> ControlFlowResult<(ControlFlowChain, Continue)> {

    if backtrack.contains(&id) {
        return Ok((ControlFlowChain::block(vec![]), Continue::Next(id)));
    }

    let block = match block_map.get(&id) {
        Some(block) => block,
        None => return Err(ControlFlowError::UnknownBlockId(id)),
    };

    match block.branch {
        GroupBranch::OpBranch(ref op) => {
            let current = ControlFlowChain::Atom(id);

            let next_id = BlockId(op.target_label.0);
            let (next_chain, next_next) = try!(search_block(next_id, backtrack, block_map));

            let chain = ControlFlowChain::block(vec![current, next_chain]);

            Ok((chain, next_next))
        }
        GroupBranch::OpBranchConditional(ref op) => {

            let (converge, hints) = match block.merge {
                Some(GroupMerge::OpSelectionMerge(ref op)) => {
                    (BlockId(op.merge_block.0), op.selection_control.clone())
                }
                _ => return Err(ControlFlowError::CouldNotPredictConverge(ControlType::IfElse, id)),
            };

            let true_block = BlockId(op.true_label.0);
            let false_block = BlockId(op.false_label.0);

            // Set up inner backtrack set
            let inner_bt = {
                let mut inner = backtrack.clone();
                inner.insert(converge);
                inner
            };

            // Recursive parse branches
            let (true_chain, true_next) = try!(search_block(true_block, &inner_bt, block_map));
            let (false_chain, false_next) = try!(search_block(false_block, &inner_bt, block_map));

            if true_next == false_next && true_next == Continue::Next(converge) {

                let true_box = Box::new(true_chain);
                let false_box = Box::new(false_chain);
                let weights = op.weights.clone();
                let ctc = ControlFlowChain::conditional(id, true_box, false_box, hints, weights);

                let (next_chain, next_next) = try!(search_block(converge, backtrack, block_map));

                let chain = ControlFlowChain::block(vec![ctc, next_chain]);

                Ok((chain, next_next))
            } else {
                Err(ControlFlowError::InvalidConvergePrediction(ControlType::IfElse, id))
            }
        }
        GroupBranch::OpReturn(_) => Ok((ControlFlowChain::Atom(id), Continue::Return)),
    }
}
