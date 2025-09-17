#[doc = "Register `DEVINEP5_CTL` reader"]
pub type R = crate::R<Devinep5CtlSpec>;
#[doc = "Register `DEVINEP5_CTL` writer"]
pub type W = crate::W<Devinep5CtlSpec>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBACTEP` reader - USB ACtive Endpoint"]
pub type UsbactepR = crate::BitReader;
#[doc = "Field `USBACTEP` writer - USB ACtive Endpoint"]
pub type UsbactepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPID` reader - Endpoint Data PID, EO_FrNum"]
pub type DpidR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK STatus"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STALL` reader - Handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO Number"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO Number"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD0PIDEF` writer - Set DATA0 PID / Even Frame"]
pub type Setd0pidefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD1PIDOF` writer - Set DATA1 PID / odd Frame"]
pub type Setd1pidofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint Disable"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - Endpoint enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB ACtive Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> UsbactepR {
        UsbactepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Data PID, EO_FrNum"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK STatus"]
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
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epena(&self) -> EpenaR {
        EpenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MpsW<'_, Devinep5CtlSpec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bit 15 - USB ACtive Endpoint"]
    #[inline(always)]
    pub fn usbactep(&mut self) -> UsbactepW<'_, Devinep5CtlSpec> {
        UsbactepW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EptypeW<'_, Devinep5CtlSpec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Devinep5CtlSpec> {
        StallW::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TxfnumW<'_, Devinep5CtlSpec> {
        TxfnumW::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<'_, Devinep5CtlSpec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<'_, Devinep5CtlSpec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - Set DATA0 PID / Even Frame"]
    #[inline(always)]
    pub fn setd0pidef(&mut self) -> Setd0pidefW<'_, Devinep5CtlSpec> {
        Setd0pidefW::new(self, 28)
    }
    #[doc = "Bit 29 - Set DATA1 PID / odd Frame"]
    #[inline(always)]
    pub fn setd1pidof(&mut self) -> Setd1pidofW<'_, Devinep5CtlSpec> {
        Setd1pidofW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EpdisW<'_, Devinep5CtlSpec> {
        EpdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<'_, Devinep5CtlSpec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devinep5_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinep5_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devinep5CtlSpec;
impl crate::RegisterSpec for Devinep5CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinep5_ctl::R`](R) reader structure"]
impl crate::Readable for Devinep5CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`devinep5_ctl::W`](W) writer structure"]
impl crate::Writable for Devinep5CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVINEP5_CTL to value 0"]
impl crate::Resettable for Devinep5CtlSpec {}
