use log::info;
use std::error::Error;

pub fn init() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("config/log4rs.yml", Default::default())
        .expect("Panic when initializing log4rs");
    info!("Starting secparser");

    Ok(())
}
