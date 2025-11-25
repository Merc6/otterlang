use crate::runtime::jit::engine::JitEngine;
use crate::runtime::symbol_registry::SymbolRegistry;
use anyhow::Result;
use ast::nodes::Program;

/// Simplified JIT executor for running programs
pub struct JitExecutor {
    engine: JitEngine,
}

impl JitExecutor {
    /// Create a new JIT executor with default LLVM backend
    pub fn new(
        program: &Program,
        symbol_registry: &'static SymbolRegistry,
    ) -> anyhow::Result<Self> {
        Self::new_with_backend(program, symbol_registry)
    }

    /// Create a new JIT executor with specified backend
    pub fn new_with_backend(
        program: &Program,
        symbol_registry: &'static SymbolRegistry,
    ) -> anyhow::Result<Self> {
        let mut engine = JitEngine::new_with_backend(symbol_registry)?;
        engine.compile_program(program)?;

        Ok(Self { engine })
    }

    /// Execute the main function
    pub fn execute_main(&mut self) -> Result<()> {
        // Execute main function
        self.engine.execute_function("main", &[])?;
        Ok(())
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> ExecutorStats {
        ExecutorStats {
            profiler_metrics: self.engine.get_profiler_stats(),
            cache_stats: self.engine.get_cache_stats(),
        }
    }
}

#[derive(Debug)]
pub struct ExecutorStats {
    pub profiler_metrics: Vec<super::profiler::FunctionMetrics>,
    pub cache_stats: super::cache::function_cache::CacheStats,
}
