
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use spv::types::*;

/// Helper for printing an argument
pub struct Arg<'a, T>(pub &'a T)
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

/// Helper for printing an optional argument
pub struct ArgOpt<'a, T>(pub &'a Option<T>)
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

/// Helper for printing an argument list
pub struct ArgList<'a, T>(pub &'a Vec<T>)
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

/// Helper for printing an argument that is a string
pub struct ArgString<'a>(pub &'a str);

impl<'a> Display for ArgString<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, " \"{}\"", self.0)
    }
}

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
