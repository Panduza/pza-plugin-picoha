panduza_platform_core::plugin_interface!("picoha");

mod dio;
mod scanner;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(dio::producer::PiochaDio::new());
    return producers;
}

//
//
pub fn plugin_scanners() -> Vec<Box<dyn Scanner>> {
    let mut scanners: Vec<Box<dyn Scanner>> = vec![];
    scanners.push(scanner::PicohaScanner::default().boxed());
    return scanners;
}
