#[doc = "Register `CFGNSRPURATD12` reader"]
pub type R = crate::R<Cfgnsrpuratd12Spec>;
#[doc = "Register `CFGNSRPURATD12` writer"]
pub type W = crate::W<Cfgnsrpuratd12Spec>;
#[doc = "Field `RATDROOTNSDATA0` reader - DATA0 Protection Bit"]
pub type Ratdrootnsdata0R = crate::BitReader;
#[doc = "Field `RATDROOTNSDATA0` writer - DATA0 Protection Bit"]
pub type Ratdrootnsdata0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATDROOTNSDATA1` reader - DATA1 Protection Bit"]
pub type Ratdrootnsdata1R = crate::BitReader;
#[doc = "Field `RATDROOTNSDATA1` writer - DATA1 Protection Bit"]
pub type Ratdrootnsdata1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DATA0 Protection Bit"]
    #[inline(always)]
    pub fn ratdrootnsdata0(&self) -> Ratdrootnsdata0R {
        Ratdrootnsdata0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DATA1 Protection Bit"]
    #[inline(always)]
    pub fn ratdrootnsdata1(&self) -> Ratdrootnsdata1R {
        Ratdrootnsdata1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DATA0 Protection Bit"]
    #[inline(always)]
    pub fn ratdrootnsdata0(&mut self) -> Ratdrootnsdata0W<'_, Cfgnsrpuratd12Spec> {
        Ratdrootnsdata0W::new(self, 0)
    }
    #[doc = "Bit 1 - DATA1 Protection Bit"]
    #[inline(always)]
    pub fn ratdrootnsdata1(&mut self) -> Ratdrootnsdata1W<'_, Cfgnsrpuratd12Spec> {
        Ratdrootnsdata1W::new(self, 1)
    }
}
#[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgnsrpuratd12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgnsrpuratd12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgnsrpuratd12Spec;
impl crate::RegisterSpec for Cfgnsrpuratd12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgnsrpuratd12::R`](R) reader structure"]
impl crate::Readable for Cfgnsrpuratd12Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgnsrpuratd12::W`](W) writer structure"]
impl crate::Writable for Cfgnsrpuratd12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGNSRPURATD12 to value 0"]
impl crate::Resettable for Cfgnsrpuratd12Spec {}
