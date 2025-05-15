use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolutionParseError {
    #[error("Invalid syntax for resolution detected. The expected syntax is <width>x<height>.\nThe angled bracket are not expected, it's there for showcase only.")]
    InvalidSyntax,
    #[error("{}", .0)]
    FailedToParseNumber(#[from] std::num::ParseIntError),
}
