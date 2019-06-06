use enum_map::{Enum};

bitflags! {
    pub struct EventReg: u8 {
        const OP_COMPLETE  = 0b00000001;
        const QUERY_ERR    = 0b00000100;
        const DEV_SPEC_ERR = 0b00001000;
        const EXEC_ERR     = 0b00010000;
        const CMD_ERR      = 0b00100000;
        const POWER_ON     = 0b10000000;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Enum)]
pub enum ChannelNo {
    Ch1 = 1,
    Ch2 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TriggerSource {
    Immediate,
    External,
    Bus,
}
