//! Data generated by scope analysis, and consumed only by emitter

use std::collections::HashMap;
use stencil::function::FunctionStencilIndex;

#[derive(Debug)]
pub struct FunctionProperty {
    is_annex_b: bool,
}

impl FunctionProperty {
    pub fn new() -> Self {
        Self { is_annex_b: false }
    }

    pub fn mark_annex_b(&mut self) {
        self.is_annex_b = true;
    }

    pub fn is_annex_b(&self) -> bool {
        self.is_annex_b
    }
}

#[derive(Debug)]
pub struct FunctionDeclarationPropertyMap {
    props: HashMap<FunctionStencilIndex, FunctionProperty>,
}

impl FunctionDeclarationPropertyMap {
    pub fn new() -> Self {
        Self {
            props: HashMap::new(),
        }
    }

    pub fn mark_annex_b(&mut self, index: FunctionStencilIndex) {
        if !self.props.contains_key(&index) {
            self.props.insert(index, FunctionProperty::new());
        }

        self.props
            .get_mut(&index)
            .expect("Should exist")
            .mark_annex_b();
    }

    pub fn is_annex_b(&self, index: FunctionStencilIndex) -> bool {
        if !self.props.contains_key(&index) {
            // Defaults to false.
            return false;
        }

        self.props.get(&index).expect("Should exist").is_annex_b()
    }
}
