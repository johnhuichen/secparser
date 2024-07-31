use log::info;

pub fn init() {
    log4rs::init_file("config/log4rs.yml", Default::default()).expect("Should initialize log4rs");
    info!("Starting secparser");
}
