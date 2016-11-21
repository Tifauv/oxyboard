/*!
 * The data storage interfaces and facilities.
 */
pub use self::backend::StorageBackend;
pub use self::file_csv::CsvFileStorage;

// The storage backends are defined in sub-modules
pub mod file_csv;
pub mod backend;
