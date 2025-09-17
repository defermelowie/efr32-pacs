#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RFFPLLRADIORDY` reader - Radio Clock Output Ready"]
pub type RffpllradiordyR = crate::BitReader;
#[doc = "Field `RFFPLLSYSRDY` reader - Digital System Clock Output Ready"]
pub type RffpllsysrdyR = crate::BitReader;
#[doc = "Field `ENS` reader - Enabled Status"]
pub type EnsR = crate::BitReader;
#[doc = "Field `HWREQRADIO` reader - PLL Radio Output Requested"]
pub type HwreqradioR = crate::BitReader;
#[doc = "Field `HWREQSYS` reader - PLL Digital System Output Requested"]
pub type HwreqsysR = crate::BitReader;
#[doc = "Configuration Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Configuration lock is unlocked"]
    Unlocked = 0,
    #[doc = "1: Configuration lock is locked"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Configuration Lock Status"]
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
    #[doc = "Configuration lock is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Configuration lock is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
impl R {
    #[doc = "Bit 0 - Radio Clock Output Ready"]
    #[inline(always)]
    pub fn rffpllradiordy(&self) -> RffpllradiordyR {
        RffpllradiordyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digital System Clock Output Ready"]
    #[inline(always)]
    pub fn rffpllsysrdy(&self) -> RffpllsysrdyR {
        RffpllsysrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Enabled Status"]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL Radio Output Requested"]
    #[inline(always)]
    pub fn hwreqradio(&self) -> HwreqradioR {
        HwreqradioR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL Digital System Output Requested"]
    #[inline(always)]
    pub fn hwreqsys(&self) -> HwreqsysR {
        HwreqsysR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Configuration Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
