#[doc = "Register `GHWCFG3` reader"]
pub type R = crate::R<Ghwcfg3Spec>;
#[doc = "Field `XFERSIZEWIDTH` reader - Width of Transfer Size Counters"]
pub type XfersizewidthR = crate::FieldReader;
#[doc = "Field `PKTSIZEWIDTH` reader - Width of Packet Size Counters"]
pub type PktsizewidthR = crate::FieldReader;
#[doc = "Field `OTGEN` reader - OTG Function Enabled"]
pub type OtgenR = crate::BitReader;
#[doc = "Field `I2CINTSEL` reader - I2C Selection"]
pub type I2cintselR = crate::BitReader;
#[doc = "Field `VNDCTLSUPT` reader - Vendor Control Interface Support"]
pub type VndctlsuptR = crate::BitReader;
#[doc = "Field `OPTFEATURE` reader - Optional Features Removed"]
pub type OptfeatureR = crate::BitReader;
#[doc = "Field `RSTTYPE` reader - Reset Style For Clocked always"]
pub type RsttypeR = crate::BitReader;
#[doc = "Field `ADPSUPPORT` reader - New BitField"]
pub type AdpsupportR = crate::BitReader;
#[doc = "Field `HSICMODE` reader - New BitField"]
pub type HsicmodeR = crate::BitReader;
#[doc = "Field `BCSUPPORT` reader - New BitField"]
pub type BcsupportR = crate::BitReader;
#[doc = "Field `LPMMODE` reader - New BitField"]
pub type LpmmodeR = crate::BitReader;
#[doc = "Field `DFIFODEPTH` reader - FIFO depth"]
pub type DfifodepthR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Width of Transfer Size Counters"]
    #[inline(always)]
    pub fn xfersizewidth(&self) -> XfersizewidthR {
        XfersizewidthR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Width of Packet Size Counters"]
    #[inline(always)]
    pub fn pktsizewidth(&self) -> PktsizewidthR {
        PktsizewidthR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OTG Function Enabled"]
    #[inline(always)]
    pub fn otgen(&self) -> OtgenR {
        OtgenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Selection"]
    #[inline(always)]
    pub fn i2cintsel(&self) -> I2cintselR {
        I2cintselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Vendor Control Interface Support"]
    #[inline(always)]
    pub fn vndctlsupt(&self) -> VndctlsuptR {
        VndctlsuptR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Optional Features Removed"]
    #[inline(always)]
    pub fn optfeature(&self) -> OptfeatureR {
        OptfeatureR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset Style For Clocked always"]
    #[inline(always)]
    pub fn rsttype(&self) -> RsttypeR {
        RsttypeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - New BitField"]
    #[inline(always)]
    pub fn adpsupport(&self) -> AdpsupportR {
        AdpsupportR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - New BitField"]
    #[inline(always)]
    pub fn hsicmode(&self) -> HsicmodeR {
        HsicmodeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - New BitField"]
    #[inline(always)]
    pub fn bcsupport(&self) -> BcsupportR {
        BcsupportR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New BitField"]
    #[inline(always)]
    pub fn lpmmode(&self) -> LpmmodeR {
        LpmmodeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - FIFO depth"]
    #[inline(always)]
    pub fn dfifodepth(&self) -> DfifodepthR {
        DfifodepthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwcfg3Spec;
impl crate::RegisterSpec for Ghwcfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwcfg3::R`](R) reader structure"]
impl crate::Readable for Ghwcfg3Spec {}
#[doc = "`reset()` method sets GHWCFG3 to value 0x02ec_0468"]
impl crate::Resettable for Ghwcfg3Spec {
    const RESET_VALUE: u32 = 0x02ec_0468;
}
