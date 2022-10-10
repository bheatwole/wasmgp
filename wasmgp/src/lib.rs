mod migration_algorithm;
mod selection_curve;
mod threading_model;

pub use migration_algorithm::MigrationAlgorithm;
pub use selection_curve::SelectionCurve;
pub use threading_model::ThreadingModel;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
