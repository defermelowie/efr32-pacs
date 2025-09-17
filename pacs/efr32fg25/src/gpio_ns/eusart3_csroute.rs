#[doc = "Register `EUSART3_CSROUTE` reader"]
pub type R = crate::R<Eusart3CsrouteSpec>;
#[doc = "Register `EUSART3_CSROUTE` writer"]
pub type W = crate::W<Eusart3CsrouteSpec>;
#[doc = "Field `PORT` reader - CS port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CS port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CS pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CS pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CS port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CS pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CS port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, Eusart3CsrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CS pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, Eusart3CsrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart3_csroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart3_csroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eusart3CsrouteSpec;
impl crate::RegisterSpec for Eusart3CsrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eusart3_csroute::R`](R) reader structure"]
impl crate::Readable for Eusart3CsrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`eusart3_csroute::W`](W) writer structure"]
impl crate::Writable for Eusart3CsrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EUSART3_CSROUTE to value 0"]
impl crate::Resettable for Eusart3CsrouteSpec {}
