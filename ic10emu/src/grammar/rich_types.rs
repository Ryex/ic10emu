
#[derive(PartialEq, Debug)]
pub struct Register {
    pub indirection: u32,
    pub target: u8,
}

impl Register {
    pub fn from_str(rs: &str) -> Self {
        match &rs[..2] {
            "sp" => Register {
                indirection: 0,
                target: 16,
            },
            "ra" => Register {
                indirection: 0,
                target: 17,
            },
            _ => Register {
                indirection: rs[1..].chars().filter(|c| c == &'r').count() as u32,
                target: rs[1..].replace("r", "").parse().unwrap(),
            },
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct DeviceSpec {
    pub device: Device,
    pub channel: Option<u32>,
}

impl DeviceSpec {
    pub fn from_str(ds: &str) -> Self {
        let mut parts = ds.split(":");
        let d = parts.next().unwrap();
        let channel: Option<u32> = parts.next().map(|c| c.parse().unwrap());

        match d.as_bytes()[1] as char {
            'b' => DeviceSpec {
                device: Device::Db,
                channel,
            },
            '0'..='9' => DeviceSpec {
                device: Device::Numbered(d[1..2].parse().unwrap()),
                channel,
            },
            'r' => DeviceSpec {
                device: Device::Indirect(Register {
                    indirection: d[2..].chars().filter(|c| c == &'r').count() as u32,
                    target: d[2..].replace("r", "").parse().unwrap(),
                }),
                channel,
            },
            c => panic!("Bad second char in device spec '{}'", c),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Device {
    Db,
    Numbered(u8),
    Indirect(Register),
}

#[derive(PartialEq, Debug)]
pub struct HashString {
    pub string: String,
    pub hash: i32,
}

impl HashString {
    pub fn from_str(s: &str) -> Self {
        let crc = const_crc32::crc32(s.as_bytes()) as i32;
        HashString {
            string: s.to_string(),
            hash: crc,
        }
    }
}
