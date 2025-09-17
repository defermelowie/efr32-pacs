#[doc = "Register `PRS0_ASYNCH7ROUTE` reader"]
pub type R = crate::R<Prs0Asynch7routeSpec>;
#[doc = "Register `PRS0_ASYNCH7ROUTE` writer"]
pub type W = crate::W<Prs0Asynch7routeSpec>;
#[doc = "Field `PORT` reader - ASYNCH7 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - ASYNCH7 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - ASYNCH7 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - ASYNCH7 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - ASYNCH7 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - ASYNCH7 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ASYNCH7 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, Prs0Asynch7routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - ASYNCH7 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, Prs0Asynch7routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "ASYNCH7 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch7route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch7route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prs0Asynch7routeSpec;
impl crate::RegisterSpec for Prs0Asynch7routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs0_asynch7route::R`](R) reader structure"]
impl crate::Readable for Prs0Asynch7routeSpec {}
#[doc = "`write(|w| ..)` method takes [`prs0_asynch7route::W`](W) writer structure"]
impl crate::Writable for Prs0Asynch7routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRS0_ASYNCH7ROUTE to value 0"]
impl crate::Resettable for Prs0Asynch7routeSpec {}
