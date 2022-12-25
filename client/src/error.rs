#[derive(Debug)]
pub(crate) enum Error {
    IO(std::io::Error),
    FileNotFound(std::path::PathBuf),
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}
