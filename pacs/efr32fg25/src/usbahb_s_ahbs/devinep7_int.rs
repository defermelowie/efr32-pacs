#[doc = "Register `DEVINEP7_INT` reader"]
pub type R = crate::R<Devinep7IntSpec>;
#[doc = "Register `DEVINEP7_INT` writer"]
pub type W = crate::W<Devinep7IntSpec>;
#[doc = "Field `XFERCOMPL` reader - Transfer Completed IRQ"]
pub type XfercomplR = crate::BitReader;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed IRQ"]
pub type XfercomplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLD` reader - Endpoint Disabled IRQ"]
pub type EpdisbldR = crate::BitReader;
#[doc = "Field `EPDISBLD` writer - Endpoint Disabled IRQ"]
pub type EpdisbldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB Error"]
pub type AhberrR = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB Error"]
pub type AhberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - COndition"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - COndition"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMP` reader - IN Token recvd When TxFIFO is Empty"]
pub type IntkntxfempR = crate::BitReader;
#[doc = "Field `INTKNTXFEMP` writer - IN Token recvd When TxFIFO is Empty"]
pub type IntkntxfempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNEPMIS` reader - IN Token recvd with EP Mismatch"]
pub type IntknepmisR = crate::BitReader;
#[doc = "Field `INTKNEPMIS` writer - IN Token recvd with EP Mismatch"]
pub type IntknepmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNAKEFF` reader - IN Endpoint NAK Effective"]
pub type InepnakeffR = crate::BitReader;
#[doc = "Field `INEPNAKEFF` writer - IN Endpoint NAK Effective"]
pub type InepnakeffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEMP` reader - TX FIFO Empty"]
pub type TxfempR = crate::BitReader;
#[doc = "Field `TXFIFOUNDRN` reader - FIFO Underrun"]
pub type TxfifoundrnR = crate::BitReader;
#[doc = "Field `TXFIFOUNDRN` writer - FIFO Underrun"]
pub type TxfifoundrnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINTR` reader - BNA (Buffer Not Available) IRQ"]
pub type BnaintrR = crate::BitReader;
#[doc = "Field `BNAINTR` writer - BNA (Buffer Not Available) IRQ"]
pub type BnaintrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS` reader - Packet Drop Status"]
pub type PktdrpstsR = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - Packet Drop Status"]
pub type PktdrpstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERR` reader - Babble Interrupt"]
pub type BbleerrR = crate::BitReader;
#[doc = "Field `BBLEERR` writer - Babble Interrupt"]
pub type BbleerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINTRPT` reader - NAK Interrupt"]
pub type NakintrptR = crate::BitReader;
#[doc = "Field `NAKINTRPT` writer - NAK Interrupt"]
pub type NakintrptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETINTRPT` reader - NYET Interrupt"]
pub type NyetintrptR = crate::BitReader;
#[doc = "Field `NYETINTRPT` writer - NYET Interrupt"]
pub type NyetintrptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed IRQ"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XfercomplR {
        XfercomplR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled IRQ"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EpdisbldR {
        EpdisbldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AhberrR {
        AhberrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COndition"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token recvd When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkntxfemp(&self) -> IntkntxfempR {
        IntkntxfempR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN Token recvd with EP Mismatch"]
    #[inline(always)]
    pub fn intknepmis(&self) -> IntknepmisR {
        IntknepmisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnakeff(&self) -> InepnakeffR {
        InepnakeffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Empty"]
    #[inline(always)]
    pub fn txfemp(&self) -> TxfempR {
        TxfempR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO Underrun"]
    #[inline(always)]
    pub fn txfifoundrn(&self) -> TxfifoundrnR {
        TxfifoundrnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) IRQ"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BnaintrR {
        BnaintrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PktdrpstsR {
        PktdrpstsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&self) -> BbleerrR {
        BbleerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NakintrptR {
        NakintrptR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET Interrupt"]
    #[inline(always)]
    pub fn nyetintrpt(&self) -> NyetintrptR {
        NyetintrptR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed IRQ"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XfercomplW<'_, Devinep7IntSpec> {
        XfercomplW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled IRQ"]
    #[inline(always)]
    pub fn epdisbld(&mut self) -> EpdisbldW<'_, Devinep7IntSpec> {
        EpdisbldW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AhberrW<'_, Devinep7IntSpec> {
        AhberrW::new(self, 2)
    }
    #[doc = "Bit 3 - COndition"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, Devinep7IntSpec> {
        TimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token recvd When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkntxfemp(&mut self) -> IntkntxfempW<'_, Devinep7IntSpec> {
        IntkntxfempW::new(self, 4)
    }
    #[doc = "Bit 5 - IN Token recvd with EP Mismatch"]
    #[inline(always)]
    pub fn intknepmis(&mut self) -> IntknepmisW<'_, Devinep7IntSpec> {
        IntknepmisW::new(self, 5)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnakeff(&mut self) -> InepnakeffW<'_, Devinep7IntSpec> {
        InepnakeffW::new(self, 6)
    }
    #[doc = "Bit 8 - FIFO Underrun"]
    #[inline(always)]
    pub fn txfifoundrn(&mut self) -> TxfifoundrnW<'_, Devinep7IntSpec> {
        TxfifoundrnW::new(self, 8)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) IRQ"]
    #[inline(always)]
    pub fn bnaintr(&mut self) -> BnaintrW<'_, Devinep7IntSpec> {
        BnaintrW::new(self, 9)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PktdrpstsW<'_, Devinep7IntSpec> {
        PktdrpstsW::new(self, 11)
    }
    #[doc = "Bit 12 - Babble Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&mut self) -> BbleerrW<'_, Devinep7IntSpec> {
        BbleerrW::new(self, 12)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&mut self) -> NakintrptW<'_, Devinep7IntSpec> {
        NakintrptW::new(self, 13)
    }
    #[doc = "Bit 14 - NYET Interrupt"]
    #[inline(always)]
    pub fn nyetintrpt(&mut self) -> NyetintrptW<'_, Devinep7IntSpec> {
        NyetintrptW::new(self, 14)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devinep7_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinep7_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devinep7IntSpec;
impl crate::RegisterSpec for Devinep7IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinep7_int::R`](R) reader structure"]
impl crate::Readable for Devinep7IntSpec {}
#[doc = "`write(|w| ..)` method takes [`devinep7_int::W`](W) writer structure"]
impl crate::Writable for Devinep7IntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVINEP7_INT to value 0x80"]
impl crate::Resettable for Devinep7IntSpec {
    const RESET_VALUE: u32 = 0x80;
}
