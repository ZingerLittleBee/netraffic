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

## 简介
`netraffic` 是一个 `rust` 库，提供**统计网络流量**的功能。

## 先决条件
### `Windows`
下载 [WinPcap](http://www.winpcap.org/install/default.htm) 开发者包, 添加 `/Lib` 或 `/Lib/x64` 目录到系统环境变量中

### `Linux`
安装 `libpcap`

Debian 系 Linux, 通过 `apt install libpcap-dev`

### `Mac OS X`
Mac OS X 默认安装 `libpcap`

## 安装
1. 获取最新版本 -> https://crates.io/crates/netraffic

2. 添加到 `Cargo.toml`
```toml
[dependencies]
netraffic = "0.1.0"
```

3. 用法
```rust
use std::{thread, time::Duration};
use netraffic::{Filter, Traffic};

fn main() {
    let mut traffic = Traffic::new();
    // rule look here: https://biot.com/capstats/bpf.html
    let rule1 = "port 443";
    let rule2 = "src host 127.0.0.1";
    // device: "any", just for linux mac
    traffic.add_listener(Filter::new("any".to_string(), rule1.to_string()));
    traffic.add_listener(Filter::new("any".to_string(), rule2.to_string()));
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
> 查看更多 [Examples](#examples)


## 总览
struct -> [Traffic](#traffic) · [Filter](#filter) · [Snapshot](#snapshot)

mod (`device`) -> [get_device](#get_device) · [get_default_device](#get_default_device)

## 文档
### `Traffic`
```rust
impl Traffic {
    /// 初始化
    pub fn new() -> Self
    /// 根据 filter 添加监听器
    pub fn add_listener(&mut self, filter: Filter)
    /// 通过 filter.rule 移除监听器
    pub fn remove_listener(&self, rule: String)
    /// 挂起 filter.rule 的监听器
    pub fn suspend_listener(&self, rule: String)
    /// 恢复 filter.rule 的监听器
    pub fn resume_listener(&self, rule: String)
    /// 阻塞线程直到获取到 Map<filter.rule, Snapshot> 键值对
    pub fn get_data(&self) -> HashMap<String, Snapshot>
    /// 尝试获 Map<filter.rule, Snapshot> 键值对
    /// 获取失败则返回 None
    pub fn try_get_data(&self) -> Option<HashMap<String, Snapshot>>
}
```

### `Filter`
```rust
#[derive(Debug, Clone)]
pub struct Filter {
    /// 网卡名称
    pub device: String,
    /// 过滤规则
    /// BPF 规则: https://biot.com/capstats/bpf.html
    pub rule: String,
    /// 是否立即模式, 默认为 true
    /// https://www.tcpdump.org/manpages/pcap_set_immediate_mode.3pcap.html
    pub immediate_mode: bool,
}

/// 初始化, 默认 immediate_mode = true
Filter::new("eth0".to_string(), "tcp port 80".to_string());
/// or 设置 immediate_mode 字段
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
    /// add_listener 之后的总 Byte
    pub total: u64,
    /// 最新一包数据的 Byte
    pub len: u64,
    /// 最新一包数据的时间戳
    pub timestamp: u64,
}
```

### `get_device`
```rust
/// 获取所有网卡
pub fn get_device() -> Result<Vec<Device>, Error>
```

### `get_default_device`
```rust
/// 获取默认网卡
pub fn get_default_device() -> Result<Device, Error>
```


## 例子
[🖥 获取网卡列表](./examples/device.rs)

[🚥 流量统计](./examples/traffic.rs)

[🚄 实时网速](./examples/speed.rs)



## 感谢
[pcap](https://github.com/rust-pcap/pcap)