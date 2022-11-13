mod code;
mod code_builder;
mod code_context;
mod convert;
mod function_signature;
mod main_entry_point;
mod migration_algorithm;
mod selection_curve;
mod slot;
mod threading_model;
mod value_type;
mod wasm_ast_assumptions;
mod world;
mod world_configuration;

pub use code::Code;
pub use code_context::CodeContext;
pub use function_signature::FunctionSignature;
pub use main_entry_point::MainEntryPoint;
pub use migration_algorithm::MigrationAlgorithm;
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
