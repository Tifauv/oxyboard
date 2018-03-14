//!
/// The configuration data structure and loaders.
pub use self::data::Config;
pub use self::data::ServerParams;
pub use self::data::BoardParams;
pub use self::data::StorageParams;
pub use self::data::UiParams;
pub use self::default::default;
pub use self::loader::ConfigLoader;
pub use self::toml::TomlConfigLoader;

// The loaders are defined in sub-modules
pub mod data;
pub mod loader;
pub mod default;
pub mod toml;

