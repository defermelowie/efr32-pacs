#[doc = "Register `EUSART1_RTSROUTE` reader"]
pub type R = crate::R<Eusart1RtsrouteSpec>;
#[doc = "Register `EUSART1_RTSROUTE` writer"]
pub type W = crate::W<Eusart1RtsrouteSpec>;
#[doc = "Field `PORT` reader - RTS port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - RTS port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - RTS pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - RTS pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - RTS port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - RTS pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTS port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, Eusart1RtsrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - RTS pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, Eusart1RtsrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_rtsroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_rtsroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eusart1RtsrouteSpec;
impl crate::RegisterSpec for Eusart1RtsrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eusart1_rtsroute::R`](R) reader structure"]
impl crate::Readable for Eusart1RtsrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`eusart1_rtsroute::W`](W) writer structure"]
impl crate::Writable for Eusart1RtsrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EUSART1_RTSROUTE to value 0"]
impl crate::Resettable for Eusart1RtsrouteSpec {}
