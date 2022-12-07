mod code;
mod code_arithmetic;
mod code_bit_ops;
mod code_builder;
mod code_compare;
mod code_const;
mod code_context;
mod code_control;
mod code_float;
mod code_stream;
mod convert;
mod error;
mod function_signature;
mod genetic_engine;
mod genetic_engine_configuration;
mod indentation;
mod individual;
mod island;
mod island_callbacks;
mod migration_algorithm;
mod run_result;
mod selection_curve;
mod slot;
mod threading_model;
mod value_type;
mod wasm_ast_assumptions;
mod world;
mod world_configuration;

pub use code::Code;
pub use code_arithmetic::*;
pub use code_bit_ops::*;
pub use code_builder::CodeBuilder;
pub use code_compare::*;
pub use code_const::*;
pub use code_context::CodeContext;
pub use code_control::*;
pub use code_float::*;
pub use code_stream::*;
pub use error::WasmgpError;
pub use function_signature::FunctionSignature;
pub use genetic_engine::GeneticEngine;
pub use genetic_engine_configuration::*;
pub use individual::Individual;
pub use island::Island;
pub use island_callbacks::IslandCallbacks;
pub use migration_algorithm::MigrationAlgorithm;
pub use run_result::*;
pub use selection_curve::SelectionCurve;
pub use slot::*;
pub use threading_model::ThreadingModel;
pub use value_type::ValueType;
pub use world::*;
pub use world_configuration::WorldConfiguration;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
