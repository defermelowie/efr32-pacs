#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `CHNLRUNNING0` reader - Channel0 Running Status"]
pub type Chnlrunning0R = crate::BitReader;
#[doc = "Field `CHNLRUNNING1` reader - Channel1 Running Status"]
pub type Chnlrunning1R = crate::BitReader;
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockstatus {
    #[doc = "0: ETAMPDET registers are unlocked"]
    Unlocked = 0,
    #[doc = "1: ETAMPDET registers are locked"]
    Locked = 1,
}
impl From<Lockstatus> for bool {
    #[inline(always)]
    fn from(variant: Lockstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKSTATUS` reader - Lock Status"]
pub type LockstatusR = crate::BitReader<Lockstatus>;
impl LockstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockstatus {
        match self.bits {
            false => Lockstatus::Unlocked,
            true => Lockstatus::Locked,
        }
    }
    #[doc = "ETAMPDET registers are unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lockstatus::Unlocked
    }
    #[doc = "ETAMPDET registers are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lockstatus::Locked
    }
}
impl R {
    #[doc = "Bit 0 - Channel0 Running Status"]
    #[inline(always)]
    pub fn chnlrunning0(&self) -> Chnlrunning0R {
        Chnlrunning0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel1 Running Status"]
    #[inline(always)]
    pub fn chnlrunning1(&self) -> Chnlrunning1R {
        Chnlrunning1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock Status"]
    #[inline(always)]
    pub fn lockstatus(&self) -> LockstatusR {
        LockstatusR::new(((self.bits >> 31) & 1) != 0)
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
