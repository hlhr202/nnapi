use crate::{IntoResult, Result};
use nnapi_sys::{
    ANeuralNetworksDevice, ANeuralNetworksDevice_getName, ANeuralNetworks_getDevice,
    ANeuralNetworks_getDeviceCount,
};
use std::fmt::Display;

#[derive(Debug)]
pub struct Device {
    pub _inner: *const ANeuralNetworksDevice,
    pub index: u32,
    pub name: String,
}

impl Clone for Device {
    fn clone(&self) -> Self {
        Self {
            _inner: self._inner,
            index: self.index,
            name: self.name.clone(),
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Device: {}", self.name)
    }
}

impl Device {
    pub fn num_devices() -> Result<u32> {
        let mut num_devices = 0u32;
        unsafe { ANeuralNetworks_getDeviceCount(&mut num_devices) }.into_result()?;
        Ok(num_devices)
    }

    pub fn get_devices() -> Result<Vec<Self>> {
        let num_devices = Device::num_devices()?;
        let mut devices = Vec::with_capacity(num_devices as usize);

        for index in 0..num_devices {
            let mut device: *mut ANeuralNetworksDevice = std::ptr::null_mut();
            unsafe { ANeuralNetworks_getDevice(index, &mut device) }.into_result()?;
            let mut name: *const u8 = std::ptr::null_mut();
            unsafe { ANeuralNetworksDevice_getName(device, &mut name) }.into_result()?;
            let name = unsafe { std::ffi::CStr::from_ptr(name) }
                .to_string_lossy()
                .to_string();

            devices.push(Self {
                index,
                name,
                _inner: device,
            });
        }

        Ok(devices)
    }
}
