use log::LevelFilter;

pub const LOGGING_LEVEL: LevelFilter = LevelFilter::Debug;

pub fn start_logging() {
    // set actix logging level
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // start logging
    env_logger::builder().filter_level(LOGGING_LEVEL)
        .init();

}