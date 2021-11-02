/// This the root of the configuration structure.
///
/// # Examples
///
/// ```
/// use oxyboard::config::{ Config, BoardParams, StorageParams };
///
/// let cfg = Config {
///         board: BoardParams {
///             name         : String::from("oxyboard"),
///             history_size : 512,
///         },
///         storage: StorageParams {
///             data_dir : String::from("data"),
///         },
/// };
/// assert_eq!(cfg.board.name,         String::from("oxyboard"));
/// assert_eq!(cfg.board.history_size, 512);
/// assert_eq!(cfg.storage.data_dir,   String::from("data"));
/// ```
#[derive(Debug,Deserialize)]
pub struct Config {
	pub board   : BoardParams,
	pub storage : StorageParams,
}


/// The board parameters define the served board (name, history size...)
///
/// # Examples
///
/// ```
/// use oxyboard::config::BoardParams;
///
/// let board_cfg = BoardParams {
///         name         : String::from("oxyboard"),
///         history_size : 512,
/// };
/// assert_eq!(board_cfg.name,         String::from("oxyboard"));
/// assert_eq!(board_cfg.history_size, 512);
/// ```
#[derive(Debug,Deserialize)]
pub struct BoardParams {
	pub name         : String,
	pub history_size : usize,
}


/// The storage parameters define where and how the data are saved.
///
/// # Examples
///
/// ```
/// use oxyboard::config::StorageParams;
///
/// let storage_cfg = StorageParams {
///         data_dir : String::from("data"),
/// };
/// assert_eq!(storage_cfg.data_dir, String::from("data"));
/// ```
#[derive(Debug,Deserialize)]
pub struct StorageParams {
	pub data_dir : String,
}
