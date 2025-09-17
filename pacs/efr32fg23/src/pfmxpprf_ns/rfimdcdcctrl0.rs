#[doc = "Register `RFIMDCDCCTRL0` reader"]
pub type R = crate::R<Rfimdcdcctrl0Spec>;
#[doc = "Register `RFIMDCDCCTRL0` writer"]
pub type W = crate::W<Rfimdcdcctrl0Spec>;
#[doc = "Field `TXMAXREQ` reader - TX Max Req"]
pub type TxmaxreqR = crate::BitReader;
#[doc = "Field `TXMAXREQ` writer - TX Max Req"]
pub type TxmaxreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPPREQ` reader - RX PP Req"]
pub type RxppreqR = crate::BitReader;
#[doc = "Field `RXPPREQ` writer - RX PP Req"]
pub type RxppreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX Max Req"]
    #[inline(always)]
    pub fn txmaxreq(&self) -> TxmaxreqR {
        TxmaxreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX PP Req"]
    #[inline(always)]
    pub fn rxppreq(&self) -> RxppreqR {
        RxppreqR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Max Req"]
    #[inline(always)]
    pub fn txmaxreq(&mut self) -> TxmaxreqW<'_, Rfimdcdcctrl0Spec> {
        TxmaxreqW::new(self, 0)
    }
    #[doc = "Bit 1 - RX PP Req"]
    #[inline(always)]
    pub fn rxppreq(&mut self) -> RxppreqW<'_, Rfimdcdcctrl0Spec> {
        RxppreqW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimdcdcctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfimdcdcctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfimdcdcctrl0Spec;
impl crate::RegisterSpec for Rfimdcdcctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfimdcdcctrl0::R`](R) reader structure"]
impl crate::Readable for Rfimdcdcctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rfimdcdcctrl0::W`](W) writer structure"]
impl crate::Writable for Rfimdcdcctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFIMDCDCCTRL0 to value 0"]
impl crate::Resettable for Rfimdcdcctrl0Spec {}
