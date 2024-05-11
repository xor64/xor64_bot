
pub(crate) fn init() {
    env_logger::builder()
        .filter_module("xorot", log::LevelFilter::Info)
        .init();
}