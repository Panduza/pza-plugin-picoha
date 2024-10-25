panduza_platform_core::plugin_interface!("picoha");

mod dio;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(dio::producer::PiochaDio::new());
    return producers;
}
