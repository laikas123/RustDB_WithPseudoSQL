pub type RusqlError = Box<dyn Error + Send + Sync + 'static>;
pub type RusqlResult<T> = Result<T, RusqlError>;

