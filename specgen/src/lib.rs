
extern crate backtrace;

mod parser;
mod generator;

pub use parser::SpecFieldType as FieldType;
pub use generator::Generator as SerdeBuilder;
