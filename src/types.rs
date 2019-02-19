bitflags! {
    pub struct EventReg: u8 {
        const OpComplete = 0b00000001;
        const QueryErr   = 0b00000100;
        const DevSpecErr = 0b00001000;
        const ExecErr    = 0b00010000;
        const CmdErr     = 0b00100000;
        const PowerOn    = 0b10000000;
    }
}
