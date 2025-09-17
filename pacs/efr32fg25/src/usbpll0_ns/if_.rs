#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `PLLRDY` reader - USBPLL Ready Interrupt Flag"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `PLLRDY` writer - USBPLL Ready Interrupt Flag"]
pub type PllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLOCKIF` reader - USBPLL Lock Interrupt Flag"]
pub type PlllockifR = crate::BitReader;
#[doc = "Field `PLLLOCKIF` writer - USBPLL Lock Interrupt Flag"]
pub type PlllockifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLNOLOCKIF` reader - USBPLL Loss of Lock Interrupt Flag"]
pub type PllnolockifR = crate::BitReader;
#[doc = "Field `PLLNOLOCKIF` writer - USBPLL Loss of Lock Interrupt Flag"]
pub type PllnolockifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USBPLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USBPLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn plllockif(&self) -> PlllockifR {
        PlllockifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USBPLL Loss of Lock Interrupt Flag"]
    #[inline(always)]
    pub fn pllnolockif(&self) -> PllnolockifR {
        PllnolockifR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBPLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn pllrdy(&mut self) -> PllrdyW<'_, IfSpec> {
        PllrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - USBPLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn plllockif(&mut self) -> PlllockifW<'_, IfSpec> {
        PlllockifW::new(self, 1)
    }
    #[doc = "Bit 2 - USBPLL Loss of Lock Interrupt Flag"]
    #[inline(always)]
    pub fn pllnolockif(&mut self) -> PllnolockifW<'_, IfSpec> {
        PllnolockifW::new(self, 2)
    }
}
#[doc = "Interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
