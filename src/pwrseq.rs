#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Sequencer Control Register 0"]
    pub reg0: crate::Reg<reg0::REG0_SPEC>,
    #[doc = "0x04 - Power Sequencer Control Register 1"]
    pub reg1: crate::Reg<reg1::REG1_SPEC>,
    #[doc = "0x08 - Power Sequencer Control Register 2"]
    pub reg2: crate::Reg<reg2::REG2_SPEC>,
    #[doc = "0x0c - Power Sequencer Control Register 3"]
    pub reg3: crate::Reg<reg3::REG3_SPEC>,
    #[doc = "0x10 - Power Sequencer Control Register 4 (Internal Test Only)"]
    pub reg4: crate::Reg<reg4::REG4_SPEC>,
    #[doc = "0x14 - Power Sequencer Control Register 5 (Trim 0)"]
    pub reg5: crate::Reg<reg5::REG5_SPEC>,
    #[doc = "0x18 - Power Sequencer Control Register 6 (Trim 1)"]
    pub reg6: crate::Reg<reg6::REG6_SPEC>,
    #[doc = "0x1c - Power Sequencer Control Register 7"]
    pub reg7: crate::Reg<reg7::REG7_SPEC>,
    #[doc = "0x20 - Power Sequencer Flags"]
    pub flags: crate::Reg<flags::FLAGS_SPEC>,
    #[doc = "0x24 - Power Sequencer Flags Mask Register"]
    pub msk_flags: crate::Reg<msk_flags::MSK_FLAGS_SPEC>,
}
#[doc = "REG0 register accessor: an alias for `Reg<REG0_SPEC>`"]
pub type REG0 = crate::Reg<reg0::REG0_SPEC>;
#[doc = "Power Sequencer Control Register 0"]
pub mod reg0;
#[doc = "REG1 register accessor: an alias for `Reg<REG1_SPEC>`"]
pub type REG1 = crate::Reg<reg1::REG1_SPEC>;
#[doc = "Power Sequencer Control Register 1"]
pub mod reg1;
#[doc = "REG2 register accessor: an alias for `Reg<REG2_SPEC>`"]
pub type REG2 = crate::Reg<reg2::REG2_SPEC>;
#[doc = "Power Sequencer Control Register 2"]
pub mod reg2;
#[doc = "REG3 register accessor: an alias for `Reg<REG3_SPEC>`"]
pub type REG3 = crate::Reg<reg3::REG3_SPEC>;
#[doc = "Power Sequencer Control Register 3"]
pub mod reg3;
#[doc = "REG4 register accessor: an alias for `Reg<REG4_SPEC>`"]
pub type REG4 = crate::Reg<reg4::REG4_SPEC>;
#[doc = "Power Sequencer Control Register 4 (Internal Test Only)"]
pub mod reg4;
#[doc = "REG5 register accessor: an alias for `Reg<REG5_SPEC>`"]
pub type REG5 = crate::Reg<reg5::REG5_SPEC>;
#[doc = "Power Sequencer Control Register 5 (Trim 0)"]
pub mod reg5;
#[doc = "REG6 register accessor: an alias for `Reg<REG6_SPEC>`"]
pub type REG6 = crate::Reg<reg6::REG6_SPEC>;
#[doc = "Power Sequencer Control Register 6 (Trim 1)"]
pub mod reg6;
#[doc = "REG7 register accessor: an alias for `Reg<REG7_SPEC>`"]
pub type REG7 = crate::Reg<reg7::REG7_SPEC>;
#[doc = "Power Sequencer Control Register 7"]
pub mod reg7;
#[doc = "FLAGS register accessor: an alias for `Reg<FLAGS_SPEC>`"]
pub type FLAGS = crate::Reg<flags::FLAGS_SPEC>;
#[doc = "Power Sequencer Flags"]
pub mod flags;
#[doc = "MSK_FLAGS register accessor: an alias for `Reg<MSK_FLAGS_SPEC>`"]
pub type MSK_FLAGS = crate::Reg<msk_flags::MSK_FLAGS_SPEC>;
#[doc = "Power Sequencer Flags Mask Register"]
pub mod msk_flags;
