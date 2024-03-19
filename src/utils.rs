use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlCollection};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: JsValue);
}

pub struct HtmlCollectionIterator<'a> {
    collection: &'a HtmlCollection,
    index: u32,
}

impl Iterator for HtmlCollectionIterator<'_> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.collection.length() {
            let element = self.collection.item(self.index).unwrap();
            self.index += 1;
            Some(element)
        } else {
            None
        }
    }
}

impl<'a> From<&'a HtmlCollection> for HtmlCollectionIterator<'a> {
    fn from(value: &'a HtmlCollection) -> Self {
        HtmlCollectionIterator {
            collection: &value,
            index: 0,
        }
    }
}