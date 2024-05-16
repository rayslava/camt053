use quick_xml::se::to_string;
use thiserror::Error;

use crate::models::Document;

#[derive(Error, Debug)]
pub enum CamtError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] quick_xml::DeError),
}

pub fn generate_camt053(doc: &Document) -> Result<String, CamtError> {
    to_string(doc).map_err(CamtError::from)
}
