use std::fmt;

use abi_stable::std_types::RVec;
use abi_stable::StableAbi;
use ahash::AHashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, StableAbi)]
pub enum FfiType {
    Unit,
    Bool,
    I32,
    I64,
    F64,
    Str,
    Opaque,
    List,
    Map,
    Struct { fields: RVec<FfiType> },
    Tuple(RVec<FfiType>),
}

impl fmt::Display for FfiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiType::Unit => write!(f, "unit"),
            FfiType::Bool => write!(f, "bool"),
            FfiType::I32 => write!(f, "i32"),
            FfiType::I64 => write!(f, "i64"),
            FfiType::F64 => write!(f, "f64"),
            FfiType::Str => write!(f, "str"),
            FfiType::Opaque => write!(f, "opaque"),
            FfiType::List => write!(f, "list"),
            FfiType::Map => write!(f, "map"),
            FfiType::Struct { fields } => {
                write!(f, "struct {{")?;
                for (idx, field) in fields.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field)?;
                }
                write!(f, "}}")
            }
            FfiType::Tuple(fields) => {
                write!(f, "(")?;
                for (idx, field) in fields.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FfiSignature {
    pub params: Vec<FfiType>,
    pub result: FfiType,
}

impl FfiSignature {
    pub fn new(params: Vec<FfiType>, result: FfiType) -> Self {
        Self { params, result }
    }
}

impl fmt::Display for FfiSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self
            .params
            .iter()
            .map(|ty| ty.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "({params}) -> {}", self.result)
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FfiFunction {
    pub name: String,
    pub symbol: String,
    pub signature: FfiSignature,
}

#[derive(Default)]
pub struct SymbolRegistry {
    functions: RwLock<AHashMap<String, FfiFunction>>,
}

pub static GLOBAL_SYMBOL_REGISTRY: Lazy<SymbolRegistry> = Lazy::new(SymbolRegistry::default);

impl SymbolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn global() -> &'static SymbolRegistry {
        &GLOBAL_SYMBOL_REGISTRY
    }

    pub fn register(&self, function: FfiFunction) {
        self.functions
            .write()
            .insert(function.name.clone(), function);
    }

    pub fn register_many<I>(&self, functions: I)
    where
        I: IntoIterator<Item = FfiFunction>,
    {
        let mut guard = self.functions.write();
        for function in functions {
            guard.insert(function.name.clone(), function);
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.functions.read().contains_key(name)
    }

    pub fn resolve(&self, name: &str) -> Option<FfiFunction> {
        self.functions.read().get(name).cloned()
    }

    pub fn all(&self) -> Vec<FfiFunction> {
        self.functions.read().values().cloned().collect()
    }
}
