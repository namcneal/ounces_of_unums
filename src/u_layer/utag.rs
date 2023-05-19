pub (crate) const LEFT_SIGN_MASK : u8 = 0b1000_0000;
pub (crate) const LEFT_UBIT_MASK : u8 = 0b0100_0000;
pub (crate) const LEFT_INF_MASK  : u8 = 0b0010_0000;
            const LEFT_NAN_MASK  : u8 = 0b0001_0000;

pub (crate) const RIGHT_SIGN_MASK : u8 = 0b1000_0000 >> 4;
pub (crate) const RIGHT_UBIT_MASK : u8 = 0b0100_0000 >> 4;
pub (crate) const RIGHT_INF_MASK  : u8 = 0b0010_0000 >> 4;
            const RIGHT_NAN_MASK  : u8 = 0b0001_0000 >> 4;

pub (crate) const NAN_MASK : u8 = LEFT_NAN_MASK | RIGHT_NAN_MASK;

#[derive(Debug, Copy, Clone)]
pub struct UTag(pub u8);

impl UTag{
    pub (crate) fn clear_left(&mut self){
        self.0 = self.0 << 4 >> 4
    }

    pub (crate) fn clear_right(&mut self){
        self.0 = self.0 >> 4 << 4
    }


}

