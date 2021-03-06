{{ #with info ~}}
//! {{id}} hardware abstraction layer (HAL)
//!
//! This crate provide an hardware abstraction layer (HAL) for the {{id}} micro-controller.
//! This micro-controller has {{ram_size}}Kb of RAM, {{flash_size}}Kb of flash on a {{package}} package.
//! Read its [datasheet] for the complete device's characteristics.
//!
//! Read the [reference manual] for exhaustive descriptions of the device's features.
//!
//! [datasheet]: {{datasheet_url}}
//! [reference manual]: {{reference_url}}
{{~ /with }}

#![no_std]

// Device's peripherals
{{ #if peripherals.rcc ~}}
pub mod rcc;
{{~ /if }}
{{ #if peripherals.gpio ~}}
pub mod gpio;
{{~ /if }}

mod api;

// Re-export 
pub use crate::api::Constrain;
pub use crate::api::ConstrainFrom;
