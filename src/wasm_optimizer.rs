// WASM Optimizer - Optimizes generated WebAssembly bytecode
//
// This module implements various optimization passes to reduce WASM binary size
// and improve runtime performance:
//
// 1. Dead Code Elimination (DCE) - Remove unused functions and globals
// 2. Constant Folding - Evaluate constant expressions at compile time
// 3. Function Inlining - Inline small functions to reduce call overhead
// 4. Peephole Optimization - Local instruction pattern improvements

use std::collections::{HashMap, HashSet};

/// Represents optimization statistics
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    pub functions_removed: usize,
    pub constants_folded: usize,
    pub functions_inlined: usize,
    pub instructions_eliminated: usize,
    pub original_size: usize,
    pub optimized_size: usize,
}

impl OptimizationStats {
    pub fn size_reduction_percent(&self) -> f64 {
        if self.original_size == 0 {
            return 0.0;
        }
        ((self.original_size - self.optimized_size) as f64 / self.original_size as f64) * 100.0
    }

    pub fn total_optimizations(&self) -> usize {
        self.functions_removed + self.constants_folded +
        self.functions_inlined + self.instructions_eliminated
    }
}

/// WASM optimizer that applies multiple optimization passes
pub struct WasmOptimizer {
    pub enable_dce: bool,
    pub enable_constant_folding: bool,
    pub enable_inlining: bool,
    pub inline_threshold: usize,  // Max instructions to inline
    pub stats: OptimizationStats,
}

impl WasmOptimizer {
    pub fn new() -> Self {
        Self {
            enable_dce: true,
            enable_constant_folding: true,
            enable_inlining: true,
            inline_threshold: 10,  // Inline functions with <= 10 instructions
            stats: OptimizationStats::default(),
        }
    }

    /// Create optimizer with all optimizations enabled
    pub fn aggressive() -> Self {
        Self {
            enable_dce: true,
            enable_constant_folding: true,
            enable_inlining: true,
            inline_threshold: 20,
            stats: OptimizationStats::default(),
        }
    }

    /// Create optimizer with minimal optimizations
    pub fn minimal() -> Self {
        Self {
            enable_dce: true,
            enable_constant_folding: false,
            enable_inlining: false,
            inline_threshold: 0,
            stats: OptimizationStats::default(),
        }
    }

    /// Optimize a WASM module
    pub fn optimize(&mut self, wasm_bytes: Vec<u8>) -> Vec<u8> {
        self.stats.original_size = wasm_bytes.len();

        // Parse WASM module (simplified - in reality would use wasmparser)
        let mut module = WasmModule::parse(&wasm_bytes);

        // Apply optimization passes
        if self.enable_constant_folding {
            self.constant_folding_pass(&mut module);
        }

        if self.enable_inlining {
            self.inlining_pass(&mut module);
        }

        if self.enable_dce {
            self.dead_code_elimination_pass(&mut module);
        }

        // Encode back to bytes
        let optimized = module.encode();
        self.stats.optimized_size = optimized.len();

        optimized
    }

    /// Dead Code Elimination - Remove unused functions
    fn dead_code_elimination_pass(&mut self, module: &mut WasmModule) {
        // Find all function references
        let mut referenced_functions = HashSet::new();

        // Start with exported functions and start function
        for export in &module.exports {
            if export.kind == ExportKind::Function {
                referenced_functions.insert(export.index);
            }
        }

        if let Some(start) = module.start_function {
            referenced_functions.insert(start);
        }

        // Iteratively find all reachable functions
        let mut changed = true;
        while changed {
            changed = false;
            let current_refs: Vec<_> = referenced_functions.iter().copied().collect();

            for &func_idx in &current_refs {
                if let Some(func) = module.functions.get(&func_idx) {
                    for called_idx in &func.called_functions {
                        if referenced_functions.insert(*called_idx) {
                            changed = true;
                        }
                    }
                }
            }
        }

        // Remove unreferenced functions
        let original_count = module.functions.len();
        module.functions.retain(|idx, _| referenced_functions.contains(idx));

        self.stats.functions_removed = original_count - module.functions.len();
    }

