#[doc = "Register `DIEP0CTL` reader"]
pub type R = crate::R<Diep0ctlSpec>;
#[doc = "Register `DIEP0CTL` writer"]
pub type W = crate::W<Diep0ctlSpec>;
#[doc = "Field `MPS` reader - Maximum packet size"]
pub type MpsR = crate::FieldReader;
#[doc = "Field `MPS` writer - Maximum packet size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type UsbactepR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `STALL` reader - Handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TXFIFO Number"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TXFIFO Number"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint Disable"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - End point enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - End point enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
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
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TXFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - End point enable"]
    #[inline(always)]
    pub fn epena(&self) -> EpenaR {
        EpenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MpsW<'_, Diep0ctlSpec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Diep0ctlSpec> {
        StallW::new(self, 21)
    }
    #[doc = "Bits 22:25 - TXFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TxfnumW<'_, Diep0ctlSpec> {
        TxfnumW::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<'_, Diep0ctlSpec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<'_, Diep0ctlSpec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EpdisW<'_, Diep0ctlSpec> {
        EpdisW::new(self, 30)
    }
    #[doc = "Bit 31 - End point enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<'_, Diep0ctlSpec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep0ctlSpec;
impl crate::RegisterSpec for Diep0ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0ctl::R`](R) reader structure"]
impl crate::Readable for Diep0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`diep0ctl::W`](W) writer structure"]
impl crate::Writable for Diep0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP0CTL to value 0x8000"]
impl crate::Resettable for Diep0ctlSpec {
    const RESET_VALUE: u32 = 0x8000;
}
