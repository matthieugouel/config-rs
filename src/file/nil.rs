use source::Source;
use value::Value;

// Nil source that does nothing for optional files
pub struct Nil {}

impl Source for Nil {
    fn get(&self, _: &str) -> Option<Value> {
        None
    }
}
