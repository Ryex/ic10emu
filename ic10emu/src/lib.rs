#[allow(dead_code)]
#[derive(Debug)]
pub struct VM {
    ip: u8,
    ra: f64,
    registers: [f64; 16], // r[0-15]
    sp: f64,
    stack: [f64; 512],
}

impl Default for VM {
    fn default() -> Self {
        VM {
            ip: 0,
            ra: 0.0,
            registers: [0.0; 16],
            sp: 0.0,
            stack: [0.0; 512],
        }
    }
    
}

impl VM {
    pub fn new() -> VM {
        VM::default()
    }
}
