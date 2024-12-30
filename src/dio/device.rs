use super::driver::PicoHaDioDriver;
use super::driver::TSafePicoHaDioDriver;
use async_trait::async_trait;
use panduza_platform_core::connector::serial::slip::Driver as SerialSlipDriver;
use panduza_platform_core::connector::serial::Settings as SerialSettings;
use panduza_platform_core::connector::usb::Settings as UsbSettings;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::log_info;
use panduza_platform_core::log_info_mount_end;
use panduza_platform_core::log_info_mount_start;
use panduza_platform_core::Container;
use panduza_platform_core::{DriverOperations, Error, Instance};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

static PICOHA_VENDOR_ID: u16 = 0x16c0;
static PICOHA_PRODUCT_ID: u16 = 0x05e1;
static PICOHA_SERIAL_BAUDRATE: u32 = 9600; // We do not care... it is USB serial

///
/// Device to control PicoHA Dio Board
///
pub struct PicoHaDioDevice {
    //
    // Device logger
    // logger: Option<DeviceLogger>,

    //
    // Serial settings to connect to the pico
    // serial_settings: Option<SerialSettings>,

    //
    // Connector to communicate with the pico
    // connector: Option<Connector>,

    // dio_driver: Option<Arc<Mutex<PicoHaDioDriver>>>,
}

impl PicoHaDioDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        PicoHaDioDevice {}
    }

    ///
    /// Prepare settings of the device
    ///
    pub async fn open_driver(&mut self, instance: Instance) -> Result<TSafePicoHaDioDriver, Error> {
        // Get the device logger
        let logger = instance.logger.clone();

        // Get the device settings
        let json_settings = instance.settings().await.or(Some(json!({}))).unwrap();

        // Log debug info
        logger.info("Build interfaces for \"picoha.dio\" device");
        logger.info(format!("JSON settings: {:?}", json_settings));

        // Usb settings
        let usb_settings = UsbSettings::new()
            .set_vendor(PICOHA_VENDOR_ID)
            .set_model(PICOHA_PRODUCT_ID)
            .optional_set_serial_from_json_settings(&json_settings);
        logger.info(format!("USB settings: {}", usb_settings));

        // Serial settings
        let serial_settings = SerialSettings::new()
            .set_port_name_from_json_or_usb_settings(&json_settings, &usb_settings)
            .map_err(|e| Error::Generic(e.to_string()))?
            .set_baudrate(PICOHA_SERIAL_BAUDRATE);

        let low_driver = SerialSlipDriver::open(&serial_settings)?;

        let driver = PicoHaDioDriver::new(logger, low_driver).into_tsafe();

        Ok(driver)
    }
}

#[async_trait]
impl DriverOperations for PicoHaDioDevice {
    ///
    ///
    ///
    async fn mount(&mut self, mut instance: Instance) -> Result<(), Error> {
        //
        // Init logger
        let logger = instance.logger.clone();
        log_info_mount_start!(logger);

        //
        //
        let driver = self.open_driver(instance.clone()).await?;

        //
        // Create pin class
        let class_pin = instance.create_class("pin").finish().await;

        //
        //
        let available_pins: Vec<usize> = (0..22)
            .filter(|&pin| pin != 0 && pin != 1)
            .filter(|&pin| pin != 23 && pin != 24 && pin != 25)
            .collect();

        //
        //
        for pin_num in available_pins {
            super::pin::mount(class_pin.clone(), driver.clone(), pin_num).await?;
        }

        //
        // End
        log_info_mount_end!(logger);
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _instance: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
