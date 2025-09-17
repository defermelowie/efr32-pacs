#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `PLLRDY` reader - USBPLL Ready"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `PLLLOCK` reader - USBPLL is locked"]
pub type PlllockR = crate::BitReader;
#[doc = "Field `SYNCBUSY` reader - Sync Busy"]
pub type SyncbusyR = crate::BitReader;
#[doc = "Locks out registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Configuration is unlocked"]
    Unlocked = 0,
    #[doc = "1: Configuration is locked"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Locks out registers"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "Configuration is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Configuration is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
impl R {
    #[doc = "Bit 16 - USBPLL Ready"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USBPLL is locked"]
    #[inline(always)]
    pub fn plllock(&self) -> PlllockR {
        PlllockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 30 - Sync Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Locks out registers"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
