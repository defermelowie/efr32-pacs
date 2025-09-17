#[doc = "Register `CONSUMER_CORE_CTIIN2` reader"]
pub type R = crate::R<ConsumerCoreCtiin2Spec>;
#[doc = "Register `CONSUMER_CORE_CTIIN2` writer"]
pub type W = crate::W<ConsumerCoreCtiin2Spec>;
#[doc = "Field `PRSSEL` reader - CTI async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - CTI async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CTI async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CTI async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, ConsumerCoreCtiin2Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerCoreCtiin2Spec;
impl crate::RegisterSpec for ConsumerCoreCtiin2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_core_ctiin2::R`](R) reader structure"]
impl crate::Readable for ConsumerCoreCtiin2Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_core_ctiin2::W`](W) writer structure"]
impl crate::Writable for ConsumerCoreCtiin2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_CORE_CTIIN2 to value 0"]
impl crate::Resettable for ConsumerCoreCtiin2Spec {}
