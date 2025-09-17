#[doc = "Register `DOEPMSK` reader"]
pub type R = crate::R<DoepmskSpec>;
#[doc = "Register `DOEPMSK` writer"]
pub type W = crate::W<DoepmskSpec>;
#[doc = "Field `XFERCOMPPLMSK` reader - Device OUT EP Common IRQ Mask Register"]
pub type XfercompplmskR = crate::BitReader;
#[doc = "Field `XFERCOMPPLMSK` writer - Device OUT EP Common IRQ Mask Register"]
pub type XfercompplmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLDMSK` reader - Endpoint Disabled Interrupt Mask"]
pub type EpdisbldmskR = crate::BitReader;
#[doc = "Field `EPDISBLDMSK` writer - Endpoint Disabled Interrupt Mask"]
pub type EpdisbldmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRMSK` reader - Ahb Error Mask"]
pub type AhberrmskR = crate::BitReader;
#[doc = "Field `AHBERRMSK` writer - Ahb Error Mask"]
pub type AhberrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPMSK` reader - Setup Phase Done Mask"]
pub type SetupmskR = crate::BitReader;
#[doc = "Field `SETUPMSK` writer - Setup Phase Done Mask"]
pub type SetupmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTKNEPDISMSK` reader - OUT Token recvd when EP Disabled Mask"]
pub type OuttknepdismskR = crate::BitReader;
#[doc = "Field `OUTTKNEPDISMSK` writer - OUT Token recvd when EP Disabled Mask"]
pub type OuttknepdismskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSERCVDMSK` reader - Status Phase Received Mask"]
pub type StsphsercvdmskR = crate::BitReader;
#[doc = "Field `STSPHSERCVDMSK` writer - Status Phase Received Mask"]
pub type StsphsercvdmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2BACKSETUP` reader - Back-to-Back SETUP Packets recvd Mask"]
pub type Back2backsetupR = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP` writer - Back-to-Back SETUP Packets recvd Mask"]
pub type Back2backsetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERRMSK` reader - OUT Packet Error Mask"]
pub type OutpkterrmskR = crate::BitReader;
#[doc = "Field `OUTPKTERRMSK` writer - OUT Packet Error Mask"]
pub type OutpkterrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERRMSK` reader - Babble Error Interrupt Mask"]
pub type BbleerrmskR = crate::BitReader;
#[doc = "Field `BBLEERRMSK` writer - Babble Error Interrupt Mask"]
pub type BbleerrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAK Interrupt Mask"]
pub type NakmskR = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK Interrupt Mask"]
pub type NakmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETMSK` reader - NYET Interrupt Mask"]
pub type NyetmskR = crate::BitReader;
#[doc = "Field `NYETMSK` writer - NYET Interrupt Mask"]
pub type NyetmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Device OUT EP Common IRQ Mask Register"]
    #[inline(always)]
    pub fn xfercompplmsk(&self) -> XfercompplmskR {
        XfercompplmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbldmsk(&self) -> EpdisbldmskR {
        EpdisbldmskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ahb Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AhberrmskR {
        AhberrmskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setup Phase Done Mask"]
    #[inline(always)]
    pub fn setupmsk(&self) -> SetupmskR {
        SetupmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token recvd when EP Disabled Mask"]
    #[inline(always)]
    pub fn outtknepdismsk(&self) -> OuttknepdismskR {
        OuttknepdismskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received Mask"]
    #[inline(always)]
    pub fn stsphsercvdmsk(&self) -> StsphsercvdmskR {
        StsphsercvdmskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets recvd Mask"]
    #[inline(always)]
    pub fn back2backsetup(&self) -> Back2backsetupR {
        Back2backsetupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    pub fn outpkterrmsk(&self) -> OutpkterrmskR {
        OutpkterrmskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Error Interrupt Mask"]
    #[inline(always)]
    pub fn bbleerrmsk(&self) -> BbleerrmskR {
        BbleerrmskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NakmskR {
        NakmskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET Interrupt Mask"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NyetmskR {
        NyetmskR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device OUT EP Common IRQ Mask Register"]
    #[inline(always)]
    pub fn xfercompplmsk(&mut self) -> XfercompplmskW<'_, DoepmskSpec> {
        XfercompplmskW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbldmsk(&mut self) -> EpdisbldmskW<'_, DoepmskSpec> {
        EpdisbldmskW::new(self, 1)
    }
    #[doc = "Bit 2 - Ahb Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&mut self) -> AhberrmskW<'_, DoepmskSpec> {
        AhberrmskW::new(self, 2)
    }
    #[doc = "Bit 3 - Setup Phase Done Mask"]
    #[inline(always)]
    pub fn setupmsk(&mut self) -> SetupmskW<'_, DoepmskSpec> {
        SetupmskW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT Token recvd when EP Disabled Mask"]
    #[inline(always)]
    pub fn outtknepdismsk(&mut self) -> OuttknepdismskW<'_, DoepmskSpec> {
        OuttknepdismskW::new(self, 4)
    }
    #[doc = "Bit 5 - Status Phase Received Mask"]
    #[inline(always)]
    pub fn stsphsercvdmsk(&mut self) -> StsphsercvdmskW<'_, DoepmskSpec> {
        StsphsercvdmskW::new(self, 5)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets recvd Mask"]
    #[inline(always)]
    pub fn back2backsetup(&mut self) -> Back2backsetupW<'_, DoepmskSpec> {
        Back2backsetupW::new(self, 6)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    pub fn outpkterrmsk(&mut self) -> OutpkterrmskW<'_, DoepmskSpec> {
        OutpkterrmskW::new(self, 8)
    }
    #[doc = "Bit 12 - Babble Error Interrupt Mask"]
    #[inline(always)]
    pub fn bbleerrmsk(&mut self) -> BbleerrmskW<'_, DoepmskSpec> {
        BbleerrmskW::new(self, 12)
    }
    #[doc = "Bit 13 - NAK Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NakmskW<'_, DoepmskSpec> {
        NakmskW::new(self, 13)
    }
    #[doc = "Bit 14 - NYET Interrupt Mask"]
    #[inline(always)]
    pub fn nyetmsk(&mut self) -> NyetmskW<'_, DoepmskSpec> {
        NyetmskW::new(self, 14)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoepmskSpec;
impl crate::RegisterSpec for DoepmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepmsk::R`](R) reader structure"]
impl crate::Readable for DoepmskSpec {}
#[doc = "`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure"]
impl crate::Writable for DoepmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DoepmskSpec {}
