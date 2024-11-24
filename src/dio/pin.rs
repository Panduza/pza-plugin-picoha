use panduza_platform_core::{Class, Error, Instance};

use super::driver::TSafePicoHaDioDriver;

///
/// Create dio interface for a given pin number
///
pub async fn mount(
    instance: Instance,
    driver: TSafePicoHaDioDriver,
    mut parent_interface: Class,
    pin_num: u32,
) -> Result<(), Error> {
    //
    // Register interface
    let class_pin = parent_interface
        .create_class(format!("{}", pin_num))
        .finish();

    // //
    // //
    // create_direction_interface(
    //     device.clone(),
    //     pico_connector.clone(),
    //     dio_interface.clone(),
    //     pin_num,
    // )
    // .await?;

    // //
    // //
    // create_value_interface(
    //     device.clone(),
    //     pico_connector.clone(),
    //     dio_interface.clone(),
    //     pin_num,
    // )
    // .await?;

    // io_%d/trigger_read    (boolean) start an input reading (oneshot)

    Ok(())
}
