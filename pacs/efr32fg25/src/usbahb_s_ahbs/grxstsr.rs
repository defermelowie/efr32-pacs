#[doc = "Register `GRXSTSR` reader"]
pub type R = crate::R<GrxstsrSpec>;
#[doc = "Field `EPNUM` reader - End Point Num"]
pub type EpnumR = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte Count"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet Status"]
pub type PktstsR = crate::FieldReader;
#[doc = "Field `FN` reader - Frame Number"]
pub type FnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - End Point Num"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PktstsR {
        PktstsR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxstsrSpec;
impl crate::RegisterSpec for GrxstsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr::R`](R) reader structure"]
impl crate::Readable for GrxstsrSpec {}
#[doc = "`reset()` method sets GRXSTSR to value 0"]
impl crate::Resettable for GrxstsrSpec {}
