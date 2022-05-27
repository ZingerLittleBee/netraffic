use netraffic::device::{get_default_device, get_device};

fn main() {
    let devices = get_device().unwrap();
    println!("device: {:#?}", devices);

    let device = get_default_device().unwrap();
    println!("default device: {:#?}", device);
}
