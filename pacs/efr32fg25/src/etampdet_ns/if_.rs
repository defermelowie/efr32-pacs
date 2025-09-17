#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `TAMPDET0` reader - Tamper0 Detect Flag"]
pub type Tampdet0R = crate::BitReader;
#[doc = "Field `TAMPDET0` writer - Tamper0 Detect Flag"]
pub type Tampdet0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPDET1` reader - Tamper1 Detect Flag"]
pub type Tampdet1R = crate::BitReader;
#[doc = "Field `TAMPDET1` writer - Tamper1 Detect Flag"]
pub type Tampdet1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper0 Detect Flag"]
    #[inline(always)]
    pub fn tampdet0(&self) -> Tampdet0R {
        Tampdet0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper1 Detect Flag"]
    #[inline(always)]
    pub fn tampdet1(&self) -> Tampdet1R {
        Tampdet1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper0 Detect Flag"]
    #[inline(always)]
    pub fn tampdet0(&mut self) -> Tampdet0W<'_, IfSpec> {
        Tampdet0W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper1 Detect Flag"]
    #[inline(always)]
    pub fn tampdet1(&mut self) -> Tampdet1W<'_, IfSpec> {
        Tampdet1W::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
