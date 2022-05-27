use std::{thread, time::Duration};

use netraffic::{Filter, Traffic};

fn main() {
    let mut traffic = Traffic::new();
    // rule look here: https://biot.com/capstats/bpf.html
    let rule1 = "port 443";
    let rule2 = "src host 127.0.0.1";
    traffic.add_listener(Filter::new("eth0".to_string(), rule1.to_string()));
    traffic.add_listener(Filter::new("eth0".to_string(), rule2.to_string()));
    loop {
        thread::sleep(Duration::from_millis(1000));
        println!(
            "rule1: {}, traffic: {:#?} Bytes",
            rule1,
            traffic.get_data().get(rule1).unwrap().total
        );
        println!(
            "rule2: {}, traffic: {:#?} Bytes",
            rule2,
            traffic.get_data().get(rule2).unwrap().total
        );
    }
}
