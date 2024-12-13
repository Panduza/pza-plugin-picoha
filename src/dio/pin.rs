mod direction;
mod value;

use panduza_platform_core::{log_debug, Class, Error, Instance};

use super::driver::TSafePicoHaDioDriver;

///
/// Create dio interface for a given pin number
///
pub async fn mount(
    instance: Instance,
    driver: TSafePicoHaDioDriver,
    mut parent_class: Class,
    pin_num: u32,
) -> Result<(), Error> {
    //
    //
    let logger = instance.logger.clone();
    log_debug!(logger, "Mounting pin[{}]...", pin_num);

    //
    // Register interface
    let class_pin = parent_class.create_class(format!("{}", pin_num)).finish();

    //
    // Mount direction
    direction::mount(instance.clone(), driver.clone(), class_pin.clone(), pin_num).await?;

    //
    //
    value::mount(instance.clone(), driver.clone(), class_pin.clone(), pin_num).await?;

    //
    //
    log_debug!(logger, "Mounting pin[{}] => ok", pin_num);
    Ok(())
}
