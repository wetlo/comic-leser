pub trait StringResult<T> {
    fn str_err(self) -> Result<T, String>;
}

impl<T, E: ToString> StringResult<T> for Result<T, E> {
    fn str_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}
