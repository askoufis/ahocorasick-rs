#![deny(clippy::all)]

use aho_corasick::AhoCorasick as _AhoCorasick;

#[macro_use]
extern crate napi_derive;

pub struct AhoCorasick {}

#[napi(js_name = "AhoCorasick")]
pub struct JsAhoCorasick {
  ac: _AhoCorasick,
}

#[napi]
impl JsAhoCorasick {
  #[napi(constructor)]
  pub fn new(patterns: Vec<&str>) -> Self {
    let ac = _AhoCorasick::new(patterns).unwrap();
    JsAhoCorasick { ac }
  }

  #[napi(ts_return_type = "Array<[number, number, number]>")]
  pub fn search(&self, query: String) -> Vec<Vec<u32>> {
    let mut matches = vec![];
    for mat in self.ac.find_iter(&query) {
      matches.push(vec![
        mat.pattern().as_u32(),
        u32::try_from(mat.start()).unwrap(),
        u32::try_from(mat.end()).unwrap(),
      ]);
    }

    matches
  }
}
