use crate::{
    KsFc,
    types::{ChannelNo},
};

// Configure commands
impl KsFc {
    /// `CONFigure:FREQuency @<channel>`
    pub fn configure_frequency(&mut self, cn: ChannelNo) -> crate::Result<()> {
        self.send(format!("CONF:FREQ (@{})", cn as u8).as_bytes())
    }
}
