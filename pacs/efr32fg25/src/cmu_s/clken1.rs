#[doc = "Register `CLKEN1` reader"]
pub type R = crate::R<Clken1Spec>;
#[doc = "Register `CLKEN1` writer"]
pub type W = crate::W<Clken1Spec>;
#[doc = "Field `HOSTMAILBOX` reader - Enable Bus Clock"]
pub type HostmailboxR = crate::BitReader;
#[doc = "Field `HOSTMAILBOX` writer - Enable Bus Clock"]
pub type HostmailboxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFFPLL0` reader - Enable Bus Clock"]
pub type Rffpll0R = crate::BitReader;
#[doc = "Field `RFFPLL0` writer - Enable Bus Clock"]
pub type Rffpll0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEMAILBOXHOST` reader - Enable Bus Clock"]
pub type SemailboxhostR = crate::BitReader;
#[doc = "Field `SEMAILBOXHOST` writer - Enable Bus Clock"]
pub type SemailboxhostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFC` reader - Enable Bus Clock"]
pub type BufcR = crate::BitReader;
#[doc = "Field `BUFC` writer - Enable Bus Clock"]
pub type BufcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EUSART3` reader - Enable Bus Clock"]
pub type Eusart3R = crate::BitReader;
#[doc = "Field `EUSART3` writer - Enable Bus Clock"]
pub type Eusart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EUSART4` reader - Enable Bus Clock"]
pub type Eusart4R = crate::BitReader;
#[doc = "Field `EUSART4` writer - Enable Bus Clock"]
pub type Eusart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMU` reader - Enable Bus Clock"]
pub type SmuR = crate::BitReader;
#[doc = "Field `SMU` writer - Enable Bus Clock"]
pub type SmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE0` reader - Enable Bus Clock"]
pub type Icache0R = crate::BitReader;
#[doc = "Field `ICACHE0` writer - Enable Bus Clock"]
pub type Icache0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSC` reader - Enable Bus Clock"]
pub type MscR = crate::BitReader;
#[doc = "Field `MSC` writer - Enable Bus Clock"]
pub type MscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG1` reader - Enable Bus Clock"]
pub type Wdog1R = crate::BitReader;
#[doc = "Field `WDOG1` writer - Enable Bus Clock"]
pub type Wdog1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP0` reader - Enable Bus Clock"]
pub type Acmp0R = crate::BitReader;
#[doc = "Field `ACMP0` writer - Enable Bus Clock"]
pub type Acmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1` reader - Enable Bus Clock"]
pub type Acmp1R = crate::BitReader;
#[doc = "Field `ACMP1` writer - Enable Bus Clock"]
pub type Acmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDAC0` reader - Enable Bus Clock"]
pub type Vdac0R = crate::BitReader;
#[doc = "Field `VDAC0` writer - Enable Bus Clock"]
pub type Vdac0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0` reader - Enable Bus Clock"]
pub type Pcnt0R = crate::BitReader;
#[doc = "Field `PCNT0` writer - Enable Bus Clock"]
pub type Pcnt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EUSART0` reader - Enable Bus Clock"]
pub type Eusart0R = crate::BitReader;
#[doc = "Field `EUSART0` writer - Enable Bus Clock"]
pub type Eusart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EUSART1` reader - Enable Bus Clock"]
pub type Eusart1R = crate::BitReader;
#[doc = "Field `EUSART1` writer - Enable Bus Clock"]
pub type Eusart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EUSART2` reader - Enable Bus Clock"]
pub type Eusart2R = crate::BitReader;
#[doc = "Field `EUSART2` writer - Enable Bus Clock"]
pub type Eusart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMEM` reader - Enable Bus Clock"]
pub type DmemR = crate::BitReader;
#[doc = "Field `DMEM` writer - Enable Bus Clock"]
pub type DmemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETAMPDET` reader - Enable Bus Clock"]
pub type EtampdetR = crate::BitReader;
#[doc = "Field `ETAMPDET` writer - Enable Bus Clock"]
pub type EtampdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER6` reader - Enable Bus Clock"]
pub type Timer6R = crate::BitReader;
#[doc = "Field `TIMER6` writer - Enable Bus Clock"]
pub type Timer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER7` reader - Enable Bus Clock"]
pub type Timer7R = crate::BitReader;
#[doc = "Field `TIMER7` writer - Enable Bus Clock"]
pub type Timer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - Enable Bus Clock"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - Enable Bus Clock"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Enable Bus Clock"]
    #[inline(always)]
    pub fn hostmailbox(&self) -> HostmailboxR {
        HostmailboxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rffpll0(&self) -> Rffpll0R {
        Rffpll0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Bus Clock"]
    #[inline(always)]
    pub fn semailboxhost(&self) -> SemailboxhostR {
        SemailboxhostR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Bus Clock"]
    #[inline(always)]
    pub fn bufc(&self) -> BufcR {
        BufcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart3(&self) -> Eusart3R {
        Eusart3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart4(&self) -> Eusart4R {
        Eusart4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Bus Clock"]
    #[inline(always)]
    pub fn smu(&self) -> SmuR {
        SmuR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    pub fn icache0(&self) -> Icache0R {
        Icache0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    pub fn msc(&self) -> MscR {
        MscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    pub fn wdog1(&self) -> Wdog1R {
        Wdog1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    pub fn acmp0(&self) -> Acmp0R {
        Acmp0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Bus Clock"]
    #[inline(always)]
    pub fn acmp1(&self) -> Acmp1R {
        Acmp1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Bus Clock"]
    #[inline(always)]
    pub fn vdac0(&self) -> Vdac0R {
        Vdac0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Bus Clock"]
    #[inline(always)]
    pub fn pcnt0(&self) -> Pcnt0R {
        Pcnt0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart0(&self) -> Eusart0R {
        Eusart0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart1(&self) -> Eusart1R {
        Eusart1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart2(&self) -> Eusart2R {
        Eusart2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Bus Clock"]
    #[inline(always)]
    pub fn dmem(&self) -> DmemR {
        DmemR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Bus Clock"]
    #[inline(always)]
    pub fn etampdet(&self) -> EtampdetR {
        EtampdetR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer6(&self) -> Timer6R {
        Timer6R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer7(&self) -> Timer7R {
        Timer7R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Bus Clock"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable Bus Clock"]
    #[inline(always)]
    pub fn hostmailbox(&mut self) -> HostmailboxW<'_, Clken1Spec> {
        HostmailboxW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rffpll0(&mut self) -> Rffpll0W<'_, Clken1Spec> {
        Rffpll0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Bus Clock"]
    #[inline(always)]
    pub fn semailboxhost(&mut self) -> SemailboxhostW<'_, Clken1Spec> {
        SemailboxhostW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Bus Clock"]
    #[inline(always)]
    pub fn bufc(&mut self) -> BufcW<'_, Clken1Spec> {
        BufcW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart3(&mut self) -> Eusart3W<'_, Clken1Spec> {
        Eusart3W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart4(&mut self) -> Eusart4W<'_, Clken1Spec> {
        Eusart4W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Bus Clock"]
    #[inline(always)]
    pub fn smu(&mut self) -> SmuW<'_, Clken1Spec> {
        SmuW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    pub fn icache0(&mut self) -> Icache0W<'_, Clken1Spec> {
        Icache0W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    pub fn msc(&mut self) -> MscW<'_, Clken1Spec> {
        MscW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    pub fn wdog1(&mut self) -> Wdog1W<'_, Clken1Spec> {
        Wdog1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> Acmp0W<'_, Clken1Spec> {
        Acmp0W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Bus Clock"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> Acmp1W<'_, Clken1Spec> {
        Acmp1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Bus Clock"]
    #[inline(always)]
    pub fn vdac0(&mut self) -> Vdac0W<'_, Clken1Spec> {
        Vdac0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Bus Clock"]
    #[inline(always)]
    pub fn pcnt0(&mut self) -> Pcnt0W<'_, Clken1Spec> {
        Pcnt0W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart0(&mut self) -> Eusart0W<'_, Clken1Spec> {
        Eusart0W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart1(&mut self) -> Eusart1W<'_, Clken1Spec> {
        Eusart1W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Bus Clock"]
    #[inline(always)]
    pub fn eusart2(&mut self) -> Eusart2W<'_, Clken1Spec> {
        Eusart2W::new(self, 24)
    }
    #[doc = "Bit 27 - Enable Bus Clock"]
    #[inline(always)]
    pub fn dmem(&mut self) -> DmemW<'_, Clken1Spec> {
        DmemW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Bus Clock"]
    #[inline(always)]
    pub fn etampdet(&mut self) -> EtampdetW<'_, Clken1Spec> {
        EtampdetW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer6(&mut self) -> Timer6W<'_, Clken1Spec> {
        Timer6W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer7(&mut self) -> Timer7W<'_, Clken1Spec> {
        Timer7W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Bus Clock"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, Clken1Spec> {
        UsbW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`clken1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clken1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clken1Spec;
impl crate::RegisterSpec for Clken1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clken1::R`](R) reader structure"]
impl crate::Readable for Clken1Spec {}
#[doc = "`write(|w| ..)` method takes [`clken1::W`](W) writer structure"]
impl crate::Writable for Clken1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKEN1 to value 0"]
impl crate::Resettable for Clken1Spec {}
