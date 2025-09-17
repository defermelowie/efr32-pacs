#[doc = "Register `TIMER3_CC0ROUTE` reader"]
pub type R = crate::R<Timer3Cc0routeSpec>;
#[doc = "Register `TIMER3_CC0ROUTE` writer"]
pub type W = crate::W<Timer3Cc0routeSpec>;
#[doc = "Field `PORT` reader - CC0 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CC0 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CC0 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CC0 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CC0 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CC0 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC0 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, Timer3Cc0routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CC0 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, Timer3Cc0routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cc0route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cc0route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer3Cc0routeSpec;
impl crate::RegisterSpec for Timer3Cc0routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer3_cc0route::R`](R) reader structure"]
impl crate::Readable for Timer3Cc0routeSpec {}
#[doc = "`write(|w| ..)` method takes [`timer3_cc0route::W`](W) writer structure"]
impl crate::Writable for Timer3Cc0routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER3_CC0ROUTE to value 0"]
impl crate::Resettable for Timer3Cc0routeSpec {}
