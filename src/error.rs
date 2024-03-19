use thiserror::Error;

#[derive(Error, Debug)]
pub enum DOMParsingError {
    #[error("the element does not match the mapped object.")]
    NoMatch,

    #[error("invalid tag (expected {expected:?}, found {found:?})")]
    InvalidTag {
        expected: String,
        found: String,
    },

    #[error("invalid attribute {attribute:?}, {reason:?})")]
    InvalidAttribute {
        attribute: String,
        reason: String,
    },

    #[error("attribute {0:?} is required!")]
    MissingAttribute (String),

    #[error("unknown data store error")]
    Unknown,
}