use pcap::{Device, Error};

pub fn get_device() -> Result<Vec<Device>, Error> {
    Device::list()
}

pub fn get_default_device() -> Result<Device, Error> {
    Device::lookup()
}

#[cfg(test)]
mod test {
    use crate::device::get_default_device;

    use super::get_device;

    #[test]
    fn test_get_device() {
        assert!(get_device().unwrap().len() > 0);
    }

    #[test]
    fn test_get_default_device() {
        assert!(get_default_device().is_ok());
    }
}
