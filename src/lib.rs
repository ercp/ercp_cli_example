mod device;

use ercp_cli::{
    opts::{Component, Connection},
    Router,
};
use ercp_device::Device;
use structopt::StructOpt;

use device::DeviceExt;

pub struct Cli {
    opts: Opts,
    router: CustomRouter,
}

/// A CLI for driving the example ERCP firmware
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
pub struct Opts {
    #[structopt(flatten)]
    connection: Connection,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Tests communication with the device
    Ping,

    /// Commands the LED
    Led {
        #[structopt(subcommand)]
        command: LedCommand,
    },

    /// Commands the counter
    Counter {
        #[structopt(subcommand)]
        command: CounterCommand,
    },

    /// Gets the version of a component
    Version { component: Component },

    /// Gets the device description
    Description,

    /// Waits for and prints logs sent by the device
    Log,
}

#[derive(Debug, StructOpt)]
pub enum LedCommand {
    /// Switches on the LED
    On,

    /// Switches off the LED
    Off,
}

#[derive(Debug, StructOpt)]
pub enum CounterCommand {
    /// Gets the counter value
    Get,

    /// Sets the counter value
    Set { value: u8 },

    /// Increments the counter value
    Inc,

    /// Decrements the counter value
    Dec,
}

pub struct CustomRouter {
    device: Device,
}

impl Router for CustomRouter {
    type Command = Command;

    fn device(&mut self) -> &mut Device {
        &mut self.device
    }

    fn route(&mut self, command: &Self::Command) {
        match command {
            Command::Ping => self.ping(),
            Command::Led { command } => match command {
                LedCommand::On => self.led_on(),
                LedCommand::Off => self.led_off(),
            },
            Command::Counter { command } => match command {
                CounterCommand::Get => self.counter_get(),
                CounterCommand::Set { value } => self.counter_set(*value),
                CounterCommand::Inc => self.counter_inc(),
                CounterCommand::Dec => self.counter_dec(),
            },
            Command::Version { component } => self.version(component),
            Command::Description => self.description(),
            Command::Log => self.log(),
        }
    }
}

impl CustomRouter {
    pub fn new(device: Device) -> Self {
        Self { device }
    }

    fn led_on(&mut self) {
        match self.device().led_on() {
            Ok(()) => println!("Device: ACK"),
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn led_off(&mut self) {
        match self.device().led_off() {
            Ok(()) => println!("Device: ACK"),
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn counter_get(&mut self) {
        match self.device().counter_get() {
            Ok(value) => println!("Counter: {}", value),
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn counter_set(&mut self, value: u8) {
        match self.device().counter_set(value) {
            Ok(()) => println!("Device: ACK"),
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn counter_inc(&mut self) {
        match self.device().counter_inc() {
            Ok(()) => println!("Device: ACK"),
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn counter_dec(&mut self) {
        match self.device().counter_dec() {
            Ok(()) => println!("Device: ACK"),
            Err(_) => eprintln!("An error has occured"),
        }
    }
}

impl Cli {
    pub fn new(opts: Opts) -> Self {
        let device = Device::new(&opts.connection.port);
        let router = CustomRouter::new(device);

        Self { opts, router }
    }

    pub fn run(&mut self) {
        self.router.route(&self.opts.command);
    }
}
