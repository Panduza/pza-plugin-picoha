mod direction;
mod value;

use panduza_platform_core::{
    log_debug, log_debug_mount_end, log_debug_mount_start, Class, Container, Error, Instance,
};

use super::driver::TSafePicoHaDioDriver;

///
/// Create dio interface for a given pin number
///
pub async fn mount<C: Container>(
    mut parent: C,
    interface: TSafePicoHaDioDriver,
    pin_num: u32,
) -> Result<(), Error> {
    //
    // Register interface
    let class_pin = parent.create_class(format!("{}", pin_num)).finish().await;
    let logger = class_pin.logger();
    log_debug_mount_start!(logger);

    //
    // Mount direction
    direction::mount(class_pin.clone(), interface.clone(), pin_num).await?;

    //
    //
    value::mount(class_pin.clone(), interface.clone(), pin_num).await?;

    //
    //
    log_debug_mount_end!(logger);
    Ok(())
}
