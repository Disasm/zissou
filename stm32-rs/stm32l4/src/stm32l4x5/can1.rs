#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - master control register"]
    pub mcr: MCR,
    #[doc = "0x04 - master status register"]
    pub msr: MSR,
    #[doc = "0x08 - transmit status register"]
    pub tsr: TSR,
    #[doc = "0x0c - receive FIFO 0 register"]
    pub rfr: [RFR; 2],
    #[doc = "0x14 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x18 - interrupt enable register"]
    pub esr: ESR,
    #[doc = "0x1c - bit timing register"]
    pub btr: BTR,
    _reserved0: [u8; 352usize],
    #[doc = "0x180 - CAN Transmit cluster"]
    pub tx: [TX; 3],
    #[doc = "0x1b0 - CAN Receive cluster"]
    pub rx: [RX; 2],
    _reserved1: [u8; 112usize],
    #[doc = "0x240 - CAN Filter Bank cluster"]
    pub fb: [FB; 28],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - TX mailbox identifier register"]
    pub tir: self::tx::TIR,
    #[doc = "0x04 - mailbox data length control and time stamp register"]
    pub tdtr: self::tx::TDTR,
    #[doc = "0x08 - mailbox data low register"]
    pub tdlr: self::tx::TDLR,
    #[doc = "0x0c - mailbox data high register"]
    pub tdhr: self::tx::TDHR,
}
#[doc = r" Register block"]
#[doc = "CAN Transmit cluster"]
pub mod tx;
#[doc = r" Register block"]
#[repr(C)]
pub struct RX {
    #[doc = "0x00 - receive FIFO mailbox identifier register"]
    pub rir: self::rx::RIR,
    #[doc = "0x04 - mailbox data high register"]
    pub rdtr: self::rx::RDTR,
    #[doc = "0x08 - mailbox data high register"]
    pub rdlr: self::rx::RDLR,
    #[doc = "0x0c - receive FIFO mailbox data high register"]
    pub rdhr: self::rx::RDHR,
}
#[doc = r" Register block"]
#[doc = "CAN Receive cluster"]
pub mod rx;
#[doc = r" Register block"]
#[repr(C)]
pub struct FB {
    #[doc = "0x00 - Filter bank 0 register 1"]
    pub fr1: self::fb::FR1,
    #[doc = "0x04 - Filter bank 0 register 2"]
    pub fr2: self::fb::FR2,
}
#[doc = r" Register block"]
#[doc = "CAN Filter Bank cluster"]
pub mod fb;
#[doc = "master control register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "master control register"]
pub mod mcr;
#[doc = "master status register"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "master status register"]
pub mod msr;
#[doc = "transmit status register"]
pub struct TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "receive FIFO 0 register"]
pub struct RFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO 0 register"]
pub mod rfr;
#[doc = "interrupt enable register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "interrupt enable register"]
pub struct ESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod esr;
#[doc = "bit timing register"]
pub struct BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "bit timing register"]
pub mod btr;
