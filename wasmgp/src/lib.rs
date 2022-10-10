mod migration_algorithm;
mod selection_curve;

pub use migration_algorithm::MigrationAlgorithm;
pub use selection_curve::SelectionCurve;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
