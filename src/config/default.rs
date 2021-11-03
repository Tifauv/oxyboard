use crate::config::data::{ Config, BoardParams, StorageParams };

/// Builds a default configuration.
///
/// # Examples
///
/// ```
/// use oxyboard::config;
///
/// let cfg = config::default();
/// assert_eq!(cfg.board.name,         String::from("oxyboard"));
/// assert_eq!(cfg.board.history_size, 512);
/// assert_eq!(cfg.storage.data_dir,   String::from("data"));
/// ```
pub fn default() -> Config {
	Config {
		board : BoardParams {
			name         : String::from("oxyboard"),
			history_size : 512,
		},

		storage: StorageParams {
			data_dir : String::from("data"),
		},
	}
}
