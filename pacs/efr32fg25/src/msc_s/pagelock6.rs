#[doc = "Register `PAGELOCK6` reader"]
pub type R = crate::R<Pagelock6Spec>;
#[doc = "Register `PAGELOCK6` writer"]
pub type W = crate::W<Pagelock6Spec>;
#[doc = "Field `LOCKBIT` reader - page lock bit"]
pub type LockbitR = crate::FieldReader<u32>;
#[doc = "Field `LOCKBIT` writer - page lock bit"]
pub type LockbitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - page lock bit"]
    #[inline(always)]
    pub fn lockbit(&self) -> LockbitR {
        LockbitR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - page lock bit"]
    #[inline(always)]
    pub fn lockbit(&mut self) -> LockbitW<'_, Pagelock6Spec> {
        LockbitW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pagelock6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pagelock6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pagelock6Spec;
impl crate::RegisterSpec for Pagelock6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pagelock6::R`](R) reader structure"]
impl crate::Readable for Pagelock6Spec {}
#[doc = "`write(|w| ..)` method takes [`pagelock6::W`](W) writer structure"]
impl crate::Writable for Pagelock6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAGELOCK6 to value 0"]
impl crate::Resettable for Pagelock6Spec {}
