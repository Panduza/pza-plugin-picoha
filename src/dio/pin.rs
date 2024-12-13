mod direction;
mod value;

use panduza_platform_core::{Class, Error, Instance};

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
    // Register interface
    let class_pin = parent_class.create_class(format!("{}", pin_num)).finish();

    //
    // Mount direction
    direction::mount(instance.clone(), driver.clone(), class_pin.clone(), pin_num).await?;

    value::mount(instance.clone(), driver.clone(), class_pin.clone(), pin_num).await?;

    Ok(())
}
