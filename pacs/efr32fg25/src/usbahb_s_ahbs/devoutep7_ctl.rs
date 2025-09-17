#[doc = "Register `DEVOUTEP7_CTL` reader"]
pub type R = crate::R<Devoutep7CtlSpec>;
#[doc = "Register `DEVOUTEP7_CTL` writer"]
pub type W = crate::W<Devoutep7CtlSpec>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type UsbactepR = crate::BitReader;
#[doc = "Field `USBACTEP` writer - USB Active Endpoint"]
pub type UsbactepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPID` reader - Endpoint Data PID"]
pub type DpidR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNP` reader - Snoop Mode"]
pub type SnpR = crate::BitReader;
#[doc = "Field `SNP` writer - Snoop Mode"]
pub type SnpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD0PIDEF` writer - Set DATA0 PID, even fr"]
pub type Setd0pidefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD1PIDOF` writer - Set DATA1 PID, odd fr"]
pub type Setd1pidofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint Disable"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> UsbactepR {
        UsbactepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 20 - Snoop Mode"]
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
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MpsW<'_, Devoutep7CtlSpec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&mut self) -> UsbactepW<'_, Devoutep7CtlSpec> {
        UsbactepW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EptypeW<'_, Devoutep7CtlSpec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&mut self) -> SnpW<'_, Devoutep7CtlSpec> {
        SnpW::new(self, 20)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Devoutep7CtlSpec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<'_, Devoutep7CtlSpec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<'_, Devoutep7CtlSpec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - Set DATA0 PID, even fr"]
    #[inline(always)]
    pub fn setd0pidef(&mut self) -> Setd0pidefW<'_, Devoutep7CtlSpec> {
        Setd0pidefW::new(self, 28)
    }
    #[doc = "Bit 29 - Set DATA1 PID, odd fr"]
    #[inline(always)]
    pub fn setd1pidof(&mut self) -> Setd1pidofW<'_, Devoutep7CtlSpec> {
        Setd1pidofW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EpdisW<'_, Devoutep7CtlSpec> {
        EpdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<'_, Devoutep7CtlSpec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devoutep7_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devoutep7_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devoutep7CtlSpec;
impl crate::RegisterSpec for Devoutep7CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devoutep7_ctl::R`](R) reader structure"]
impl crate::Readable for Devoutep7CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`devoutep7_ctl::W`](W) writer structure"]
impl crate::Writable for Devoutep7CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVOUTEP7_CTL to value 0"]
impl crate::Resettable for Devoutep7CtlSpec {}
