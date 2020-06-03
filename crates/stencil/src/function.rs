use crate::gcthings::GCThing;
use crate::script::{ScriptStencil, ScriptStencilBase};
use ast::source_atom_set::SourceAtomSetIndex;

#[derive(Debug)]
pub struct FunctionFlags {
    flags: u16,
}

// WARNING
// The following section is generated by update_stencil.py.
// Do mot modify manually.
//
// @@@@ BEGIN TYPES @@@@
#[derive(Debug, Clone, Copy)]
pub enum FunctionKind {
    NormalFunction = 0,
    Arrow = 1,
    Method = 2,
    ClassConstructor = 3,
    Getter = 4,
    Setter = 5,
    AsmJS = 6,
    Wasm = 7,
    FunctionKindLimit = 8,
}

#[allow(dead_code)]
const FUNCTION_KIND_SHIFT: u16 = 0;
#[allow(dead_code)]
const FUNCTION_KIND_MASK: u16 = 0x0007;
#[allow(dead_code)]
const EXTENDED: u16 = 1 << 3;
#[allow(dead_code)]
const SELF_HOSTED: u16 = 1 << 4;
#[allow(dead_code)]
const BASESCRIPT: u16 = 1 << 5;
#[allow(dead_code)]
const SELFHOSTLAZY: u16 = 1 << 6;
#[allow(dead_code)]
const CONSTRUCTOR: u16 = 1 << 7;
#[allow(dead_code)]
const BOUND_FUN: u16 = 1 << 8;
#[allow(dead_code)]
const LAMBDA: u16 = 1 << 9;
#[allow(dead_code)]
const WASM_JIT_ENTRY: u16 = 1 << 10;
#[allow(dead_code)]
const HAS_INFERRED_NAME: u16 = 1 << 11;
#[allow(dead_code)]
const ATOM_EXTRA_FLAG: u16 = 1 << 12;
#[allow(dead_code)]
const HAS_GUESSED_ATOM: u16 = ATOM_EXTRA_FLAG;
#[allow(dead_code)]
const HAS_BOUND_FUNCTION_NAME_PREFIX: u16 = ATOM_EXTRA_FLAG;
#[allow(dead_code)]
const RESOLVED_NAME: u16 = 1 << 13;
#[allow(dead_code)]
const RESOLVED_LENGTH: u16 = 1 << 14;
#[allow(dead_code)]
const NEW_SCRIPT_CLEARED: u16 = 1 << 15;
#[allow(dead_code)]
const NORMAL_KIND: u16 = (FunctionKind::NormalFunction as u16) << FUNCTION_KIND_SHIFT;
#[allow(dead_code)]
const ASMJS_KIND: u16 = (FunctionKind::AsmJS as u16) << FUNCTION_KIND_SHIFT;
#[allow(dead_code)]
const WASM_KIND: u16 = (FunctionKind::Wasm as u16) << FUNCTION_KIND_SHIFT;
#[allow(dead_code)]
const ARROW_KIND: u16 = (FunctionKind::Arrow as u16) << FUNCTION_KIND_SHIFT;
#[allow(dead_code)]
const METHOD_KIND: u16 = (FunctionKind::Method as u16) << FUNCTION_KIND_SHIFT;
#[allow(dead_code)]
const CLASSCONSTRUCTOR_KIND: u16 = (FunctionKind::ClassConstructor as u16) << FUNCTION_KIND_SHIFT;
#[allow(dead_code)]
const GETTER_KIND: u16 = (FunctionKind::Getter as u16) << FUNCTION_KIND_SHIFT;
#[allow(dead_code)]
const SETTER_KIND: u16 = (FunctionKind::Setter as u16) << FUNCTION_KIND_SHIFT;
#[allow(dead_code)]
const NATIVE_FUN: u16 = NORMAL_KIND;
#[allow(dead_code)]
const NATIVE_CTOR: u16 = CONSTRUCTOR | NORMAL_KIND;
#[allow(dead_code)]
const ASMJS_CTOR: u16 = CONSTRUCTOR | ASMJS_KIND;
#[allow(dead_code)]
const ASMJS_LAMBDA_CTOR: u16 = CONSTRUCTOR | LAMBDA | ASMJS_KIND;
#[allow(dead_code)]
const WASM: u16 = WASM_KIND;
#[allow(dead_code)]
const INTERPRETED_NORMAL: u16 = BASESCRIPT | CONSTRUCTOR | NORMAL_KIND;
#[allow(dead_code)]
const INTERPRETED_CLASS_CTOR: u16 = BASESCRIPT | CONSTRUCTOR | CLASSCONSTRUCTOR_KIND;
#[allow(dead_code)]
const INTERPRETED_GENERATOR_OR_ASYNC: u16 = BASESCRIPT | NORMAL_KIND;
#[allow(dead_code)]
const INTERPRETED_LAMBDA: u16 = BASESCRIPT | LAMBDA | CONSTRUCTOR | NORMAL_KIND;
#[allow(dead_code)]
const INTERPRETED_LAMBDA_ARROW: u16 = BASESCRIPT | LAMBDA | ARROW_KIND;
#[allow(dead_code)]
const INTERPRETED_LAMBDA_GENERATOR_OR_ASYNC: u16 = BASESCRIPT | LAMBDA | NORMAL_KIND;
#[allow(dead_code)]
const INTERPRETED_GETTER: u16 = BASESCRIPT | GETTER_KIND;
#[allow(dead_code)]
const INTERPRETED_SETTER: u16 = BASESCRIPT | SETTER_KIND;
#[allow(dead_code)]
const INTERPRETED_METHOD: u16 = BASESCRIPT | METHOD_KIND;
#[allow(dead_code)]
const MUTABLE_FLAGS: u16 = RESOLVED_NAME | RESOLVED_LENGTH | NEW_SCRIPT_CLEARED;
#[allow(dead_code)]
const STABLE_ACROSS_CLONES: u16 = CONSTRUCTOR | LAMBDA | SELF_HOSTED | FUNCTION_KIND_MASK;
// @@@@ END TYPES @@@@

