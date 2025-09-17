#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `CHNLSTART0` reader - Synchronizer busy status"]
pub type Chnlstart0R = crate::BitReader;
#[doc = "Field `CHNLSTOP0` reader - Synchronizer busy status"]
pub type Chnlstop0R = crate::BitReader;
#[doc = "Field `CHNLLOAD0` reader - Synchronizer busy status"]
pub type Chnlload0R = crate::BitReader;
#[doc = "Field `CHNLSTART1` reader - Synchronizer busy status"]
pub type Chnlstart1R = crate::BitReader;
#[doc = "Field `CHNLSTOP1` reader - Synchronizer busy status"]
pub type Chnlstop1R = crate::BitReader;
#[doc = "Field `CHNLLOAD1` reader - Synchronizer busy status"]
pub type Chnlload1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Synchronizer busy status"]
    #[inline(always)]
    pub fn chnlstart0(&self) -> Chnlstart0R {
        Chnlstart0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronizer busy status"]
    #[inline(always)]
    pub fn chnlstop0(&self) -> Chnlstop0R {
        Chnlstop0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronizer busy status"]
    #[inline(always)]
    pub fn chnlload0(&self) -> Chnlload0R {
        Chnlload0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronizer busy status"]
    #[inline(always)]
    pub fn chnlstart1(&self) -> Chnlstart1R {
        Chnlstart1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronizer busy status"]
    #[inline(always)]
    pub fn chnlstop1(&self) -> Chnlstop1R {
        Chnlstop1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronizer busy status"]
    #[inline(always)]
    pub fn chnlload1(&self) -> Chnlload1R {
        Chnlload1R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
