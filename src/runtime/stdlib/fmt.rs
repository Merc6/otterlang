use crate::runtime::symbol_registry::{FfiFunction, FfiSignature, FfiType, SymbolRegistry};

#[no_mangle]
pub extern "C" fn otter_std_fmt_println(msg: *const u8) {
    if msg.is_null() {
        println!();
        return;
    }

    unsafe {
        let c_str = std::ffi::CStr::from_ptr(msg as *const i8);
        if let Ok(s) = c_str.to_str() {
            println!("{}", s);
        } else {
            eprintln!("[fmt.println: invalid UTF-8]");
        }
    }
}

#[no_mangle]
pub extern "C" fn otter_std_fmt_print(msg: *const u8) {
    if msg.is_null() {
        return;
    }

    unsafe {
        let c_str = std::ffi::CStr::from_ptr(msg as *const i8);
        if let Ok(s) = c_str.to_str() {
            print!("{}", s);
        } else {
            eprint!("[fmt.print: invalid UTF-8]");
        }
    }
}

#[no_mangle]
pub extern "C" fn otter_std_fmt_eprintln(msg: *const u8) {
    if msg.is_null() {
        eprintln!();
        return;
    }

    unsafe {
        let c_str = std::ffi::CStr::from_ptr(msg as *const i8);
        if let Ok(s) = c_str.to_str() {
            eprintln!("{}", s);
        } else {
            eprintln!("[fmt.eprintln: invalid UTF-8]");
        }
    }
}

fn register_std_fmt_symbols(registry: &SymbolRegistry) {
    registry.register(FfiFunction {
        name: "fmt.println".into(),
        symbol: "otter_std_fmt_println".into(),
        signature: FfiSignature::new(vec![FfiType::Str], FfiType::Unit),
    });

    registry.register(FfiFunction {
        name: "fmt.print".into(),
        symbol: "otter_std_fmt_print".into(),
        signature: FfiSignature::new(vec![FfiType::Str], FfiType::Unit),
    });

    registry.register(FfiFunction {
        name: "fmt.eprintln".into(),
        symbol: "otter_std_fmt_eprintln".into(),
        signature: FfiSignature::new(vec![FfiType::Str], FfiType::Unit),
    });
}

fn register_std_error_symbols(registry: &crate::runtime::symbol_registry::SymbolRegistry) {
    use crate::runtime::symbol_registry::{FfiFunction, FfiSignature, FfiType};

    registry.register(FfiFunction {
        name: "runtime.push_context".into(),
        symbol: "otter_error_push_context".into(),
        signature: FfiSignature::new(vec![], FfiType::Unit),
    });

    registry.register(FfiFunction {
        name: "runtime.pop_context".into(),
        symbol: "otter_error_pop_context".into(),
        signature: FfiSignature::new(vec![], FfiType::Unit),
    });

    registry.register(FfiFunction {
        name: "runtime.raise".into(),
        symbol: "otter_error_raise".into(),
        signature: FfiSignature::new(vec![FfiType::Opaque, FfiType::I64], FfiType::Bool),
    });

    registry.register(FfiFunction {
        name: "runtime.clear".into(),
        symbol: "otter_error_clear".into(),
        signature: FfiSignature::new(vec![], FfiType::Bool),
    });

    registry.register(FfiFunction {
        name: "runtime.has_error".into(),
        symbol: "otter_error_has_error".into(),
        signature: FfiSignature::new(vec![], FfiType::Bool),
    });

    registry.register(FfiFunction {
        name: "runtime.get_message".into(),
        symbol: "otter_error_get_message".into(),
        signature: FfiSignature::new(vec![], FfiType::Str),
    });

    registry.register(FfiFunction {
        name: "runtime.rethrow".into(),
        symbol: "otter_error_rethrow".into(),
        signature: FfiSignature::new(vec![], FfiType::Unit),
    });
}

inventory::submit! {
    crate::runtime::ffi::SymbolProvider {
        register: register_std_fmt_symbols,
    }
}

inventory::submit! {
    crate::runtime::ffi::SymbolProvider {
        register: register_std_error_symbols,
    }
}
