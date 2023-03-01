use crate::application::errors::Error;

pub trait Feature <Input, Output> {
    fn execute(&self, input: Option<Input>) -> Result<Output, Error>;
}