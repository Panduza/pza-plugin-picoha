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
    pin_num: usize,
) -> Result<(), Error> {
    //
    //
    let att_dir = parent
        .create_attribute("direction")
        .with_rw()
        .finish_as_enum(vec!["input".to_string(), "output".to_string()])
        .await?;
    let logger = att_dir.logger();
    log_debug_mount_start!(logger);

    //
    //
    let read_direction = interface.lock().await.pico_get_direction(pin_num).await?;
    att_dir.set(read_direction).await?;

    //
    // Execute action on each command received
    let att_dir_2 = att_dir.clone();
    spawn_on_command!(
        "on_command => direction",
        parent,
        att_dir_2,
        on_command(att_dir_2.clone(), interface.clone(), pin_num)
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
    mut att_dir: EnumAttServer,
    interface: TSafePicoHaDioDriver,
    pin_num: usize,
) -> Result<(), Error> {
    while let Some(command) = att_dir.pop_cmd().await {
        let logger = att_dir.logger();
        match command {
            Ok(v) => {
                //
                //
                log_trace!(logger, "set direction command {:?}", v);

                //
                //
                let mut driver_lock = interface.lock().await;
                driver_lock.pico_set_direction(pin_num, v).await?;
                let read_direction = driver_lock.pico_get_direction(pin_num).await?;
                drop(driver_lock);

                //
                //
                log_trace!(logger, "read back {:?}", read_direction);

                //
                //
                att_dir.set(read_direction).await?;
            }
            Err(e) => {
                log_error!(logger, "Error setting direction {:?}", e);
            }
        }
    }
    Ok(())
}
