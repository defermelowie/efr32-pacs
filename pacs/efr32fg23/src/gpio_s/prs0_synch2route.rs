#[doc = "Register `PRS0_SYNCH2ROUTE` reader"]
pub type R = crate::R<Prs0Synch2routeSpec>;
#[doc = "Register `PRS0_SYNCH2ROUTE` writer"]
pub type W = crate::W<Prs0Synch2routeSpec>;
#[doc = "Field `PORT` reader - SYNCH2 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - SYNCH2 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - SYNCH2 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - SYNCH2 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - SYNCH2 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - SYNCH2 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SYNCH2 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, Prs0Synch2routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - SYNCH2 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, Prs0Synch2routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "SYNCH2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_synch2route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_synch2route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prs0Synch2routeSpec;
impl crate::RegisterSpec for Prs0Synch2routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs0_synch2route::R`](R) reader structure"]
impl crate::Readable for Prs0Synch2routeSpec {}
#[doc = "`write(|w| ..)` method takes [`prs0_synch2route::W`](W) writer structure"]
impl crate::Writable for Prs0Synch2routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRS0_SYNCH2ROUTE to value 0"]
impl crate::Resettable for Prs0Synch2routeSpec {}
