
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

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
    Loop(BlockId, Box<ControlFlowChain>, LoopControl, Option<BranchWeights>),
    Break,
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
                try!(write!(f, "if {}", id));
                if SelectionControl::default() != *hint {
                    try!(write!(f, " [{}]", hint));
                }
                if let Some(ref weights) = *weights {
                    try!(write!(f, " [{}]", weights));
                }
                try!(writeln!(f, " {{"));
                try!(left.fmt_indent(f, indent + 1));
                try!(write_indent(f));
                try!(writeln!(f, "}} else {{"));
                try!(right.fmt_indent(f, indent + 1));
                try!(write_indent(f));
                writeln!(f, "}}")
            }
            ControlFlowChain::Loop(ref id, ref inner, ref hint, ref weights) => {
                try!(write_indent(f));
                try!(write!(f, "loop {}", id));
                if LoopControl::default() != *hint {
                    try!(write!(f, " [{}]", hint));
                }
                if let Some(ref weights) = *weights {
                    try!(write!(f, " [{}]", weights));
                }
                try!(writeln!(f, " {{"));
                try!(inner.fmt_indent(f, indent + 1));
                try!(write_indent(f));
                writeln!(f, "}}")
            }
            ControlFlowChain::Break => {
                try!(write_indent(f));
                writeln!(f, "break;")
            }
        }
    }
}

#[derive(Debug)]
pub enum ControlType {
    If,
    IfElse,
    Loop,
}

#[derive(Debug)]
pub enum ControlFlowError {
    DuplicateBlockId(BlockId),
    NoBlocks,
    UnknownBlockId(BlockId),
    CouldNotPredictConverge(BlockId),
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

    let (chain, next) = try!(search_block(start, &FlowList::new(), &block_map));

    match next {
        Continue::Return => Ok(chain),
        _ => Err(ControlFlowError::Failed),
    }
}

/// State to allow backtracking from end points to continue parsing from a parent block
#[derive(Debug, PartialEq)]
enum Continue {
    Next(BlockId),
    Return,
}

/// Marks how we're meant to converge from the children of a block
#[derive(Clone, Debug, PartialEq)]
enum Converge {
    If(BlockId, SelectionControl),
    Loop(BlockId, BlockId, LoopControl),
}

/// Links back to the current flow control we're inside
struct FlowNode {
    converge: Converge,
    ptr: Option<Rc<FlowNode>>,
}

impl FlowNode {
    fn merges(&self, id: &BlockId) -> bool {
        match self.converge {
            Converge::If(join, _) |
            Converge::Loop(join, _, _) => {
                if join == *id {
                    return true;
                }
            }
        }
        match self.ptr {
            Some(ref node) => node.merges(id),
            None => false,
        }
    }

    fn innermost_loop(&self) -> Option<(BlockId, BlockId)> {
        if let Converge::Loop(ref head, ref merge, _) = self.converge {
            return Some((*head, *merge));
        }
        match self.ptr {
            Some(ref node) => node.innermost_loop(),
            None => None,
        }
    }
}

/// Manages the FlowNode list
struct FlowList {
    head: Option<Rc<FlowNode>>,
}

impl FlowList {
    /// Start a new empty flow list
    fn new() -> FlowList {
        FlowList { head: None }
    }

    /// Add a node to the flow list
    fn extend(&self, converge: Converge) -> FlowList {
        FlowList {
            head: Some(Rc::new(FlowNode {
                converge: converge,
                ptr: self.head.clone(),
            })),
        }
    }

    /// Check if a block is in the flow list
    fn merges(&self, id: &BlockId) -> bool {
        match self.head {
            Some(ref node) => node.merges(id),
            None => false,
        }
    }

    /// Find the merge block for the innermost loop
    fn innermost_loop(&self) -> Option<(BlockId, BlockId)> {
        match self.head {
            Some(ref node) => node.innermost_loop(),
            None => None,
        }
    }
}

