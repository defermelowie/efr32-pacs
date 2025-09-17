#[doc = "Register `CHNLSEEDVAL0` reader"]
pub type R = crate::R<Chnlseedval0Spec>;
#[doc = "Register `CHNLSEEDVAL0` writer"]
pub type W = crate::W<Chnlseedval0Spec>;
#[doc = "Field `CHNLSEEDVAL0` reader - Channel 0 LFSR Seed Value"]
pub type Chnlseedval0R = crate::FieldReader<u32>;
#[doc = "Field `CHNLSEEDVAL0` writer - Channel 0 LFSR Seed Value"]
pub type Chnlseedval0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 0 LFSR Seed Value"]
    #[inline(always)]
    pub fn chnlseedval0(&self) -> Chnlseedval0R {
        Chnlseedval0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 0 LFSR Seed Value"]
    #[inline(always)]
    pub fn chnlseedval0(&mut self) -> Chnlseedval0W<'_, Chnlseedval0Spec> {
        Chnlseedval0W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlseedval0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlseedval0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chnlseedval0Spec;
impl crate::RegisterSpec for Chnlseedval0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnlseedval0::R`](R) reader structure"]
impl crate::Readable for Chnlseedval0Spec {}
#[doc = "`write(|w| ..)` method takes [`chnlseedval0::W`](W) writer structure"]
impl crate::Writable for Chnlseedval0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLSEEDVAL0 to value 0"]
impl crate::Resettable for Chnlseedval0Spec {}
