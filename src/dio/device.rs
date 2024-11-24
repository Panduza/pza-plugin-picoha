use super::driver::PicoHaDioDriver;
use super::driver::TSafePicoHaDioDriver;
use async_trait::async_trait;
use panduza_platform_core::drivers::serial::slip::Driver as SerialSlipDriver;
use panduza_platform_core::drivers::serial::Settings as SerialSettings;
use panduza_platform_core::drivers::usb::Settings as UsbSettings;
use panduza_platform_core::spawn_on_command;
use panduza_platform_core::BidirMsgAtt;
use panduza_platform_core::DeviceLogger;
use panduza_platform_core::StringCodec;
use panduza_platform_core::StringListCodec;
use panduza_platform_core::TaskResult;
use panduza_platform_core::{DriverOperations, Error, Instance};
use serde_json::json;
use std::fmt::format;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
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

        // let driver = PicoHaDioDriver::open(settings, vec![b'\n'])?;

        // let kdriver = KoradDriver::new(driver);

        // Ok(Arc::new(Mutex::new(kdriver)))

        Ok(())
    }

    // ///
    // /// Try to mount the connector to reach the device
    // ///
    // pub async fn mount_connector(&mut self) -> Result<(), Error> {
    //     //
    //     // Recover settings
    //     let settings = self.serial_settings.as_ref().ok_or(Error::BadSettings(
    //         "Serial Settings not provided".to_string(),
    //     ))?;
    //     //
    //     // Try to get connector
    //     self.connector = Some(
    //         get_connector(settings)
    //             .await
    //             .map_err(|e| Error::Generic(e.to_string()))?,
    //     );
    //     //
    //     // Try to init it
    //     self.connector
    //         .as_ref()
    //         .ok_or(Error::BadSettings(
    //             "Connector is not initialized".to_string(),
    //         ))?
    //         .lock()
    //         .await
    //         .init()
    //         .await
    //         .map_err(|e| Error::Generic(e.to_string()))?;

    //     //
    //     self.driver = Some(PicoHaDioConnector::new(
    //         self.logger.as_ref().unwrap().clone(),
    //         self.connector.as_ref().unwrap().clone(),
    //     ));

    //     Ok(())
    // }

    // ///
    // /// Create io interfaces
    // ///
    // pub async fn create_io_interfaces(&mut self, mut instance: Instance) -> Result<(), Error> {
    //     // Get the device logger
    //     let logger = instance.logger.clone();

    //     //
    //     //
    //     // let mut array = Vec::new();
    //     for n in 0..5 {
    //         // Debug log
    //         logger.debug(format!("Create io_{}", n));

    //         //
    //         create_dio_interface(
    //             instance.clone(),
    //             self.driver.clone().unwrap(),
    //             interface.clone(),
    //             n,
    //         )
    //         .await?;
    //     }

    //     Ok(())
    // }
}

#[async_trait]
impl DriverOperations for PicoHaDioDevice {
    ///
    ///
    ///
    async fn mount(&mut self, mut instance: Instance) -> Result<(), Error> {
        //
        // Init logger
        // self.logger = Some(instance.logger.clone());

        // self.prepare_settings(instance.clone()).await?;
        // self.mount_connector().await?;

        //
        // Create pin class
        let class_pin = instance.create_class("pin").finish();

        //
        //
        for pin_num in 0..5 {
            // // Debug log
            // logger.debug(format!("Create io_{}", n));

            //
            super::pin::mount(instance, driver, class_pin, pin_num).await?;
        }

        // self.create_io_interfaces(instance.clone()).await?;

        // self.pico_get_direction(2).await?;

        // une interface pour chaque io_%d
        //
        // io_%d/direction              meta : enum
        // io_%d/direction/choices      list of string
        // io_%d/direction/value        string
        // io_%d/value           (enum/string) set/get (when input cannot be set)
        // io_%d/trigger_read    (boolean) start an input reading (oneshot)
        //

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut instance: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
