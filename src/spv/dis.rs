
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use spv::types::*;

/// Helper for printing the result id
pub struct Result<'a>(pub &'a ResultId);

impl<'a> Display for Result<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:>12} = ", OpId((self.0).0))
    }
}

/// Helper for printing the space to align with instructions that return a result
pub struct NoResult;

impl Display for NoResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "               ")

    }
}

/// Formats an argument for display as an argument to an intruction. This
/// / includes padding at the start and between elements if there are multiple.
pub trait FormatArg {
    fn format_arg(&self, f: &mut Formatter) -> fmt::Result;
}

impl<T> FormatArg for T
    where T: DisplayArg
{
    fn format_arg(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, " "));
        <T as DisplayArg>::display_arg(self, f)
    }
}

impl<T> FormatArg for Option<T>
    where T: DisplayArg
{
    fn format_arg(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Some(ref t) => {
                try!(write!(f, " "));
                <T as DisplayArg>::display_arg(t, f)
            }
            None => Ok(()),
        }
    }
}

impl<T> FormatArg for Vec<T>
    where T: DisplayArgType
{
    fn format_arg(&self, f: &mut Formatter) -> fmt::Result {
        for element in self {
            try!(write!(f, " "));
            try!(<T as DisplayArg>::display_arg(element, f));
        }
        Ok(())
    }
}

/// Formats an argument element for display inside a formatted argument.
pub trait DisplayArg {
    fn display_arg(&self, f: &mut Formatter) -> fmt::Result;
}

impl DisplayArg for String {
    fn display_arg(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

/// Marker trait for Display types to implement DisplayArg for them.
/// This limits arguments that may be printed to explicitly marked types instead
/// for all Display types.
pub trait DisplayArgType: Display {}

impl<T: DisplayArgType> DisplayArg for T {
    fn display_arg(&self, f: &mut Formatter) -> fmt::Result {
        <T as Display>::fmt(self, f)
    }
}

impl DisplayArgType for u32 {}

/// Macro to implement displaying for an instruction
macro_rules! def_op_display {
    ($name: ident; result_id = $($operand_name: ident)|*) => {
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                try!(write!(f,
                    "{}{}",
                    Result(&self.result_id),
                    stringify!($name),
                ));
                $(try!($crate::spv::dis::FormatArg::format_arg(&self.$operand_name, f));)*
                Ok(())
            }
        }
    };
    ($name: ident; $($operand_name: ident)|*) => {
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                try!(write!(f,
                    "{}{}",
                    NoResult,
                    stringify!($name),
                ));
                $(try!($crate::spv::dis::FormatArg::format_arg(&self.$operand_name, f));)*
                Ok(())
            }
        }
    };
}

/// Macro to define printing for standard single operand code instructions
macro_rules! def_op_display_s1 {
    ($name: ident) => {
        def_op_display!($name; result_id = result_type | operand);
    };
}

/// Macro to define printing for standard double operand code instructions
macro_rules! def_op_display_s2 {
    ($name: ident) => {
        def_op_display!($name; result_id = result_type | operand1 | operand2);
    };
}