fn search_block(id: BlockId,
                flow_list: &FlowList,
                block_map: &BlockMap)
                -> ControlFlowResult<(ControlFlowChain, Continue)> {

    if flow_list.merges(&id) {
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

            match block.merge {
                // Loop construct
                Some(GroupMerge::OpLoopMerge(ref loop_merge)) => {
                    let head = id;
                    let after = BlockId(loop_merge.merge_block.0);
                    let hints = loop_merge.loop_control.clone();
                    let converge = Converge::Loop(head, after, hints.clone());

                    // Set up inner flow list
                    let inner_fl = flow_list.extend(converge.clone());

                    // Recursive parse branches
                    let (inner_chain, inner_next) =
                        try!(search_block(next_id, &inner_fl, block_map));

                    if inner_next == Continue::Next(head) {
                        let inner_box = Box::new(inner_chain);

                        let ctc = ControlFlowChain::Loop(id, inner_box, hints, None);

                        let (after_chain, after_next) =
                            try!(search_block(after, flow_list, block_map));

                        let chain = ControlFlowChain::block(vec![ctc, after_chain]);

                        Ok((chain, after_next))
                    } else {
                        Err(ControlFlowError::InvalidConvergePrediction(ControlType::Loop, id))
                    }
                }
                None => {
                    let (next_chain, next_next) = try!(search_block(next_id, flow_list, block_map));

                    let chain = ControlFlowChain::block(vec![current, next_chain]);

                    Ok((chain, next_next))
                }
                _ => unimplemented!(),
            }
        }
        GroupBranch::OpBranchConditional(ref op) => {

            let true_block = BlockId(op.true_label.0);
            let false_block = BlockId(op.false_label.0);

            match block.merge {
                // Selection construct
                Some(GroupMerge::OpSelectionMerge(ref selection)) => {
                    let merge = BlockId(selection.merge_block.0);
                    let hints = selection.selection_control.clone();
                    let converge = Converge::If(merge, hints.clone());

                    // Set up inner flow list
                    let inner_fl = flow_list.extend(converge.clone());

                    // Recursive parse branches
                    let (true_chain, true_next) =
                        try!(search_block(true_block, &inner_fl, block_map));
                    let (false_chain, false_next) =
                        try!(search_block(false_block, &inner_fl, block_map));

                    if true_next == false_next && true_next == Continue::Next(merge) {

                        let true_box = Box::new(true_chain);
                        let false_box = Box::new(false_chain);
                        let weights = op.weights.clone();
                        let ctc =
                            ControlFlowChain::conditional(id, true_box, false_box, hints, weights);

                        let (next_chain, next_next) =
                            try!(search_block(merge, flow_list, block_map));

                        let chain = ControlFlowChain::block(vec![ctc, next_chain]);

                        Ok((chain, next_next))
                    } else {
                        Err(ControlFlowError::InvalidConvergePrediction(ControlType::IfElse, id))
                    }
                }
                // Loop construct
                Some(GroupMerge::OpLoopMerge(_)) => unimplemented!(),
                None => {
                    let (head, merge) = match flow_list.innermost_loop() {
                        Some(l) => l,
                        None => return Err(ControlFlowError::CouldNotPredictConverge(id)),
                    };
                    if false_block == merge {
                        // Break block

                        let inner = try!(search_block(true_block, &flow_list, block_map));
                        let (inner_chain, inner_next) = inner;

                        if inner_next == Continue::Next(head) {
                            let inner_box = Box::new(inner_chain);
                            let break_box = Box::new(ControlFlowChain::Break);
                            let weights = op.weights.clone();
                            let ctc = ControlFlowChain::conditional(id,
                                                                    inner_box,
                                                                    break_box,
                                                                    SelectionControl::default(),
                                                                    weights);

                            Ok((ctc, inner_next))
                        } else {
                            Err(ControlFlowError::InvalidConvergePrediction(ControlType::Loop, id))
                        }
                    } else {
                        return Err(ControlFlowError::CouldNotPredictConverge(id));
                    }
                }
            }
        }
        GroupBranch::OpReturn(_) => {
            // Return block
            Ok((ControlFlowChain::Atom(id), Continue::Return))
        }
    }
}
