#[doc = "Register `CONSUMER_PCNT0_S1IN` reader"]
pub type R = crate::R<ConsumerPcnt0S1inSpec>;
#[doc = "Register `CONSUMER_PCNT0_S1IN` writer"]
pub type W = crate::W<ConsumerPcnt0S1inSpec>;
#[doc = "Field `PRSSEL` reader - S1IN async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - S1IN async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - S1IN async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - S1IN async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, ConsumerPcnt0S1inSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "S1IN Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_pcnt0_s1in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_pcnt0_s1in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerPcnt0S1inSpec;
impl crate::RegisterSpec for ConsumerPcnt0S1inSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_pcnt0_s1in::R`](R) reader structure"]
impl crate::Readable for ConsumerPcnt0S1inSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_pcnt0_s1in::W`](W) writer structure"]
impl crate::Writable for ConsumerPcnt0S1inSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_PCNT0_S1IN to value 0"]
impl crate::Resettable for ConsumerPcnt0S1inSpec {}
