#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `FORCEEN` reader - Force PLL Enable"]
pub type ForceenR = crate::BitReader;
#[doc = "Field `FORCEEN` writer - Force PLL Enable"]
pub type ForceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISONDEMAND` reader - Disable On-demand request"]
pub type DisondemandR = crate::BitReader;
#[doc = "Field `DISONDEMAND` writer - Disable On-demand request"]
pub type DisondemandW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Force PLL Enable"]
    #[inline(always)]
    pub fn forceen(&self) -> ForceenR {
        ForceenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable On-demand request"]
    #[inline(always)]
    pub fn disondemand(&self) -> DisondemandR {
        DisondemandR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Force PLL Enable"]
    #[inline(always)]
    pub fn forceen(&mut self) -> ForceenW<'_, CtrlSpec> {
        ForceenW::new(self, 1)
    }
    #[doc = "Bit 8 - Disable On-demand request"]
    #[inline(always)]
    pub fn disondemand(&mut self) -> DisondemandW<'_, CtrlSpec> {
        DisondemandW::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
