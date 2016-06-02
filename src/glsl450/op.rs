
use std::fmt;
use std::fmt::{Display, Formatter};
use spv::Op;
use spv::types::OpId;

#[derive(Clone, Debug, PartialEq)]
pub struct Sin {
    pub x: OpId,
}

impl Op for Sin {
    fn get_name(&self) -> &'static str {
        "Sin"
    }
}

impl Display for Sin {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Sin {}", self.x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cos {
    pub x: OpId,
}

impl Op for Cos {
    fn get_name(&self) -> &'static str {
        "Cos"
    }
}

impl Display for Cos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Cos {}", self.x)
    }
}
