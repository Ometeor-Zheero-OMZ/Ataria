// use chrono::Local;
// use std::fs::OpenOptions;
// use std::io::Write;
// use std::path::Path;
// use lazy_static::lazy_static;

// use crate::PROJECT_PATH;

// lazy_static! {
//     pub static ref LOG_PATH: String = format!("{}/log/axum.log", PROJECT_PATH);
// }

// #[allow(dead_code)]
// pub enum Header {
//     SUCCESS,
//     INFO,
//     WARNING,
//     ERROR
// }

// pub fn log(header: Header, message: &str) {
//     let header = match header {
//         Header::SUCCESS => "SUCCESS",
//         Header::INFO => "INFO",
//         Header::WARNING => "WARNING",
//         Header::ERROR => "ERROR"
//     };

//     println!("[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message);

//     if Path::new(&*LOG_PATH).exists() {
//         // If the log file already exists, append the new log entry
//         let mut log_file = OpenOptions::new().append(true).open(&*LOG_PATH).unwrap();
//         writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
//     } else {
//         // If the log file does not exist, create it and then append the log entry
//         let mut log_file = OpenOptions::new().create_new(true).append(true).open(&*LOG_PATH).unwrap();
//         writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
//     }
// }

// #[macro_export]
// macro_rules! app_log {
//     // 引数あり
//     ($header:expr, $msg:expr, $($arg:tt)*) => {
//         let formatted_message = format!($msg, $($arg)*);
//         crate::application::helpers::logger::log($header, &formatted_message);
//     };
//     // 引数なし
//     ($header:expr, $msg:expr) => {
//         crate::application::helpers::logger::log($header, $msg);
//     };
// }

// #[macro_export]
// macro_rules! success_log {
//     ($msg:expr, $($arg:tt)*) => {
//         app_log!(crate::application::helpers::logger::Header::SUCCESS, $msg, $($arg)*);
//     };
//     ($msg:expr) => {
//         app_log!(crate::application::helpers::logger::Header::SUCCESS, $msg);
//     }
// }

// #[macro_export]
// macro_rules! info_log {
//     ($msg:expr, $($arg:tt)*) => {
//         app_log!(crate::application::helpers::logger::Header::INFO, $msg, $($arg)*);
//     };
//     ($msg:expr) => {
//         app_log!(crate::application::helpers::logger::Header::INFO, $msg);
//     };
// }

// #[macro_export]
// macro_rules! warning_log {
//     ($msg:expr, $($arg:tt)*) => {
//         app_log!(crate::application::helpers::logger::Header::WARNING, $msg, $($arg)*);
//     };
//     ($msg:expr) => {
//         app_log!(crate::application::helpers::logger::Header::WARNING, $msg);
//     };
// }

// #[macro_export]
// macro_rules! error_log {
//     ($msg:expr, $($arg:tt)*) => {
//         app_log!(crate::application::helpers::logger::Header::ERROR, $msg, $($arg)*);
//     };
//     ($msg:expr) => {
//         app_log!(crate::application::helpers::logger::Header::ERROR, $msg);
//     };
// }