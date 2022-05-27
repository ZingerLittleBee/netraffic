use std::{thread, time::Duration};

use netraffic::{Filter, Traffic};

fn main() {
    let mut traffic = Traffic::new();
    let rule = "port 5000 or tcp port 443";
    traffic.add_listener(Filter::new("en0".to_string(), rule.to_string()));
    loop {
        let pre = traffic.get_data().get(rule).unwrap().total;
        thread::sleep(Duration::from_millis(1000));
        let bytes = (traffic.get_data().get(rule).unwrap().total - pre) as f64;
        println!(
            "speed: {:.2?} {}/s",
            if bytes >= 1000.0 * 1000.0 {
                bytes / (1000.0 * 1000.0)
            } else if bytes >= 1000.0 {
                bytes / 1000.0
            } else {
                bytes
            },
            if bytes >= 1000.0 * 1000.0 {
                "MB"
            } else if bytes >= 1000.0 {
                "KB"
            } else {
                "B"
            }
        );
    }
}
