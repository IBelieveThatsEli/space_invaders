bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[allow(dead_code)]
    pub struct KeyMods: u32 {
        const SHIFT = 0x0001;
        const CONTROL = 0x0002;
        const ALT = 0x0004;
        const SUPER = 0x0008;
        const CAPS_LOCK = 0x0010;
        const NUM_LOCK = 0x0020;
    }
}
