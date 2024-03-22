use log::{warn, info};

pub fn logger() {
    env_logger::init();
    info!("starting up");
    warn!("oops! not implementated");
}