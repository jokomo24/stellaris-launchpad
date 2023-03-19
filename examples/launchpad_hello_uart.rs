//! This example uses launchpad-rs.

#![no_std]
#![no_main]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate embedded_hal;
extern crate stellaris_launchpad;
extern crate tm4c123x_hal;

use core::fmt::Write;
use tm4c123x_hal::gpio::GpioExt;
use tm4c123x_hal::serial;
use tm4c123x_hal::time::Bps;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

#[no_mangle]
fn stellaris_main(mut board: stellaris_launchpad::board::Board) {
    let mut pins_a = board.GPIO_PORTA.split(&board.power_control);
    let mut uart = serial::Serial::uart0(
        board.UART0,
        pins_a.pa1.into_af_push_pull(&mut pins_a.control),
        pins_a.pa0.into_af_push_pull(&mut pins_a.control),
        (),
        (),
        Bps(115200),
        serial::NewlineMode::SwapLFtoCRLF,
        stellaris_launchpad::board::clocks(),
        &board.power_control,
    );

    writeln!(uart,"Hello, world!").unwrap();
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************