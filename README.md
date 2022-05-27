Language : [🇺🇸 English](./README.md) | 🇨🇳 简体中文

<h1 align="center">netraffic</h1>
<div align="center">

[![Build Status](https://img.shields.io/crates/v/netraffic)](https://crates.io/crates/netraffic)
![Crates Downloads](https://img.shields.io/crates/d/netraffic)
![Last Commit](https://img.shields.io/github/last-commit/ZingerLittleBee/netraffic)

</div>
<div align="center">

[![Docs](https://img.shields.io/docsrs/netraffic)](https://docs.rs/netraffic/0.1.0/netraffic/)
[![GitHub Actions CI](https://img.shields.io/github/workflow/status/ZingerLittleBee/netraffic/Test%20CI)](https://github.com/ZingerLittleBee/netraffic/actions)
[![LICENSE](https://img.shields.io/crates/l/netraffic)](./LICENSE)

</div>

## Overview
netraffic is a rust library that provides ability to **statistics network traffic**.

## Prerequisites
### Windows
Download the [WinPcap](http://www.winpcap.org/install/default.htm) Developer's Pack. Add the `/Lib` or `/Lib/x64` folder to your LIB environment variable.

### Linux
Install `libpcap`

On Debian based Linux, `apt install libpcap-dev`

### Mac OS X
libpcap should be installed on Mac OS X by default.

## Installation
1. Get the latest version -> https://crates.io/crates/netraffic

2. Add the dependent
```toml
[dependencies]
netraffic = "0.1.0"
```

3. Usage
```rust
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
```
> Learn More [Examples](#examples)


## Goods
struct -> [Traffic](#traffic) · [Filter](#filter) · [Snapshot](#snapshot)

enum -> [Action](#action)

mod (`device`) -> [get_device](#get_device) · [get_default_device](#get_default_device)

## Documentation
### `Traffic`
```rust
impl Traffic {
    /// Init traffic
    pub fn new() -> Self
    /// Add a new filter to the traffic data center.
    pub fn add_listener(&mut self, filter: Filter)
    /// Remove a filter from the traffic data center.
    pub fn remove_listener(&self, rule: String)
    /// Suspend a listener by rule.
    pub fn suspend_listener(&self, rule: String)
    /// Resume a listener by rule.
    pub fn resume_listener(&self, rule: String)
    /// Get the traffic snapshot, until Rwlock is free.
    pub fn get_data(&self) -> HashMap<String, Snapshot>
    /// Try to get the traffic snapshot.
    /// if Rwlock is locked, return None.
    pub fn try_get_data(&self) -> Option<HashMap<String, Snapshot>>
}
```

### `Filter`
```rust
#[derive(Debug, Clone)]
pub struct Filter {
    /// Name of network interface
    pub device: String,
    /// Filtering rules
    /// BPF : https://biot.com/capstats/bpf.html
    pub rule: String,
    /// Whether the mode is immediately modeled, the default true
    /// https://www.tcpdump.org/manpages/pcap_set_immediate_mode.3pcap.html
    pub immediate_mode: bool,
}

/// Init filter, the default immediate_mode = true
Filter::new("eth0".to_string(), "tcp port 80".to_string());
/// or set immediate_mode field
Filter {
    device: "eth0".to_string(),
    rule: "tcp port 80".to_string(),
    immediate_mode: true,
}
```

### `Snapshot`
```rust
#[derive(Debug, Clone, Copy)]
pub struct Snapshot {
    /// The total byte after add_listener
    pub total: u64,
    /// The latest package of data byte
    pub len: u64,
    /// The latest package of data timestamp
    pub timestamp: u64,
}
```

### `get_device`
```rust
/// Get all network interface
pub fn get_device() -> Result<Vec<Device>, Error>
```

### `get_default_device`
```rust
/// Get default network interface
pub fn get_default_device() -> Result<Device, Error>
```


## Examples
[🖥 Get network interface device](./examples/device.rs)

[🚥 Statistical traffic](./examples/traffic.rs)

[🚄 Calculate network speed](./examples/speed.rs)



## Thanks
[pcap](https://github.com/rust-pcap/pcap)