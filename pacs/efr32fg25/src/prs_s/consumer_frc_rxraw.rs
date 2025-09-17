#[doc = "Register `CONSUMER_FRC_RXRAW` reader"]
pub type R = crate::R<ConsumerFrcRxrawSpec>;
#[doc = "Register `CONSUMER_FRC_RXRAW` writer"]
pub type W = crate::W<ConsumerFrcRxrawSpec>;
#[doc = "Field `PRSSEL` reader - RXRAW async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - RXRAW async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RXRAW async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RXRAW async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, ConsumerFrcRxrawSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "RXRAW consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_frc_rxraw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_frc_rxraw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerFrcRxrawSpec;
impl crate::RegisterSpec for ConsumerFrcRxrawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_frc_rxraw::R`](R) reader structure"]
impl crate::Readable for ConsumerFrcRxrawSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_frc_rxraw::W`](W) writer structure"]
impl crate::Writable for ConsumerFrcRxrawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_FRC_RXRAW to value 0"]
impl crate::Resettable for ConsumerFrcRxrawSpec {}
