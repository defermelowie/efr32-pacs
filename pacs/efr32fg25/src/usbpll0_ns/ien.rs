#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `PLLRDY` reader - USBPLL Ready Interrupt Enable"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `PLLRDY` writer - USBPLL Ready Interrupt Enable"]
pub type PllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLOCKIEN` reader - USBPLL Lock Interrupt Enable"]
pub type PlllockienR = crate::BitReader;
#[doc = "Field `PLLLOCKIEN` writer - USBPLL Lock Interrupt Enable"]
pub type PlllockienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLNOLOCKIEN` reader - USBPLL Loss of Lock Interrupt Enable"]
pub type PllnolockienR = crate::BitReader;
#[doc = "Field `PLLNOLOCKIEN` writer - USBPLL Loss of Lock Interrupt Enable"]
pub type PllnolockienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USBPLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USBPLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn plllockien(&self) -> PlllockienR {
        PlllockienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USBPLL Loss of Lock Interrupt Enable"]
    #[inline(always)]
    pub fn pllnolockien(&self) -> PllnolockienR {
        PllnolockienR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBPLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllrdy(&mut self) -> PllrdyW<'_, IenSpec> {
        PllrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - USBPLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn plllockien(&mut self) -> PlllockienW<'_, IenSpec> {
        PlllockienW::new(self, 1)
    }
    #[doc = "Bit 2 - USBPLL Loss of Lock Interrupt Enable"]
    #[inline(always)]
    pub fn pllnolockien(&mut self) -> PllnolockienW<'_, IenSpec> {
        PllnolockienW::new(self, 2)
    }
}
#[doc = "Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
