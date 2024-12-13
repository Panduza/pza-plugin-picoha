use panduza_platform_core::drivers::serial::Settings as SerialSettings;
use panduza_platform_core::ProductionOrder;
use panduza_platform_core::Scanner;

static PICOHA_VENDOR_ID: u16 = 0x16c0;
static PICOHA_PRODUCT_ID: u16 = 0x05e1;

///
///
///
#[derive(Default)]
pub struct PicohaScanner {}

impl PicohaScanner {
    ///
    ///
    ///
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Scanner for PicohaScanner {
    ///
    ///
    ///
    fn name(&self) -> String {
        "picoha".to_string()
    }

    ///
    ///
    ///
    fn scan(&self) -> Vec<ProductionOrder> {
        let mut results = Vec::new();

        let search_results = SerialSettings::available_usb_serial_ports_with_ids(
            PICOHA_VENDOR_ID,
            PICOHA_PRODUCT_ID,
        );
        match search_results {
            Ok(ports) => {
                for (name, serial_number) in ports {
                    println!("{:?} {:?}", name, &serial_number);

                    let po = ProductionOrder::new("panudza.picoha-dio", serial_number.clone())
                        .add_string_setting("usb_serial", serial_number.clone());

                    results.push(po);
                }
            }
            Err(_e) => {}
        }

        results
    }
}
