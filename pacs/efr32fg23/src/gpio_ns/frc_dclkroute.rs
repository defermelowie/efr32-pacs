#[doc = "Register `FRC_DCLKROUTE` reader"]
pub type R = crate::R<FrcDclkrouteSpec>;
#[doc = "Register `FRC_DCLKROUTE` writer"]
pub type W = crate::W<FrcDclkrouteSpec>;
#[doc = "Field `PORT` reader - DCLK port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - DCLK port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - DCLK pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - DCLK pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - DCLK port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - DCLK pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DCLK port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, FrcDclkrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - DCLK pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, FrcDclkrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "DCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`frc_dclkroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frc_dclkroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrcDclkrouteSpec;
impl crate::RegisterSpec for FrcDclkrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc_dclkroute::R`](R) reader structure"]
impl crate::Readable for FrcDclkrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`frc_dclkroute::W`](W) writer structure"]
impl crate::Writable for FrcDclkrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRC_DCLKROUTE to value 0"]
impl crate::Resettable for FrcDclkrouteSpec {}
