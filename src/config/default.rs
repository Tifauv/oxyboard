use config::data::{ Config, ServerParams, BoardParams, StorageParams, UiParams };

/**
 * Builds a default configuration.
 *
 * # Examples
 *
 * ```
 * use oxyboard::config;
 *
 * let cfg = config::default();
 * assert_eq!(cfg.server.ip,          String::from("localhost"));
 * assert_eq!(cfg.server.port,        8080);
 * assert_eq!(cfg.board.name,         String::from("oxyboard"));
 * assert_eq!(cfg.board.history_size, 512);
 * assert_eq!(cfg.storage.data_dir,   String::from("data"));
 * assert_eq!(cfg.ui.templates_dir,   String::from("templates"));
 * ```
 */
pub fn default() -> Config {
	Config {
		server: ServerParams {
			ip   : String::from("localhost"),
			port : 8080,
		},

		board : BoardParams {
			name         : String::from("oxyboard"),
			history_size : 512,
		},

		storage: StorageParams {
			data_dir : String::from("data"),
		},

		ui: UiParams {
			templates_dir : String::from("templates"),
		},
	}
}
