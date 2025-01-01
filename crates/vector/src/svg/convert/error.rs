use lyon::tessellation::TessellationError;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Svg(usvg::Error),
    Tessellation(TessellationError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<usvg::Error> for Error {
    fn from(error: usvg::Error) -> Self {
        Error::Svg(error)
    }
}

impl From<TessellationError> for Error {
    fn from(error: TessellationError) -> Self {
        Error::Tessellation(error)
    }
}
