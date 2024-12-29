use crate::dio::driver::TSafePicoHaDioDriver;
use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_error, log_trace, spawn_on_command, Container,
    EnumAttServer, Error,
};

///
///
///
pub async fn mount<C: Container>(
    mut parent: C,
    interface: TSafePicoHaDioDriver,
    pin_num: u32,
) -> Result<(), Error> {
    //
    //
    let att_value = parent
        .create_attribute("value")
        .with_rw()
        .finish_as_enum(vec!["low".to_string(), "high".to_string()])
        .await?;
    let logger = att_value.logger();
    log_debug_mount_start!(logger);

    //
    // Execute action on each command received
    let att_value_2 = att_value.clone();
    spawn_on_command!(
        "on_command => value",
        parent,
        att_value_2,
        on_command(att_value_2.clone(), interface.clone(), pin_num)
    );

    //
    // End
    log_debug_mount_end!(logger);
    Ok(())
}

///
///
///
async fn on_command(
    mut att_value: EnumAttServer,
    driver: TSafePicoHaDioDriver,
    pin_num: u32,
) -> Result<(), Error> {
    while let Some(command) = att_value.pop_cmd().await {
        let logger = att_value.logger();
        match command {
            Ok(v) => {
                log_trace!(logger, "set direction command {:?}", v);

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
