#[doc = "Register `EM4WUEN` reader"]
pub type R = crate::R<Em4wuenSpec>;
#[doc = "Register `EM4WUEN` writer"]
pub type W = crate::W<Em4wuenSpec>;
#[doc = "Field `CHNLEM4WUEN0` reader - Channel0 Tampdet EM4 Wakeup Enable"]
pub type Chnlem4wuen0R = crate::BitReader;
#[doc = "Field `CHNLEM4WUEN0` writer - Channel0 Tampdet EM4 Wakeup Enable"]
pub type Chnlem4wuen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNLEM4WUEN1` reader - Channel1 Tampdet EM4 Wakeup Enable"]
pub type Chnlem4wuen1R = crate::BitReader;
#[doc = "Field `CHNLEM4WUEN1` writer - Channel1 Tampdet EM4 Wakeup Enable"]
pub type Chnlem4wuen1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel0 Tampdet EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn chnlem4wuen0(&self) -> Chnlem4wuen0R {
        Chnlem4wuen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel1 Tampdet EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn chnlem4wuen1(&self) -> Chnlem4wuen1R {
        Chnlem4wuen1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel0 Tampdet EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn chnlem4wuen0(&mut self) -> Chnlem4wuen0W<'_, Em4wuenSpec> {
        Chnlem4wuen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel1 Tampdet EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn chnlem4wuen1(&mut self) -> Chnlem4wuen1W<'_, Em4wuenSpec> {
        Chnlem4wuen1W::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em4wuenSpec;
impl crate::RegisterSpec for Em4wuenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wuen::R`](R) reader structure"]
impl crate::Readable for Em4wuenSpec {}
#[doc = "`write(|w| ..)` method takes [`em4wuen::W`](W) writer structure"]
impl crate::Writable for Em4wuenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for Em4wuenSpec {}
