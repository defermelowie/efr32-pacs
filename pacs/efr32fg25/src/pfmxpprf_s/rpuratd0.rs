#[doc = "Register `RPURATD0` reader"]
pub type R = crate::R<Rpuratd0Spec>;
#[doc = "Register `RPURATD0` writer"]
pub type W = crate::W<Rpuratd0Spec>;
#[doc = "Field `RATDRFIMDCDCCTRL0` reader - RFIMDCDCCTRL0 Protection Bit"]
pub type Ratdrfimdcdcctrl0R = crate::BitReader;
#[doc = "Field `RATDRFIMDCDCCTRL0` writer - RFIMDCDCCTRL0 Protection Bit"]
pub type Ratdrfimdcdcctrl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATDRFIMDCDCCTRL1` reader - RFIMDCDCCTRL1 Protection Bit"]
pub type Ratdrfimdcdcctrl1R = crate::BitReader;
#[doc = "Field `RATDRFIMDCDCCTRL1` writer - RFIMDCDCCTRL1 Protection Bit"]
pub type Ratdrfimdcdcctrl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATDRFIMDCDCCTRL2` reader - RFIMDCDCCTRL2 Protection Bit"]
pub type Ratdrfimdcdcctrl2R = crate::BitReader;
#[doc = "Field `RATDRFIMDCDCCTRL2` writer - RFIMDCDCCTRL2 Protection Bit"]
pub type Ratdrfimdcdcctrl2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RFIMDCDCCTRL0 Protection Bit"]
    #[inline(always)]
    pub fn ratdrfimdcdcctrl0(&self) -> Ratdrfimdcdcctrl0R {
        Ratdrfimdcdcctrl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RFIMDCDCCTRL1 Protection Bit"]
    #[inline(always)]
    pub fn ratdrfimdcdcctrl1(&self) -> Ratdrfimdcdcctrl1R {
        Ratdrfimdcdcctrl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RFIMDCDCCTRL2 Protection Bit"]
    #[inline(always)]
    pub fn ratdrfimdcdcctrl2(&self) -> Ratdrfimdcdcctrl2R {
        Ratdrfimdcdcctrl2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RFIMDCDCCTRL0 Protection Bit"]
    #[inline(always)]
    pub fn ratdrfimdcdcctrl0(&mut self) -> Ratdrfimdcdcctrl0W<'_, Rpuratd0Spec> {
        Ratdrfimdcdcctrl0W::new(self, 0)
    }
    #[doc = "Bit 1 - RFIMDCDCCTRL1 Protection Bit"]
    #[inline(always)]
    pub fn ratdrfimdcdcctrl1(&mut self) -> Ratdrfimdcdcctrl1W<'_, Rpuratd0Spec> {
        Ratdrfimdcdcctrl1W::new(self, 1)
    }
    #[doc = "Bit 2 - RFIMDCDCCTRL2 Protection Bit"]
    #[inline(always)]
    pub fn ratdrfimdcdcctrl2(&mut self) -> Ratdrfimdcdcctrl2W<'_, Rpuratd0Spec> {
        Ratdrfimdcdcctrl2W::new(self, 2)
    }
}
#[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rpuratd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpuratd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rpuratd0Spec;
impl crate::RegisterSpec for Rpuratd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpuratd0::R`](R) reader structure"]
impl crate::Readable for Rpuratd0Spec {}
#[doc = "`write(|w| ..)` method takes [`rpuratd0::W`](W) writer structure"]
impl crate::Writable for Rpuratd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RPURATD0 to value 0"]
impl crate::Resettable for Rpuratd0Spec {}
