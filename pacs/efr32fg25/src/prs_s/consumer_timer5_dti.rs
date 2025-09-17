#[doc = "Register `CONSUMER_TIMER5_DTI` reader"]
pub type R = crate::R<ConsumerTimer5DtiSpec>;
#[doc = "Register `CONSUMER_TIMER5_DTI` writer"]
pub type W = crate::W<ConsumerTimer5DtiSpec>;
#[doc = "Field `PRSSEL` reader - DTI async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - DTI async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DTI async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DTI async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, ConsumerTimer5DtiSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer5_dti::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer5_dti::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerTimer5DtiSpec;
impl crate::RegisterSpec for ConsumerTimer5DtiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_timer5_dti::R`](R) reader structure"]
impl crate::Readable for ConsumerTimer5DtiSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_timer5_dti::W`](W) writer structure"]
impl crate::Writable for ConsumerTimer5DtiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_TIMER5_DTI to value 0"]
impl crate::Resettable for ConsumerTimer5DtiSpec {}
