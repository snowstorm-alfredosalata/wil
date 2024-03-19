use std::panic;
use data_source::DataSource;
use error::DOMParsingError;
use utils::HtmlCollectionIterator;
use web_sys::{self, Element, HtmlCollection};

mod error;
mod utils;
mod data_source; 

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let data_sources = data_source::data_sources();

    let e_tags = document.get_elements_by_tag_name("link");

    for element in HtmlCollectionIterator::from(&e_tags) {
        match DataSource::try_from(&element) {
            Ok(data_source) => {
                utils::log(format!("[eHTML] Registering data source: {data_source:?}.").into());

                data_sources
                    .lock()
                    .unwrap()
                    .push(data_source);

            },
            Err(DOMParsingError::NoMatch) => {},
            Err(err) => utils::log(err.to_string().into()) 
        }

        
    }
}

