#[doc = "Register `DOEP0CTL` reader"]
pub type R = crate::R<Doep0ctlSpec>;
#[doc = "Register `DOEP0CTL` writer"]
pub type W = crate::W<Doep0ctlSpec>;
#[doc = "Field `MPS` reader - Max packet size"]
pub type MpsR = crate::FieldReader;
#[doc = "Field `USBACTEP` reader - US Active Endpoint"]
pub type UsbactepR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `SNP` reader - Snoop mode"]
pub type SnpR = crate::BitReader;
#[doc = "Field `SNP` writer - Snoop mode"]
pub type SnpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Max packet size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - US Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> UsbactepR {
        UsbactepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NakstsR {
        NakstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snp(&self) -> SnpR {
        SnpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EpenaR {
        EpenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snp(&mut self) -> SnpW<'_, Doep0ctlSpec> {
        SnpW::new(self, 20)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Doep0ctlSpec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<'_, Doep0ctlSpec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<'_, Doep0ctlSpec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<'_, Doep0ctlSpec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep0ctlSpec;
impl crate::RegisterSpec for Doep0ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep0ctl::R`](R) reader structure"]
impl crate::Readable for Doep0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`doep0ctl::W`](W) writer structure"]
impl crate::Writable for Doep0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP0CTL to value 0"]
impl crate::Resettable for Doep0ctlSpec {}
