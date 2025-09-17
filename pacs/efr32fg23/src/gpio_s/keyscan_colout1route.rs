#[doc = "Register `KEYSCAN_COLOUT1ROUTE` reader"]
pub type R = crate::R<KeyscanColout1routeSpec>;
#[doc = "Register `KEYSCAN_COLOUT1ROUTE` writer"]
pub type W = crate::W<KeyscanColout1routeSpec>;
#[doc = "Field `PORT` reader - COLOUT1 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - COLOUT1 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - COLOUT1 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - COLOUT1 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - COLOUT1 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - COLOUT1 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - COLOUT1 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, KeyscanColout1routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - COLOUT1 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, KeyscanColout1routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "COLOUT1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout1route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout1route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyscanColout1routeSpec;
impl crate::RegisterSpec for KeyscanColout1routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyscan_colout1route::R`](R) reader structure"]
impl crate::Readable for KeyscanColout1routeSpec {}
#[doc = "`write(|w| ..)` method takes [`keyscan_colout1route::W`](W) writer structure"]
impl crate::Writable for KeyscanColout1routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYSCAN_COLOUT1ROUTE to value 0"]
impl crate::Resettable for KeyscanColout1routeSpec {}