#[derive(Debug)]
pub struct FunctionSyntaxKind {
    kind: FunctionKind,
    is_lambda: bool,
    is_generator: bool,
    is_async: bool,
}

impl FunctionSyntaxKind {
    pub fn function_declaration(is_generator: bool, is_async: bool) -> Self {
        Self {
            kind: FunctionKind::NormalFunction,
            is_lambda: false,
            is_generator,
            is_async,
        }
    }

    pub fn function_expression(is_generator: bool, is_async: bool) -> Self {
        Self {
            kind: FunctionKind::NormalFunction,
            is_lambda: true,
            is_generator,
            is_async,
        }
    }

    pub fn method(is_generator: bool, is_async: bool) -> Self {
        Self {
            kind: FunctionKind::Method,
            is_lambda: false,
            is_generator,
            is_async,
        }
    }

    pub fn getter() -> Self {
        FunctionSyntaxKind {
            kind: FunctionKind::Getter,
            is_lambda: false,
            is_generator: false,
            is_async: false,
        }
    }

    pub fn setter() -> Self {
        FunctionSyntaxKind {
            kind: FunctionKind::Setter,
            is_lambda: false,
            is_generator: false,
            is_async: false,
        }
    }

    pub fn arrow(is_async: bool) -> Self {
        Self {
            kind: FunctionKind::Arrow,
            is_lambda: true,
            is_generator: false,
            is_async,
        }
    }

    pub fn is_generator(&self) -> bool {
        self.is_generator
    }

    pub fn is_async(&self) -> bool {
        self.is_async
    }
}

impl FunctionFlags {
    fn new(flags: u16) -> Self {
        debug_assert!(
            (((FunctionKind::FunctionKindLimit as u16) - 1) << FUNCTION_KIND_SHIFT)
                <= FUNCTION_KIND_MASK
        );

        Self { flags }
    }

    pub fn interpreted(syntax_kind: FunctionSyntaxKind) -> Self {
        let kind_flag = (syntax_kind.kind as u16) << FUNCTION_KIND_SHIFT;
        let mut flags = BASESCRIPT | kind_flag;
        match syntax_kind.kind {
            FunctionKind::NormalFunction => {
                if !syntax_kind.is_generator && !syntax_kind.is_async {
                    flags |= CONSTRUCTOR;
                }
                if syntax_kind.is_lambda {
                    flags |= LAMBDA;
                }
            }
            FunctionKind::ClassConstructor => {
                debug_assert!(!syntax_kind.is_generator);
                debug_assert!(!syntax_kind.is_async);
                flags |= CONSTRUCTOR;
            }
            _ => {}
        }
        Self::new(flags)
    }
}

#[derive(Debug)]
pub struct NonLazyFunctionScript {
    script: ScriptStencil,
}

#[derive(Debug)]
pub struct LazyFunctionScript {
    script: ScriptStencilBase,
}

#[derive(Debug)]
pub enum FunctionScript {
    NonLazy(NonLazyFunctionScript),
    Lazy(LazyFunctionScript),
}

#[derive(Debug)]
pub struct SourceExtent {
    pub source_start: u32,
    pub source_end: u32,
    pub to_string_start: u32,
    pub to_string_end: u32,