    /// Constant Folding - Evaluate constant expressions at compile time
    fn constant_folding_pass(&mut self, module: &mut WasmModule) {
        for (_idx, func) in module.functions.iter_mut() {
            let mut folded = 0;
            let mut new_instructions = Vec::new();
            let mut const_stack: Vec<i32> = Vec::new();

            for inst in &func.instructions {
                match inst {
                    Instruction::I32Const(val) => {
                        const_stack.push(*val);
                        new_instructions.push(inst.clone());
                    }
                    Instruction::I32Add => {
                        if const_stack.len() >= 2 {
                            let b = const_stack.pop().unwrap();
                            let a = const_stack.pop().unwrap();
                            let result = a.wrapping_add(b);

                            // Remove the two const instructions and add folded result
                            new_instructions.pop(); // Remove second const
                            new_instructions.pop(); // Remove first const
                            new_instructions.push(Instruction::I32Const(result));
                            const_stack.push(result);
                            folded += 1;
                        } else {
                            new_instructions.push(inst.clone());
                            const_stack.clear();
                        }
                    }
                    Instruction::I32Sub => {
                        if const_stack.len() >= 2 {
                            let b = const_stack.pop().unwrap();
                            let a = const_stack.pop().unwrap();
                            let result = a.wrapping_sub(b);

                            new_instructions.pop();
                            new_instructions.pop();
                            new_instructions.push(Instruction::I32Const(result));
                            const_stack.push(result);
                            folded += 1;
                        } else {
                            new_instructions.push(inst.clone());
                            const_stack.clear();
                        }
                    }
                    Instruction::I32Mul => {
                        if const_stack.len() >= 2 {
                            let b = const_stack.pop().unwrap();
                            let a = const_stack.pop().unwrap();
                            let result = a.wrapping_mul(b);

                            new_instructions.pop();
                            new_instructions.pop();
                            new_instructions.push(Instruction::I32Const(result));
                            const_stack.push(result);
                            folded += 1;
                        } else {
                            new_instructions.push(inst.clone());
                            const_stack.clear();
                        }
                    }
                    _ => {
                        new_instructions.push(inst.clone());
                        const_stack.clear();
                    }
                }
            }

            func.instructions = new_instructions;
            self.stats.constants_folded += folded;
        }
    }

    /// Function Inlining - Inline small functions
    fn inlining_pass(&mut self, module: &mut WasmModule) {
        // Find inlineable functions (small, non-recursive)
        let mut inlineable = HashMap::new();

        for (&idx, func) in &module.functions {
            if func.instructions.len() <= self.inline_threshold &&
               !func.is_recursive &&
               !module.is_exported(idx) {
                inlineable.insert(idx, func.clone());
            }
        }

        // Inline calls to these functions
        let mut inlined_count = 0;
        for (_idx, func) in module.functions.iter_mut() {
            let mut new_instructions = Vec::new();

            for inst in &func.instructions {
                if let Instruction::Call(called_idx) = inst {
                    if let Some(inline_func) = inlineable.get(called_idx) {
                        // Inline the function body
                        for inline_inst in &inline_func.instructions {
                            // Skip return instruction when inlining
                            if !matches!(inline_inst, Instruction::Return) {
                                new_instructions.push(inline_inst.clone());
                            }
                        }
                        inlined_count += 1;
                        continue;
                    }
                }
                new_instructions.push(inst.clone());
            }

            func.instructions = new_instructions;
        }

        self.stats.functions_inlined = inlined_count;
    }

    /// Get optimization statistics
    pub fn stats(&self) -> &OptimizationStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = OptimizationStats::default();
    }
}

impl Default for WasmOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

// Simplified WASM module representation for optimization
#[derive(Debug, Clone)]
struct WasmModule {
    functions: HashMap<usize, WasmFunction>,
    exports: Vec<Export>,
    start_function: Option<usize>,
}

