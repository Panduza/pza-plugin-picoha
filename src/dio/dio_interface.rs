use super::connector::PicoHaDioConnector;
use panduza_platform_core::spawn_on_command;
use panduza_platform_core::BidirMsgAtt;
use panduza_platform_core::DeviceLogger;
use panduza_platform_core::Interface;
use panduza_platform_core::StringCodec;
use panduza_platform_core::StringListCodec;
use panduza_platform_core::TaskResult;
use panduza_platform_core::{Device, Error};

///
///
///
async fn on_direction_change(
    logger: DeviceLogger,
    connector: PicoHaDioConnector,
    mut direction_value_attr: BidirMsgAtt<StringCodec>,
    pin_num: u32,
) -> TaskResult {
    while let Some(command) = direction_value_attr.pop_cmd().await {
        logger.debug(format!("set direction command {:?}", command));

        if command.value == "input".to_string() {
            connector.pico_set_direction(pin_num, command.value).await?;
        } else if command.value == "output".to_string() {
            connector.pico_set_direction(pin_num, command.value).await?;
        }

        let read_direction = connector.pico_get_direction(pin_num).await?;

        direction_value_attr.set(read_direction).await?;
    }
    Ok(())
}

///
///
///
pub async fn create_direction_interface(
    mut device: Device,
    pico_connector: PicoHaDioConnector,
    mut parent_interface: Interface,
    pin_num: u32,
) -> Result<(), Error> {
    //
    // Create interface direction
    let mut direction = parent_interface.create_interface("direction").finish();

    // meta : enum ?

    let choices = direction
        .create_attribute("choices")
        .message()
        .with_att_only_access()
        .finish_with_codec::<StringListCodec>()
        .await;

    choices
        .set(vec!["input".to_string(), "output".to_string()])
        .await?;

    let value = direction
        .create_attribute("value")
        .message()
        .with_bidir_access()
        .finish_with_codec::<StringCodec>()
        .await;

    //
    // Execute action on each command received
    let logger = device.logger.clone();
    let value_attr = value.clone();
    let connector = pico_connector.clone();
    spawn_on_command!(
        device,
        value_attr,
        on_direction_change(
            logger.clone(),
            connector.clone(),
            value_attr.clone(),
            pin_num
        )
    );

    // read a first time here then only set when a new value arrive
    value.set("input").await?;

    Ok(())
}

///
///
///
async fn on_value_change(
    logger: DeviceLogger,
    connector: PicoHaDioConnector,
    mut value_value_attr: BidirMsgAtt<StringCodec>,
    pin_num: u32,
) -> TaskResult {
    while let Some(command) = value_value_attr.pop_cmd().await {
        logger.debug(format!("set value command {:?}", command));

        if command.value == "low".to_string() {
            connector.pico_set_value(pin_num, command.value).await?;
        } else if command.value == "high".to_string() {
            connector.pico_set_value(pin_num, command.value).await?;
        }

        let read_value = connector.pico_get_value(pin_num).await?;

        value_value_attr.set(read_value).await?;
    }
    Ok(())
}

///
///
///
pub async fn create_value_interface(
    mut device: Device,
    pico_connector: PicoHaDioConnector,
    mut parent_interface: Interface,
    pin_num: u32,
) -> Result<(), Error> {
    //
    // Create interface direction
    let mut io_value_attr = parent_interface.create_interface("value").finish();

    // meta : enum ?

    let choices = io_value_attr
        .create_attribute("choices")
        .message()
        .with_att_only_access()
        .finish_with_codec::<StringListCodec>()
        .await;

    choices
        .set(vec!["low".to_string(), "high".to_string()])
        .await?;

    let value = io_value_attr
        .create_attribute("value")
        .message()
        .with_bidir_access()
        .finish_with_codec::<StringCodec>()
        .await;

    //
    // Execute action on each command received
    let logger = device.logger.clone();
    let value_attr = value.clone();
    let connector = pico_connector.clone();
    spawn_on_command!(
        device,
        value_attr,
        on_value_change(
            logger.clone(),
            connector.clone(),
            value_attr.clone(),
            pin_num
        )
    );

    // read a first time here then only set when a new value arrive
    value.set("low").await?;

    Ok(())
}

///
/// Create dio interface for a given pin number
///
pub async fn create_dio_interface(
    device: Device,
    pico_connector: PicoHaDioConnector,
    mut parent_interface: Interface,
    pin_num: u32,
) -> Result<(), Error> {
    //
    // Register interface
    let dio_interface = parent_interface
        .create_interface(format!("{}", pin_num))
        .finish();

    //
    //
    create_direction_interface(
        device.clone(),
        pico_connector.clone(),
        dio_interface.clone(),
        pin_num,
    )
    .await?;

    //
    //
    create_value_interface(
        device.clone(),
        pico_connector.clone(),
        dio_interface.clone(),
        pin_num,
    )
    .await?;

    // io_%d/trigger_read    (boolean) start an input reading (oneshot)

    Ok(())
}
