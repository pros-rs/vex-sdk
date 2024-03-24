use core::ffi::c_double;

use crate::{map_jump_table, V5_DeviceT};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_AdiPortConfiguration {
    kAdiPortTypeAnalogIn = 0,
    kAdiPortTypeAnalogOut,
    kAdiPortTypeDigitalIn,
    kAdiPortTypeDigitalOut,

    kAdiPortTypeSmartButton,
    kAdiPortTypeSmartPot,

    kAdiPortTypeLegacyButton,
    kAdiPortTypeLegacyPotentiometer,
    kAdiPortTypeLegacyLineSensor,
    kAdiPortTypeLegacyLightSensor,
    kAdiPortTypeLegacyGyro,
    kAdiPortTypeLegacyAccelerometer,

    kAdiPortTypeLegacyServo,
    kAdiPortTypeLegacyPwm,

    kAdiPortTypeQuadEncoder,
    kAdiPortTypeSonar,

    kAdiPortTypeLegacyPwmSlew,

    kAdiPortTypeUndefined = 255,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum BumperState {
    Released = 0,
    Pressed = 1,
}

map_jump_table! {
    0x208 => pub fn vexDeviceAdiPortConfigSet(device: V5_DeviceT, port: u32, config: V5_AdiPortConfiguration),
    0x20c => pub fn vexDeviceAdiPortConfigGet(device: V5_DeviceT, port: u32) -> V5_AdiPortConfiguration,
    0x210 => pub fn vexDeviceAdiValueSet(device: V5_DeviceT, port: u32, value: i32),
    0x214 => pub fn vexDeviceAdiValueGet(device: V5_DeviceT, port: u32) -> i32,
    0x230 => pub fn vexDeviceBumperGet(device: V5_DeviceT) -> BumperState,
    0x258 => pub fn vexDeviceGyroReset(device: V5_DeviceT),
    0x25c => pub fn vexDeviceGyroHeadingGet(device: V5_DeviceT) -> c_double,
    0x260 => pub fn vexDeviceGyroDegreesGet(device: V5_DeviceT) -> c_double,
    0x280 => pub fn vexDeviceSonarValueGet(device: V5_DeviceT) -> i32,
    0x488 => pub fn vexDeviceAbsEncReset(device: V5_DeviceT),
    0x48c => pub fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: i32),
    0x490 => pub fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> i32,
    0x494 => pub fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> i32,
    0x498 => pub fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> i32,
    0x49c => pub fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool),
    0x4a0 => pub fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool,
    0x4a4 => pub fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> u32,
}