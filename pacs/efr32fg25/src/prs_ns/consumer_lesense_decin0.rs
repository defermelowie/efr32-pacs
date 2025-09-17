#[doc = "Register `CONSUMER_LESENSE_DECIN0` reader"]
pub type R = crate::R<ConsumerLesenseDecin0Spec>;
#[doc = "Register `CONSUMER_LESENSE_DECIN0` writer"]
pub type W = crate::W<ConsumerLesenseDecin0Spec>;
#[doc = "Field `PRSSEL` reader - DECIN0 async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - DECIN0 async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DECIN0 async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DECIN0 async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, ConsumerLesenseDecin0Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "DECIN0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_lesense_decin0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_lesense_decin0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerLesenseDecin0Spec;
impl crate::RegisterSpec for ConsumerLesenseDecin0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_lesense_decin0::R`](R) reader structure"]
impl crate::Readable for ConsumerLesenseDecin0Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_lesense_decin0::W`](W) writer structure"]
impl crate::Writable for ConsumerLesenseDecin0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_LESENSE_DECIN0 to value 0"]
impl crate::Resettable for ConsumerLesenseDecin0Spec {}
