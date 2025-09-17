#[doc = "Register `CHNLSEEDVAL1` reader"]
pub type R = crate::R<Chnlseedval1Spec>;
#[doc = "Register `CHNLSEEDVAL1` writer"]
pub type W = crate::W<Chnlseedval1Spec>;
#[doc = "Field `CHNLSEEDVAL1` reader - Channel 1 LFSR Seed Value"]
pub type Chnlseedval1R = crate::FieldReader<u32>;
#[doc = "Field `CHNLSEEDVAL1` writer - Channel 1 LFSR Seed Value"]
pub type Chnlseedval1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 1 LFSR Seed Value"]
    #[inline(always)]
    pub fn chnlseedval1(&self) -> Chnlseedval1R {
        Chnlseedval1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 1 LFSR Seed Value"]
    #[inline(always)]
    pub fn chnlseedval1(&mut self) -> Chnlseedval1W<'_, Chnlseedval1Spec> {
        Chnlseedval1W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlseedval1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlseedval1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chnlseedval1Spec;
impl crate::RegisterSpec for Chnlseedval1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnlseedval1::R`](R) reader structure"]
impl crate::Readable for Chnlseedval1Spec {}
#[doc = "`write(|w| ..)` method takes [`chnlseedval1::W`](W) writer structure"]
impl crate::Writable for Chnlseedval1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLSEEDVAL1 to value 0"]
impl crate::Resettable for Chnlseedval1Spec {}
