pub trait Feature<Input, Output> {
    fn execute(&self, input: Input) -> Result<Output, String>;
}
