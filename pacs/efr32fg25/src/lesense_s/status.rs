#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RESFIFOV` reader - Result fifo valid"]
pub type ResfifovR = crate::BitReader;
#[doc = "Field `RESFIFOFULL` reader - Result fifo full"]
pub type ResfifofullR = crate::BitReader;
#[doc = "Field `SCANACTIVE` reader - LESENSE scan active"]
pub type ScanactiveR = crate::BitReader;
#[doc = "Field `RUNNING` reader - LESENSE periodic counter running"]
pub type RunningR = crate::BitReader;
#[doc = "Field `READBUSY` reader - FIFO Read Busy"]
pub type ReadbusyR = crate::BitReader;
#[doc = "Field `FLUSHING` reader - FIFO Flushing"]
pub type FlushingR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Result fifo valid"]
    #[inline(always)]
    pub fn resfifov(&self) -> ResfifovR {
        ResfifovR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result fifo full"]
    #[inline(always)]
    pub fn resfifofull(&self) -> ResfifofullR {
        ResfifofullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LESENSE scan active"]
    #[inline(always)]
    pub fn scanactive(&self) -> ScanactiveR {
        ScanactiveR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LESENSE periodic counter running"]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO Read Busy"]
    #[inline(always)]
    pub fn readbusy(&self) -> ReadbusyR {
        ReadbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO Flushing"]
    #[inline(always)]
    pub fn flushing(&self) -> FlushingR {
        FlushingR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
