extern crate iron;
extern crate persistent;
extern crate rustc_serialize;
extern crate time;
extern crate toml;


/**
 * This macro prints an infomation message prefixed by Unicode character
 * 'CIRCLED LATIN SMALL LETTER I' (U+24D8).
 */
#[macro_export]
macro_rules! info_msg {
	( $tmpl: tt )               => ( println!(concat!("\u{24d8} ", $tmpl)) );
	( $tmpl: tt, $($arg: tt)* ) => ( println!(concat!("\u{24d8} ", $tmpl), $($arg)*) )
}


/**
 * This macro prints a warning message prefixed by Unicode character
 * 'WARNING SIGN' (U+26A0).
 */
#[macro_export]
macro_rules! warn_msg {
	( $tmpl: tt )               => ( println!(concat!("\u{26a0} ", $tmpl)) );
	( $tmpl: tt, $($arg: tt)* ) => ( println!(concat!("\u{26a0} ", $tmpl), $($arg)*) )
}


pub mod config;
pub mod history;
pub mod history_recorder;
pub mod post;
pub mod requests;
pub mod storage;
