use super::device::PicoHaDioDevice;
use panduza_platform_core::Props;
use panduza_platform_core::{DriverOperations, Producer};

pub struct PiochaDio {}

impl PiochaDio {
    pub fn new() -> Box<PiochaDio> {
        Box::new(PiochaDio {})
    }
}

impl Producer for PiochaDio {
    fn manufacturer(&self) -> String {
        "panduza".to_string()
    }

    fn model(&self) -> String {
        "picoha-dio".to_string()
    }

    fn description(&self) -> String {
        "".to_string()
    }

    fn props(&self) -> Props {
        Props::default()
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(PicoHaDioDevice::new()));
    }
}
