use std::sync::Arc;

use super::api_dio::{PicohaDioAnswer, PicohaDioRequest, PinValue, RequestType};
use crate::dio::api_dio::AnswerType;
use panduza_platform_core::drivers::serial::slip::Driver as SerialSlipDriver;
use panduza_platform_core::{log_trace, Error, InstanceLogger};
use prost::Message;
use tokio::sync::Mutex;

///
/// Connector dedicated to the picoha dio communication
///
pub struct PicoHaDioDriver {
    ///
    /// Device logger
    logger: InstanceLogger,

    ///
    /// Connector to communicate with the pico
    low_driver: SerialSlipDriver,
}

/// Thread Safe
///
pub type TSafePicoHaDioDriver = Arc<Mutex<PicoHaDioDriver>>;

impl PicoHaDioDriver {
    ///
    /// Constructor
    ///
    pub fn new(logger: InstanceLogger, low_driver: SerialSlipDriver) -> Self {
        PicoHaDioDriver {
            logger: logger,
            low_driver: low_driver,
        }
    }

    pub fn into_tsafe(self) -> TSafePicoHaDioDriver {
        Arc::new(Mutex::new(self))
    }

    ///
    /// Send a request and get the answer
    ///
    async fn send_then_receive(
        &mut self,
        request: &PicohaDioRequest,
    ) -> Result<PicohaDioAnswer, Error> {
        //
        // Get the data vector
        let data_vec = &request.encode_to_vec();

        //
        //
        log_trace!(
            self.logger,
            "Send request data on serial port {:?}",
            data_vec
        );

        //
        // Send and receive
        let answer_buffer = &mut [0u8; 1024];
        let size = self
            .low_driver
            .write_then_read(data_vec, answer_buffer)
            .await?;

        // Decode the answer
        let answer_slice = answer_buffer[..size].as_ref();

        //
        //
        log_trace!(self.logger, "Received {} bytes -> {:?}", size, answer_slice);

        //
        // Decode
        let answer = PicohaDioAnswer::decode(answer_slice)
            .map_err(|_| Error::Generic("invalid direction value".to_string()))?;

        // Return the decoded answer
        Ok(answer)
    }

    ///
    /// Communicate with the pico to get the pin direction
    ///
    pub async fn pico_get_direction(&mut self, pin_num: u32) -> Result<String, Error> {
        //
        //
        log_trace!(self.logger, "pico_get_direction(pin={:?})", pin_num);

        // Create the request
        let mut request = PicohaDioRequest::default();
        request.set_type(RequestType::GetPinDirection);
        request.pin_num = pin_num;

        // Communication with the pico
        let answer = self.send_then_receive(&request).await?;

        // Debug log
        log_trace!(self.logger, "decoded {:?}", answer);

        if answer.value.is_none() {
            return Err(Error::Generic("Answer from pico has no value".to_string()));
        }

        // Convert direction answer into a string value
        match PinValue::try_from(answer.value.unwrap()) {
            Ok(value) => match value {
                PinValue::Low => Err(Error::Generic("invalid direction value".to_string())),
                PinValue::High => Err(Error::Generic("invalid direction value".to_string())),
                PinValue::Input => Ok("input".to_string()),
                PinValue::Output => Ok("output".to_string()),
            },
            Err(_) => Err(Error::Generic("invalid direction value".to_string())),
        }
    }

    ///
    ///
    ///
    pub async fn pico_set_direction(
        &mut self,
        pin_num: u32,
        direction: String,
    ) -> Result<(), Error> {
        // Debug log
        self.logger.debug(format!(
            "pico_set_direction({:?}, {:?})",
            pin_num, direction
        ));

        //
        // Create the request
        let mut request = PicohaDioRequest::default();
        request.set_type(RequestType::SetPinDirection);
        request.pin_num = pin_num;

        if direction == "input" {
            request.value = PinValue::Input.into();
        } else if direction == "output" {
            request.value = PinValue::Output.into();
        }

        // Communication with the pico
        let answer = self.send_then_receive(&request).await?;

        println!("{:?}", answer);

        AnswerType::try_from(answer.r#type)
            .map_err(|e| Error::Generic("Unable to parse answer type".to_string()))
            .and_then(|t| match t {
                AnswerType::Success => Ok(()),
                AnswerType::Failure => Err(Error::Generic("Command Failed".to_string())),
            })?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn pico_get_value(&mut self, pin_num: u32) -> Result<String, Error> {
        // Debug log
        self.logger.debug(format!("pico_get_value({:?})", pin_num));

        // Create the request
        let mut request = PicohaDioRequest::default();
        request.set_type(RequestType::GetPinValue);
        request.pin_num = pin_num;

        // Communication with the pico
        let answer = self.send_then_receive(&request).await?;

        // Debug log
        self.logger.debug(format!("decoded {:?}", answer));

        if answer.value.is_none() {
            return Err(Error::Generic("Answer from pico has no value".to_string()));
        }

        // Convert direction answer into a string value
        match PinValue::try_from(answer.value.unwrap()) {
            Ok(value) => match value {
                PinValue::Low => Ok("low".to_string()),
                PinValue::High => Ok("high".to_string()),
                PinValue::Input => Err(Error::Generic("invalid value".to_string())),
                PinValue::Output => Err(Error::Generic("invalid value".to_string())),
            },
            Err(_) => Err(Error::Generic("invalid direction value".to_string())),
        }
    }

    ///
    ///
    ///
    pub async fn pico_set_value(&mut self, pin_num: u32, direction: String) -> Result<(), Error> {
        // Debug log
        self.logger
            .debug(format!("pico_set_value({:?}, {:?})", pin_num, direction));

        //
        // Create the request
        let mut request = PicohaDioRequest::default();
        request.set_type(RequestType::SetPinValue);
        request.pin_num = pin_num;

        if direction == "low" {
            request.value = PinValue::Low.into();
        } else if direction == "high" {
            request.value = PinValue::High.into();
        }

        // Communication with the pico
        let answer = self.send_then_receive(&request).await?;

        println!("{:?}", answer);

        AnswerType::try_from(answer.r#type)
            .map_err(|e| Error::Generic("Unable to parse answer type".to_string()))
            .and_then(|t| match t {
                AnswerType::Success => Ok(()),
                AnswerType::Failure => Err(Error::Generic("Command Failed".to_string())),
            })?;

        Ok(())
    }
}
