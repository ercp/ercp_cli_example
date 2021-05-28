use ercp_device::{command::ACK, CommandError, Device, Error};
pub trait DeviceExt {
    fn led_on(&mut self) -> Result<(), Error>;
    fn led_off(&mut self) -> Result<(), Error>;
}

const LED_ON: u8 = 0x20;
const LED_OFF: u8 = 0x21;

impl DeviceExt for Device {
    fn led_on(&mut self) -> Result<(), Error> {
        let reply = self.command(LED_ON, &[])?;

        if reply.command() == ACK {
            Ok(())
        } else {
            Err(CommandError::UnexpectedReply.into())
        }
    }

    fn led_off(&mut self) -> Result<(), Error> {
        let reply = self.command(LED_OFF, &[])?;

        if reply.command() == ACK {
            Ok(())
        } else {
            Err(CommandError::UnexpectedReply.into())
        }
    }
}
