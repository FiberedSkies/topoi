use std::collections::HashMap;

use crate::categories::*;
pub struct Topos<O: Object, Omega: Object> {
    base_category: Category<O>,
    terminal: usize,
    truths: Omega,
    characteristics: HashMap<usize, Box<dyn Morphism<Domain = O, Codomain = Omega>>>,
}

impl<O: Object, Omega: Object> Topos<O, Omega>{
    pub fn from_subobject_classifier(base: Category<O>, truths: Omega, characteristics: HashMap<usize, Box<dyn Morphism<Domain = O, Codomain = Omega>>>) -> Result<Self, String> {
        let terminal = base.terminal()?;
        if characteristics.len() < base.fetch_subobjects().unwrap().len() {
            return Err("Incomplete classifier".to_string());
        }
        Ok(Topos {
            base_category: base,
            terminal,
            truths,
            characteristics
        })
    }
}