use std::fmt;

pub trait StateTrait:
    Clone + fmt::Debug + Send + for<'a> serde::Deserialize<'a> + serde::Serialize
{
    fn ascii_representation(&self) -> String;
}
