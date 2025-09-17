#[doc = "Register `DIEPMSK` reader"]
pub type R = crate::R<DiepmskSpec>;
#[doc = "Register `DIEPMSK` writer"]
pub type W = crate::W<DiepmskSpec>;
#[doc = "Field `XFERCOMPLMSK` reader - Transfer Completed Interrupt Mask"]
pub type XfercomplmskR = crate::BitReader;
#[doc = "Field `XFERCOMPLMSK` writer - Transfer Completed Interrupt Mask"]
pub type XfercomplmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLDMSK` reader - Endpoint Disabled Interrupt Mask"]
pub type EpdisbldmskR = crate::BitReader;
#[doc = "Field `EPDISBLDMSK` writer - Endpoint Disabled Interrupt Mask"]
pub type EpdisbldmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRMSK` reader - AHB Error Mask"]
pub type AhberrmskR = crate::BitReader;
#[doc = "Field `AHBERRMSK` writer - AHB Error Mask"]
pub type AhberrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTMSK` reader - Timout Condition Mask"]
pub type TimeoutmskR = crate::BitReader;
#[doc = "Field `TIMEOUTMSK` writer - Timout Condition Mask"]
pub type TimeoutmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMPMSK` reader - IN token recvd when FIFO mt irq mask"]
pub type IntkntxfempmskR = crate::BitReader;
#[doc = "Field `INTKNTXFEMPMSK` writer - IN token recvd when FIFO mt irq mask"]
pub type IntkntxfempmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNEPMISMSK` reader - IN Token recvd with EP Mismatch Mask"]
pub type IntknepmismskR = crate::BitReader;
#[doc = "Field `INTKNEPMISMSK` writer - IN Token recvd with EP Mismatch Mask"]
pub type IntknepmismskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNAKEFFMSK` reader - IN Endpoint NAK Effective Mask"]
pub type InepnakeffmskR = crate::BitReader;
#[doc = "Field `INEPNAKEFFMSK` writer - IN Endpoint NAK Effective Mask"]
pub type InepnakeffmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOUNDRNMSK` reader - FIFO Underrun Mask"]
pub type TxfifoundrnmskR = crate::BitReader;
#[doc = "Field `TXFIFOUNDRNMSK` writer - FIFO Underrun Mask"]
pub type TxfifoundrnmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAK interrupt Mask"]
pub type NakmskR = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK interrupt Mask"]
pub type NakmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XfercomplmskR {
        XfercomplmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbldmsk(&self) -> EpdisbldmskR {
        EpdisbldmskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AhberrmskR {
        AhberrmskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timout Condition Mask"]
    #[inline(always)]
    pub fn timeoutmsk(&self) -> TimeoutmskR {
        TimeoutmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token recvd when FIFO mt irq mask"]
    #[inline(always)]
    pub fn intkntxfempmsk(&self) -> IntkntxfempmskR {
        IntkntxfempmskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN Token recvd with EP Mismatch Mask"]
    #[inline(always)]
    pub fn intknepmismsk(&self) -> IntknepmismskR {
        IntknepmismskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    pub fn inepnakeffmsk(&self) -> InepnakeffmskR {
        InepnakeffmskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO Underrun Mask"]
    #[inline(always)]
    pub fn txfifoundrnmsk(&self) -> TxfifoundrnmskR {
        TxfifoundrnmskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NakmskR {
        NakmskR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&mut self) -> XfercomplmskW<'_, DiepmskSpec> {
        XfercomplmskW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbldmsk(&mut self) -> EpdisbldmskW<'_, DiepmskSpec> {
        EpdisbldmskW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&mut self) -> AhberrmskW<'_, DiepmskSpec> {
        AhberrmskW::new(self, 2)
    }
    #[doc = "Bit 3 - Timout Condition Mask"]
    #[inline(always)]
    pub fn timeoutmsk(&mut self) -> TimeoutmskW<'_, DiepmskSpec> {
        TimeoutmskW::new(self, 3)
    }
    #[doc = "Bit 4 - IN token recvd when FIFO mt irq mask"]
    #[inline(always)]
    pub fn intkntxfempmsk(&mut self) -> IntkntxfempmskW<'_, DiepmskSpec> {
        IntkntxfempmskW::new(self, 4)
    }
    #[doc = "Bit 5 - IN Token recvd with EP Mismatch Mask"]
    #[inline(always)]
    pub fn intknepmismsk(&mut self) -> IntknepmismskW<'_, DiepmskSpec> {
        IntknepmismskW::new(self, 5)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    pub fn inepnakeffmsk(&mut self) -> InepnakeffmskW<'_, DiepmskSpec> {
        InepnakeffmskW::new(self, 6)
    }
    #[doc = "Bit 8 - FIFO Underrun Mask"]
    #[inline(always)]
    pub fn txfifoundrnmsk(&mut self) -> TxfifoundrnmskW<'_, DiepmskSpec> {
        TxfifoundrnmskW::new(self, 8)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NakmskW<'_, DiepmskSpec> {
        NakmskW::new(self, 13)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepmskSpec;
impl crate::RegisterSpec for DiepmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepmsk::R`](R) reader structure"]
impl crate::Readable for DiepmskSpec {}
#[doc = "`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure"]
impl crate::Writable for DiepmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DiepmskSpec {}
