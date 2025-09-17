#[doc = "Register `BMPUSATD0` reader"]
pub type R = crate::R<Bmpusatd0Spec>;
#[doc = "Register `BMPUSATD0` writer"]
pub type W = crate::W<Bmpusatd0Spec>;
#[doc = "Field `RADIOAES` reader - RADIOAES DMA secure mode"]
pub type RadioaesR = crate::BitReader;
#[doc = "Field `RADIOAES` writer - RADIOAES DMA secure mode"]
pub type RadioaesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RADIOSUBSYSTEM` reader - RADIO subsystem manager secure mode"]
pub type RadiosubsystemR = crate::BitReader;
#[doc = "Field `RADIOSUBSYSTEM` writer - RADIO subsystem manager secure mode"]
pub type RadiosubsystemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA` reader - MCU LDMA secure mode"]
pub type LdmaR = crate::BitReader;
#[doc = "Field `LDMA` writer - MCU LDMA secure mode"]
pub type LdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFECA0` reader - RFECA0 secure mode"]
pub type Rfeca0R = crate::BitReader;
#[doc = "Field `RFECA0` writer - RFECA0 secure mode"]
pub type Rfeca0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFECA1` reader - RFECA1 secure mode"]
pub type Rfeca1R = crate::BitReader;
#[doc = "Field `RFECA1` writer - RFECA1 secure mode"]
pub type Rfeca1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEEXTDMA` reader - SEEXTDMA secure mode"]
pub type SeextdmaR = crate::BitReader;
#[doc = "Field `SEEXTDMA` writer - SEEXTDMA secure mode"]
pub type SeextdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RADIOAES DMA secure mode"]
    #[inline(always)]
    pub fn radioaes(&self) -> RadioaesR {
        RadioaesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RADIO subsystem manager secure mode"]
    #[inline(always)]
    pub fn radiosubsystem(&self) -> RadiosubsystemR {
        RadiosubsystemR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCU LDMA secure mode"]
    #[inline(always)]
    pub fn ldma(&self) -> LdmaR {
        LdmaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RFECA0 secure mode"]
    #[inline(always)]
    pub fn rfeca0(&self) -> Rfeca0R {
        Rfeca0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RFECA1 secure mode"]
    #[inline(always)]
    pub fn rfeca1(&self) -> Rfeca1R {
        Rfeca1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SEEXTDMA secure mode"]
    #[inline(always)]
    pub fn seextdma(&self) -> SeextdmaR {
        SeextdmaR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RADIOAES DMA secure mode"]
    #[inline(always)]
    pub fn radioaes(&mut self) -> RadioaesW<'_, Bmpusatd0Spec> {
        RadioaesW::new(self, 0)
    }
    #[doc = "Bit 1 - RADIO subsystem manager secure mode"]
    #[inline(always)]
    pub fn radiosubsystem(&mut self) -> RadiosubsystemW<'_, Bmpusatd0Spec> {
        RadiosubsystemW::new(self, 1)
    }
    #[doc = "Bit 2 - MCU LDMA secure mode"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LdmaW<'_, Bmpusatd0Spec> {
        LdmaW::new(self, 2)
    }
    #[doc = "Bit 3 - RFECA0 secure mode"]
    #[inline(always)]
    pub fn rfeca0(&mut self) -> Rfeca0W<'_, Bmpusatd0Spec> {
        Rfeca0W::new(self, 3)
    }
    #[doc = "Bit 4 - RFECA1 secure mode"]
    #[inline(always)]
    pub fn rfeca1(&mut self) -> Rfeca1W<'_, Bmpusatd0Spec> {
        Rfeca1W::new(self, 4)
    }
    #[doc = "Bit 5 - SEEXTDMA secure mode"]
    #[inline(always)]
    pub fn seextdma(&mut self) -> SeextdmaW<'_, Bmpusatd0Spec> {
        SeextdmaW::new(self, 5)
    }
}
#[doc = "Set master bits to 1 to mark as a secure master\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpusatd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmpusatd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bmpusatd0Spec;
impl crate::RegisterSpec for Bmpusatd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmpusatd0::R`](R) reader structure"]
impl crate::Readable for Bmpusatd0Spec {}
#[doc = "`write(|w| ..)` method takes [`bmpusatd0::W`](W) writer structure"]
impl crate::Writable for Bmpusatd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMPUSATD0 to value 0x3f"]
impl crate::Resettable for Bmpusatd0Spec {
    const RESET_VALUE: u32 = 0x3f;
}