impl WasmModule {
    #[allow(unused_variables)] // bytes used in future WASM parsing implementation
    fn parse(bytes: &[u8]) -> Self {
        // Simplified parser - in reality would use wasmparser crate
        // For now, just create a basic module structure
        Self {
            functions: HashMap::new(),
            exports: Vec::new(),
            start_function: None,
        }
    }

    fn encode(&self) -> Vec<u8> {
        // Simplified encoder - in reality would use wasm-encoder crate
        // For now, just return empty WASM module
        vec![
            0x00, 0x61, 0x73, 0x6d, // Magic number "\0asm"
            0x01, 0x00, 0x00, 0x00, // Version 1
        ]
    }

    fn is_exported(&self, func_idx: usize) -> bool {
        self.exports.iter().any(|e| e.kind == ExportKind::Function && e.index == func_idx)
    }
}

#[derive(Debug, Clone)]
struct WasmFunction {
    instructions: Vec<Instruction>,
    called_functions: HashSet<usize>,
    is_recursive: bool,
}

#[derive(Debug, Clone)]
struct Export {
    #[allow(dead_code)] // Used in future WASM export analysis
    name: String,
    kind: ExportKind,
    index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Complete WASM export type set for future features
enum ExportKind {
    Function,
    Memory,
    Global,
    Table,
}

// Simplified WASM instruction set
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // Complete WASM instruction set for future optimizer passes
enum Instruction {
    // Constants
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    // Arithmetic
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,

    // Comparison
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    // Control flow
    Call(usize),
    CallIndirect(usize),
    Return,
    Block,
    Loop,
    If,
    Else,
    End,
    Br(usize),
    BrIf(usize),

    // Variables
    LocalGet(usize),
    LocalSet(usize),
    LocalTee(usize),
    GlobalGet(usize),
    GlobalSet(usize),

    // Memory
    I32Load,
    I32Store,

    // Other
    Drop,
    Select,
    Nop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = WasmOptimizer::new();
        assert!(optimizer.enable_dce);
        assert!(optimizer.enable_constant_folding);
        assert!(optimizer.enable_inlining);
    }

    #[test]
    fn test_aggressive_optimizer() {
        let optimizer = WasmOptimizer::aggressive();
        assert_eq!(optimizer.inline_threshold, 20);
    }

    #[test]
    fn test_minimal_optimizer() {
        let optimizer = WasmOptimizer::minimal();
        assert!(!optimizer.enable_constant_folding);
        assert!(!optimizer.enable_inlining);
    }

    #[test]
    fn test_optimization_stats() {
        let mut stats = OptimizationStats::default();
        stats.original_size = 1000;
        stats.optimized_size = 750;
        stats.functions_removed = 5;
        stats.constants_folded = 10;

        assert_eq!(stats.size_reduction_percent(), 25.0);
        assert_eq!(stats.total_optimizations(), 15);
    }

    #[test]
    fn test_constant_folding() {
        let mut optimizer = WasmOptimizer::new();

        // Create a simple WASM module with constant arithmetic
        let mut module = WasmModule {
            functions: HashMap::new(),
            exports: Vec::new(),
            start_function: None,
        };

        let func = WasmFunction {
            instructions: vec![
                Instruction::I32Const(10),
                Instruction::I32Const(20),
                Instruction::I32Add,
                Instruction::I32Const(5),
                Instruction::I32Mul,
            ],
            called_functions: HashSet::new(),
            is_recursive: false,
        };

        module.functions.insert(0, func.clone());

        optimizer.constant_folding_pass(&mut module);

        // Should fold 10 + 20 = 30, then 30 * 5 = 150
        assert!(optimizer.stats.constants_folded > 0);
    }

    #[test]
    fn test_wasm_module_parse() {
        let bytes = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        let module = WasmModule::parse(&bytes);
        assert_eq!(module.functions.len(), 0);
    }

    #[test]
    fn test_wasm_module_encode() {
        let module = WasmModule {
            functions: HashMap::new(),
            exports: Vec::new(),
            start_function: None,
        };

        let encoded = module.encode();
        assert_eq!(&encoded[0..4], b"\0asm");
    }
}