    pub lineno: u32,
    pub column: u32,
}

/// Maps to JSFunction in m-c/js/src/vm/JSFunction.h, and BaseScript/JSScript
/// in m-c/js/src/vm/JSScript.h, linked from it.
#[derive(Debug)]
pub struct FunctionStencil {
    name: Option<SourceAtomSetIndex>,
    script: FunctionScript,
    flags: FunctionFlags,
    extent: SourceExtent,
    // FIXME: add more fields
}

impl FunctionStencil {
    pub fn non_lazy(
        name: Option<SourceAtomSetIndex>,
        script: ScriptStencil,
        flags: FunctionFlags,
        extent: SourceExtent,
    ) -> Self {
        Self {
            name,
            script: FunctionScript::NonLazy(NonLazyFunctionScript { script }),
            flags,
            extent,
        }
    }

    pub fn lazy(
        name: Option<SourceAtomSetIndex>,
        script: ScriptStencilBase,
        flags: FunctionFlags,
        extent: SourceExtent,
    ) -> Self {
        Self {
            name,
            script: FunctionScript::Lazy(LazyFunctionScript { script }),
            flags,
            extent,
        }
    }

    pub fn is_non_lazy(&self) -> bool {
        match self.script {
            FunctionScript::NonLazy(_) => true,
            FunctionScript::Lazy(_) => false,
        }
    }

    pub fn is_lazy(&self) -> bool {
        match self.script {
            FunctionScript::NonLazy(_) => false,
            FunctionScript::Lazy(_) => true,
        }
    }

    pub fn non_lazy_script<'a>(&'a self) -> &'a ScriptStencil {
        match &self.script {
            FunctionScript::NonLazy(nonlazy) => &nonlazy.script,
            FunctionScript::Lazy(_) => panic!("unexpeceted function type"),
        }
    }

    pub fn lazy_script<'a>(&'a self) -> &'a ScriptStencilBase {
        match &self.script {
            FunctionScript::NonLazy(_) => panic!("unexpeceted function type"),
            FunctionScript::Lazy(lazy) => &lazy.script,
        }
    }

    pub fn lazy_script_mut<'a>(&'a mut self) -> &'a mut ScriptStencilBase {
        match &mut self.script {
            FunctionScript::NonLazy(_) => panic!("unexpeceted function type"),
            FunctionScript::Lazy(lazy) => &mut lazy.script,
        }
    }

    pub fn set_name(&mut self, name: SourceAtomSetIndex) {
        self.name = Some(name);
    }

    pub fn name<'a>(&'a self) -> &'a Option<SourceAtomSetIndex> {
        &self.name
    }

    pub fn set_has_rest(&mut self) {
        self.lazy_script_mut().set_has_rest();
    }

    pub fn set_to_string_starts(&mut self, to_string_start: usize) {
        self.extent.to_string_start = to_string_start as u32;
    }

    pub fn set_to_string_end(&mut self, to_string_end: usize) {
        self.extent.to_string_end = to_string_end as u32;
    }

    pub fn set_source_end(&mut self, source_end: usize) {
        self.extent.source_end = source_end as u32;
    }

    pub fn push_inner_function(&mut self, fun: FunctionStencilIndex) {
        self.lazy_script_mut().gcthings.push(GCThing::Function(fun));
    }

    pub fn push_closed_over_bindings(&mut self, name: SourceAtomSetIndex) {
        self.lazy_script_mut().gcthings.push(GCThing::Atom(name));
    }
}

/// Index into FunctionStencilList.items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionStencilIndex {
    index: usize,
}

impl FunctionStencilIndex {
    fn new(index: usize) -> Self {
        Self { index }
    }
}

impl From<FunctionStencilIndex> for usize {
    fn from(index: FunctionStencilIndex) -> usize {
        index.index
    }
}

/// List of FunctionStencil.
#[derive(Debug)]
pub struct FunctionStencilList {
    items: Vec<FunctionStencil>,
}

impl FunctionStencilList {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, fun_data: FunctionStencil) -> FunctionStencilIndex {
        let index = self.items.len();
        self.items.push(fun_data);
        FunctionStencilIndex::new(index)
    }

    pub fn get<'a>(&'a self, index: FunctionStencilIndex) -> &'a FunctionStencil {
        &self.items[usize::from(index)]
    }

    pub fn get_mut<'a>(&'a mut self, index: FunctionStencilIndex) -> &'a mut FunctionStencil {
        &mut self.items[usize::from(index)]
    }
}

impl From<FunctionStencilList> for Vec<FunctionStencil> {
    fn from(list: FunctionStencilList) -> Vec<FunctionStencil> {
        list.items
    }
}
