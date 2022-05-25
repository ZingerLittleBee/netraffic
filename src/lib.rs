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

fn resigster(
    total: Arc<RwLock<HashMap<String, Snapshot>>>,
    filter: Filter,
) -> (String, Sender<Action>) {
    let (tx, rx) = mpsc::channel::<Action>();
    let rule = filter.rule.clone();
    thread::spawn(move || {
        let mut cap = pcap::Capture::from_device(&filter.device[..])
            .unwrap()
            .immediate_mode(filter.immediate_mode)
            .open()
            .unwrap();
        cap.filter(&filter.rule[..], true)
            .expect("set filter failed");
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
        let mut i: (u32, Snapshot) = (0, Default::default());
        while let Ok(packet) = cap.next() {
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
            if i.0 % 2 == 0 {
                let mut t = total.write().unwrap();
                t.insert(String::from(&filter.rule[..]), i.1);
            }
        }
    });
    (rule, tx)
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

    pub fn add_listener(&mut self, filter: Filter) {
        let (rule, tx) = resigster(self.data_center.clone(), filter);
        self.signal.insert(rule, tx);
    }

    pub fn remove_listener(&self, rule: String) {
        match self.signal.get(&rule) {
            Some(s) => {
                let _ = s.send(Action::Stop);
            }
            None => {}
        }
    }

    pub fn suspend_listener(&self, rule: String) {
        match self.signal.get(&rule) {
            Some(s) => {
                let _ = s.send(Action::Suspend);
            }
            None => {}
        }
    }

    pub fn resume_listener(&self, rule: String) {
        match self.signal.get(&rule) {
            Some(s) => {
                let _ = s.send(Action::Resume);
            }
            None => {}
        }
    }

    pub fn get_data(&self) -> HashMap<String, Snapshot> {
        self.data_center.read().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use std::{process::Command, thread, time::Duration};

    use crate::{Filter, Traffic};

    #[test]
    fn it_works() {
        let mut traffic = Traffic::new();
        traffic.add_listener(Filter::new(String::from("en0"), String::from("port 443")));
        Command::new("telnet")
            .args(["baidu.com", "443"])
            .output()
            .unwrap();

        thread::sleep(Duration::from_millis(100));
        assert_eq!(traffic.get_data().len(), 1);
        assert!(traffic.get_data().get("port 443").unwrap().len > 0);
        assert!(traffic.get_data().get("port 443").unwrap().total > 0);
    }
}
