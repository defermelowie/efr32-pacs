#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `TAMPDET0` reader - TAMPDET0 interrupt enable"]
pub type Tampdet0R = crate::BitReader;
#[doc = "Field `TAMPDET0` writer - TAMPDET0 interrupt enable"]
pub type Tampdet0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPDET1` reader - TAMPDET1 interrupt enable"]
pub type Tampdet1R = crate::BitReader;
#[doc = "Field `TAMPDET1` writer - TAMPDET1 interrupt enable"]
pub type Tampdet1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMPDET0 interrupt enable"]
    #[inline(always)]
    pub fn tampdet0(&self) -> Tampdet0R {
        Tampdet0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPDET1 interrupt enable"]
    #[inline(always)]
    pub fn tampdet1(&self) -> Tampdet1R {
        Tampdet1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMPDET0 interrupt enable"]
    #[inline(always)]
    pub fn tampdet0(&mut self) -> Tampdet0W<'_, IenSpec> {
        Tampdet0W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMPDET1 interrupt enable"]
    #[inline(always)]
    pub fn tampdet1(&mut self) -> Tampdet1W<'_, IenSpec> {
        Tampdet1W::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
