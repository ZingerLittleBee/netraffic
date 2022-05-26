mod device;

use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Sender, TryRecvError},
        Arc, RwLock,
    },
    thread,
};

#[derive(Debug, Clone)]
pub struct Filter {
    pub device: String,
    pub rule: String,
    pub direction: Direction,
    pub immediate_mode: bool,
}

impl Filter {
    pub fn new(device: String, rule: String) -> Self {
        Filter {
            device,
            rule,
            direction: Direction::InOut,
            immediate_mode: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Snapshot {
    total: u64,
    len: u64,
    timestamp: u64,
}

impl Default for Snapshot {
    fn default() -> Self {
        Snapshot {
            total: 0,
            timestamp: 0,
            len: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Suspend,
    Resume,
    Stop,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    InOut,
    In,
    Out,
}

#[derive(Debug, Clone)]
pub struct Traffic {
    data_center: Arc<RwLock<HashMap<String, Snapshot>>>,
    signal: HashMap<String, Sender<Action>>,
}

impl Traffic {
    pub fn new() -> Self {
        Traffic {
            data_center: Arc::new(RwLock::new(HashMap::new())),
            signal: HashMap::new(),
        }
    }

    /// Add a new filter to the traffic data center.
    pub fn add_listener(&mut self, filter: Filter) {
        let (rule, tx) = self.resigster(filter);
        self.signal.insert(rule, tx);
    }

    /// Remove a filter from the traffic data center.
    pub fn remove_listener(&self, rule: String) {
        match self.signal.get(&rule) {
            Some(s) => {
                let _ = s.send(Action::Stop);
            }
            None => {}
        }
    }

    /// Suspend a listener by rule.
    pub fn suspend_listener(&self, rule: String) {
        match self.signal.get(&rule) {
            Some(s) => {
                let _ = s.send(Action::Suspend);
            }
            None => {}
        }
    }

    /// Resume a listener by rule.
    pub fn resume_listener(&self, rule: String) {
        match self.signal.get(&rule) {
            Some(s) => {
                let _ = s.send(Action::Resume);
            }
            None => {}
        }
    }

    /// Get the traffic snapshot, until Rwlock is free.
    pub fn get_data(&self) -> HashMap<String, Snapshot> {
        self.data_center.read().expect("get data failed").clone()
    }

    /// Try to get the traffic snapshot.
    /// if Rwlock is locked, return None.
    pub fn try_get_data(&self) -> Option<HashMap<String, Snapshot>> {
        match self.data_center.try_read() {
            Ok(dc) => Some(dc.clone()),
            Err(_) => None,
        }
    }

    fn resigster(&self, filter: Filter) -> (String, Sender<Action>) {
        let total = self.data_center.clone();
        // Control the thread
        let (tx, rx) = mpsc::channel::<Action>();
        let rule = filter.rule.clone();
        thread::spawn(move || {
            let mut cap = pcap::Capture::from_device(&filter.device[..])
                .unwrap()
                .immediate_mode(filter.immediate_mode)
                .open()
                .unwrap();
            // filter the packet by BPF syntax
            // BPF syntax, look at https://biot.com/capstats/bpf.html
            cap.filter(&filter.rule[..], true)
                .expect("set filter failed");
            // set capture direction
            match filter.direction {
                Direction::InOut => cap
                    .direction(pcap::Direction::InOut)
                    .expect("set direction failed"),
                Direction::In => cap
                    .direction(pcap::Direction::In)
                    .expect("set direction failed"),
                Direction::Out => cap
                    .direction(pcap::Direction::Out)
                    .expect("set direction failed"),
            };
            // (index, Snapshot)
            let mut i: (u32, Snapshot) = (0, Default::default());
            while let Ok(packet) = cap.next() {
                // Check channel signal
                match rx.try_recv() {
                    Ok(action) => match action {
                        Action::Suspend => {
                            println!("{} Suspend", &filter.rule[..]);
                            match rx.recv() {
                                Ok(inner_action) => match inner_action {
                                    Action::Stop => break,
                                    _ => println!("{} Resume", &filter.rule[..]),
                                },
                                Err(_) => break,
                            }
                        }
                        Action::Stop => {
                            println!("{} Stop", &filter.rule[..]);
                            break;
                        }
                        _ => {}
                    },
                    Err(TryRecvError::Disconnected) => {
                        println!("{} terminating", &filter.rule[..]);
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
                i.0 += 1;
                i.1.total += packet.header.len as u64;
                i.1.len = packet.header.len as u64;
                i.1.timestamp += packet.header.ts.tv_sec as u64;
                // Update data center
                // `i.0 % 2` to avoid the Rwlock is always locked.
                if i.0 % 2 == 0 {
                    let mut t = total.write().unwrap();
                    t.insert(String::from(&filter.rule[..]), i.1);
                }
            }
        });
        (rule, tx)
    }
}

#[cfg(test)]
mod test {
    use std::{process::Command, thread, time::Duration};

    use crate::{Filter, Traffic};

    #[test]
    fn it_works() {
        let mut traffic = Traffic::new();
        traffic.add_listener(Filter::new(String::from("any"), String::from("port 443")));
        Command::new("telnet")
            .args(["baidu.com", "443"])
            .output()
            .unwrap();

        thread::sleep(Duration::from_millis(100));
        assert!(traffic.get_data().len() > 0);
        assert!(traffic.get_data().get("port 443").unwrap().len > 0);
        assert!(traffic.get_data().get("port 443").unwrap().total > 0);
    }
}
