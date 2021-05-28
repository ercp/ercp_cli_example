use ercp_device::{
    command::{ACK, NACK},
    CommandError, Device, Error,
};
pub trait DeviceExt {
    fn led_on(&mut self) -> Result<(), Error>;
    fn led_off(&mut self) -> Result<(), Error>;
    fn counter_get(&mut self) -> Result<u8, Error>;
    fn counter_set(&mut self, value: u8) -> Result<(), Error>;
    fn counter_inc(&mut self) -> Result<(), Error>;
    fn counter_dec(&mut self) -> Result<(), Error>;
}

const LED_ON: u8 = 0x20;
const LED_OFF: u8 = 0x21;
const COUNTER_GET: u8 = 0x30;
const COUNTER_GET_REPLY: u8 = 0x31;
const COUNTER_SET: u8 = 0x32;
const COUNTER_INC: u8 = 0x33;
const COUNTER_DEC: u8 = 0x34;

const OUT_OF_BOUNDS: u8 = 0xFF;

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

    fn counter_get(&mut self) -> Result<u8, Error> {
        let reply = self.command(COUNTER_GET, &[])?;

        if reply.command() == COUNTER_GET_REPLY && reply.length() == 1 {
            Ok(reply.value()[0])
        } else {
            Err(CommandError::UnexpectedReply.into())
        }
    }

    fn counter_set(&mut self, value: u8) -> Result<(), Error> {
        let reply = self.command(COUNTER_SET, &[value])?;

        if reply.command() == ACK {
            Ok(())
        } else {
            Err(CommandError::UnexpectedReply.into())
        }
    }

    fn counter_inc(&mut self) -> Result<(), Error> {
        let reply = self.command(COUNTER_INC, &[])?;

        if reply.command() == ACK {
            Ok(())
        } else if reply.command() == NACK && reply.value() == [OUT_OF_BOUNDS] {
            todo!()
        } else {
            Err(CommandError::UnexpectedReply.into())
        }
    }

    fn counter_dec(&mut self) -> Result<(), Error> {
        let reply = self.command(COUNTER_DEC, &[])?;

        if reply.command() == ACK {
            Ok(())
        } else if reply.command() == NACK && reply.value() == [OUT_OF_BOUNDS] {
            todo!()
        } else {
            Err(CommandError::UnexpectedReply.into())
        }
    }
}
