// use log::SetLoggerError;
// Disable warnings
#[allow(unused_macros)]

#[macro_export]
macro_rules! log {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[macro_export]
macro_rules! debug_log {
    ($( $args:expr ),*) => {
        if cfg!(debug_assertions) {
            println!( $( $args ),* );
        }
    }
}

// pub fn init_logger() -> Result<(), SetLoggerError> {
//     env_logger::Builder::from_default_env()
//         .format_level(true)
//         .format_timestamp_nanos()
//         .try_init()
// }