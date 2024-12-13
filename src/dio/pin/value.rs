use crate::dio::driver::TSafePicoHaDioDriver;
use panduza_platform_core::{
    log_error, spawn_on_command, Class, EnumAttServer, Error, Instance, InstanceLogger,
};

///
///
///
pub async fn mount(
    mut instance: Instance,
    driver: TSafePicoHaDioDriver,
    mut parent_class: Class,
    pin_num: u32,
) -> Result<(), Error> {
    //
    // Create interface direction
    // let mut direction = parent_class.create_class("direction").finish();

    // direction

    let att_value = parent_class
        .create_attribute("value")
        .with_rw()
        .finish_as_enum(vec!["low".to_string(), "high".to_string()])
        .await?;

    //
    // Execute action on each command received
    let logger_2 = instance.logger.clone();
    let att_value_2 = att_value.clone();
    spawn_on_command!(
        "on_command => value",
        instance,
        att_value_2,
        on_command(
            logger_2.clone(),
            driver.clone(),
            att_value_2.clone(),
            pin_num
        )
    );

    Ok(())
}

///
///
///
async fn on_command(
    logger: InstanceLogger,
    driver: TSafePicoHaDioDriver,
    mut att_value: EnumAttServer,
    pin_num: u32,
) -> Result<(), Error> {
    while let Some(command) = att_value.pop_cmd().await {
        match command {
            Ok(v) => {
                logger.debug(format!("set direction command {:?}", v));

                let mut driver_lock = driver.lock().await;
                driver_lock.pico_set_value(pin_num, v).await?;
                let read_direction = driver_lock.pico_get_value(pin_num).await?;
                drop(driver_lock);

                att_value.set(read_direction).await?;
            }
            Err(e) => {
                log_error!(logger, "Error setting direction {:?}", e);
            }
        }
    }
    Ok(())
}
