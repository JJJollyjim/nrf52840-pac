#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1796usize],
    #[doc = "0x704 - Disable all ACL protection mechanisms for regions while in debug mode"]
    pub disableindebug: DISABLEINDEBUG,
    _reserved1: [u8; 248usize],
    #[doc = "0x800 - Unspecified"]
    pub acl: [ACL; 8],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct ACL { # [ doc = "0x00 - Description cluster[0]: Configure the word-aligned start address of region 0 to protect" ] pub addr : self :: acl :: ADDR , # [ doc = "0x04 - Description cluster[0]: Size of region to protect counting from address ACL[0].ADDR. Write '0' as no effect." ] pub size : self :: acl :: SIZE , # [ doc = "0x08 - Description cluster[0]: Access permissions for region 0 as defined by start address ACL[0].ADDR and size ACL[0].SIZE" ] pub perm : self :: acl :: PERM , # [ doc = "0x0c - Unspecified" ] pub unused0 : self :: acl :: UNUSED0 }
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod acl;
#[doc = "Disable all ACL protection mechanisms for regions while in debug mode"]
pub struct DISABLEINDEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable all ACL protection mechanisms for regions while in debug mode"]
pub mod disableindebug;
