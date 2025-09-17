#[doc = "Register `BMPUNSPATD0` reader"]
pub type R = crate::R<Bmpunspatd0Spec>;
#[doc = "Register `BMPUNSPATD0` writer"]
pub type W = crate::W<Bmpunspatd0Spec>;
#[doc = "Field `RADIOAES` reader - RADIO AES DMA privileged mode"]
pub type RadioaesR = crate::BitReader;
#[doc = "Field `RADIOAES` writer - RADIO AES DMA privileged mode"]
pub type RadioaesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RADIOSUBSYSTEM` reader - RADIO subsystem manager privileged mode"]
pub type RadiosubsystemR = crate::BitReader;
#[doc = "Field `RADIOSUBSYSTEM` writer - RADIO subsystem manager privileged mode"]
pub type RadiosubsystemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA` reader - MCU LDMA privileged mode"]
pub type LdmaR = crate::BitReader;
#[doc = "Field `LDMA` writer - MCU LDMA privileged mode"]
pub type LdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFECA0` reader - RFECA0 privileged mode"]
pub type Rfeca0R = crate::BitReader;
#[doc = "Field `RFECA0` writer - RFECA0 privileged mode"]
pub type Rfeca0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFECA1` reader - RFECA1 privileged mode"]
pub type Rfeca1R = crate::BitReader;
#[doc = "Field `RFECA1` writer - RFECA1 privileged mode"]
pub type Rfeca1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEEXTDMA` reader - SEEXTDMA privileged mode"]
pub type SeextdmaR = crate::BitReader;
#[doc = "Field `SEEXTDMA` writer - SEEXTDMA privileged mode"]
pub type SeextdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RADIO AES DMA privileged mode"]
    #[inline(always)]
    pub fn radioaes(&self) -> RadioaesR {
        RadioaesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RADIO subsystem manager privileged mode"]
    #[inline(always)]
    pub fn radiosubsystem(&self) -> RadiosubsystemR {
        RadiosubsystemR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCU LDMA privileged mode"]
    #[inline(always)]
    pub fn ldma(&self) -> LdmaR {
        LdmaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RFECA0 privileged mode"]
    #[inline(always)]
    pub fn rfeca0(&self) -> Rfeca0R {
        Rfeca0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RFECA1 privileged mode"]
    #[inline(always)]
    pub fn rfeca1(&self) -> Rfeca1R {
        Rfeca1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SEEXTDMA privileged mode"]
    #[inline(always)]
    pub fn seextdma(&self) -> SeextdmaR {
        SeextdmaR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RADIO AES DMA privileged mode"]
    #[inline(always)]
    pub fn radioaes(&mut self) -> RadioaesW<'_, Bmpunspatd0Spec> {
        RadioaesW::new(self, 0)
    }
    #[doc = "Bit 1 - RADIO subsystem manager privileged mode"]
    #[inline(always)]
    pub fn radiosubsystem(&mut self) -> RadiosubsystemW<'_, Bmpunspatd0Spec> {
        RadiosubsystemW::new(self, 1)
    }
    #[doc = "Bit 2 - MCU LDMA privileged mode"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LdmaW<'_, Bmpunspatd0Spec> {
        LdmaW::new(self, 2)
    }
    #[doc = "Bit 3 - RFECA0 privileged mode"]
    #[inline(always)]
    pub fn rfeca0(&mut self) -> Rfeca0W<'_, Bmpunspatd0Spec> {
        Rfeca0W::new(self, 3)
    }
    #[doc = "Bit 4 - RFECA1 privileged mode"]
    #[inline(always)]
    pub fn rfeca1(&mut self) -> Rfeca1W<'_, Bmpunspatd0Spec> {
        Rfeca1W::new(self, 4)
    }
    #[doc = "Bit 5 - SEEXTDMA privileged mode"]
    #[inline(always)]
    pub fn seextdma(&mut self) -> SeextdmaW<'_, Bmpunspatd0Spec> {
        SeextdmaW::new(self, 5)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpunspatd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmpunspatd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bmpunspatd0Spec;
impl crate::RegisterSpec for Bmpunspatd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmpunspatd0::R`](R) reader structure"]
impl crate::Readable for Bmpunspatd0Spec {}
#[doc = "`write(|w| ..)` method takes [`bmpunspatd0::W`](W) writer structure"]
impl crate::Writable for Bmpunspatd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMPUNSPATD0 to value 0"]
impl crate::Resettable for Bmpunspatd0Spec {}
