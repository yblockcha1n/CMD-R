use env_logger::Builder;
use log::LevelFilter;

pub fn setup_logger() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .filter(Some("serenity"), LevelFilter::Warn)
        .filter(Some("tracing"), LevelFilter::Warn)
        .format_timestamp(None)
        .format_module_path(false)
        .init();
}