use std::sync::Mutex;
use once_cell::sync::OnceCell;
use web_sys::Element;
use edy::prelude::Map;

use crate::error::DOMParsingError;

const DATA_SOURCE_TAG: &'static str = "link";
const DATA_SOURCE_REL_PREFIX: &'static str = "data/";

const DATA_SOURCE_TYPE_JSON: &'static str = "data/json"; 

pub fn data_sources() -> &'static Mutex<Vec<DataSource>> {
    static INSTANCE: OnceCell<Mutex<Vec<DataSource>>> = OnceCell::new();
    INSTANCE.get_or_init(|| Mutex::default())
}

#[derive(Debug)]
pub enum DataSourceType {
    Json,
}

#[derive(Debug)]
pub struct DataSource {
    data_type: DataSourceType,
    href: String,
    update: u16,
    data: Map,
}

impl TryFrom<&Element> for DataSource {
    type Error = DOMParsingError;

    fn try_from(value: &Element) -> Result<Self, Self::Error> {
        if value.tag_name().to_lowercase() != DATA_SOURCE_TAG {
            Err(DOMParsingError::NoMatch)?
        }

        let rel = value.get_attribute("rel").unwrap_or_default().to_lowercase();
        if !rel.starts_with(DATA_SOURCE_REL_PREFIX) {
            Err(DOMParsingError::NoMatch)?
        }

        let href = value.get_attribute("href").unwrap_or_default().to_lowercase();
        let update: u16 = match value.get_attribute("update") {
            None => 0,
            Some(value) => value
                .parse()
                .map_err(|_| DOMParsingError::InvalidAttribute { attribute: "update".to_string(), reason: "must be an int value!".to_string() })?
        };

        match rel.as_str() {
            DATA_SOURCE_TYPE_JSON => Ok(DataSource { data_type: DataSourceType::Json, href, update, data: Map::default() }),
            _ => Err(DOMParsingError::InvalidAttribute { attribute: "rel".to_string(), reason: "must be `data/json`!".to_string() })?
        }
    }
}

impl DataSource {
    
}