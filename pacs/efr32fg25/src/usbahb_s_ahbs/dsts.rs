#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DstsSpec>;
#[doc = "Field `SUSPSTS` reader - Suspend Status"]
pub type SuspstsR = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - Enumerated Speed"]
pub type EnumspdR = crate::FieldReader;
#[doc = "Field `ERRTICERR` reader - Erratic Error"]
pub type ErrticerrR = crate::BitReader;
#[doc = "Field `SOFFN` reader - Frame/u-frameNumber of the recvd SOF"]
pub type SoffnR = crate::FieldReader<u16>;
#[doc = "Field `DEVLNSTS` reader - logic level of USB data lines"]
pub type DevlnstsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SuspstsR {
        SuspstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> EnumspdR {
        EnumspdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errticerr(&self) -> ErrticerrR {
        ErrticerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame/u-frameNumber of the recvd SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SoffnR {
        SoffnR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:23 - logic level of USB data lines"]
    #[inline(always)]
    pub fn devlnsts(&self) -> DevlnstsR {
        DevlnstsR::new(((self.bits >> 22) & 3) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstsSpec;
impl crate::RegisterSpec for DstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DstsSpec {}
#[doc = "`reset()` method sets DSTS to value 0"]
impl crate::Resettable for DstsSpec {}
