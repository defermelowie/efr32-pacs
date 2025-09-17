#[doc = "Register `BMPUPATD0` reader"]
pub type R = crate::R<Bmpupatd0Spec>;
#[doc = "Register `BMPUPATD0` writer"]
pub type W = crate::W<Bmpupatd0Spec>;
#[doc = "Field `RADIOAES` reader - RADIO AES DMA privileged mode"]
pub type RadioaesR = crate::BitReader;
#[doc = "Field `RADIOAES` writer - RADIO AES DMA privileged mode"]
pub type RadioaesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RADIOSUBSYSTEM` reader - RADIO subsystem manager privileged mode"]
pub type RadiosubsystemR = crate::BitReader;
#[doc = "Field `RADIOSUBSYSTEM` writer - RADIO subsystem manager privileged mode"]
pub type RadiosubsystemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFC` reader - BUFC privileged mode"]
pub type BufcR = crate::BitReader;
#[doc = "Field `BUFC` writer - BUFC privileged mode"]
pub type BufcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA0` reader - MCU LDMA port 0 privileged mode"]
pub type Ldma0R = crate::BitReader;
#[doc = "Field `LDMA0` writer - MCU LDMA port 0 privileged mode"]
pub type Ldma0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA1` reader - MCU LDMA port 1 privileged mode"]
pub type Ldma1R = crate::BitReader;
#[doc = "Field `LDMA1` writer - MCU LDMA port 1 privileged mode"]
pub type Ldma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFECA0` reader - RFECA0 privileged mode"]
pub type Rfeca0R = crate::BitReader;
#[doc = "Field `RFECA0` writer - RFECA0 privileged mode"]
pub type Rfeca0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFECA1` reader - RFECA1 privileged mode"]
pub type Rfeca1R = crate::BitReader;
#[doc = "Field `RFECA1` writer - RFECA1 privileged mode"]
pub type Rfeca1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFLDMA` reader - RFLDMA privileged mode"]
pub type RfldmaR = crate::BitReader;
#[doc = "Field `RFLDMA` writer - RFLDMA privileged mode"]
pub type RfldmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEEXTDMA` reader - SEEXTDMA privileged mode"]
pub type SeextdmaR = crate::BitReader;
#[doc = "Field `SEEXTDMA` writer - SEEXTDMA privileged mode"]
pub type SeextdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - USB privileged mode"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - USB privileged mode"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - BUFC privileged mode"]
    #[inline(always)]
    pub fn bufc(&self) -> BufcR {
        BufcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MCU LDMA port 0 privileged mode"]
    #[inline(always)]
    pub fn ldma0(&self) -> Ldma0R {
        Ldma0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MCU LDMA port 1 privileged mode"]
    #[inline(always)]
    pub fn ldma1(&self) -> Ldma1R {
        Ldma1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFECA0 privileged mode"]
    #[inline(always)]
    pub fn rfeca0(&self) -> Rfeca0R {
        Rfeca0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RFECA1 privileged mode"]
    #[inline(always)]
    pub fn rfeca1(&self) -> Rfeca1R {
        Rfeca1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RFLDMA privileged mode"]
    #[inline(always)]
    pub fn rfldma(&self) -> RfldmaR {
        RfldmaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SEEXTDMA privileged mode"]
    #[inline(always)]
    pub fn seextdma(&self) -> SeextdmaR {
        SeextdmaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB privileged mode"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RADIO AES DMA privileged mode"]
    #[inline(always)]
    pub fn radioaes(&mut self) -> RadioaesW<'_, Bmpupatd0Spec> {
        RadioaesW::new(self, 0)
    }
    #[doc = "Bit 1 - RADIO subsystem manager privileged mode"]
    #[inline(always)]
    pub fn radiosubsystem(&mut self) -> RadiosubsystemW<'_, Bmpupatd0Spec> {
        RadiosubsystemW::new(self, 1)
    }
    #[doc = "Bit 2 - BUFC privileged mode"]
    #[inline(always)]
    pub fn bufc(&mut self) -> BufcW<'_, Bmpupatd0Spec> {
        BufcW::new(self, 2)
    }
    #[doc = "Bit 3 - MCU LDMA port 0 privileged mode"]
    #[inline(always)]
    pub fn ldma0(&mut self) -> Ldma0W<'_, Bmpupatd0Spec> {
        Ldma0W::new(self, 3)
    }
    #[doc = "Bit 4 - MCU LDMA port 1 privileged mode"]
    #[inline(always)]
    pub fn ldma1(&mut self) -> Ldma1W<'_, Bmpupatd0Spec> {
        Ldma1W::new(self, 4)
    }
    #[doc = "Bit 5 - RFECA0 privileged mode"]
    #[inline(always)]
    pub fn rfeca0(&mut self) -> Rfeca0W<'_, Bmpupatd0Spec> {
        Rfeca0W::new(self, 5)
    }
    #[doc = "Bit 6 - RFECA1 privileged mode"]
    #[inline(always)]
    pub fn rfeca1(&mut self) -> Rfeca1W<'_, Bmpupatd0Spec> {
        Rfeca1W::new(self, 6)
    }
    #[doc = "Bit 7 - RFLDMA privileged mode"]
    #[inline(always)]
    pub fn rfldma(&mut self) -> RfldmaW<'_, Bmpupatd0Spec> {
        RfldmaW::new(self, 7)
    }
    #[doc = "Bit 8 - SEEXTDMA privileged mode"]
    #[inline(always)]
    pub fn seextdma(&mut self) -> SeextdmaW<'_, Bmpupatd0Spec> {
        SeextdmaW::new(self, 8)
    }
    #[doc = "Bit 9 - USB privileged mode"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, Bmpupatd0Spec> {
        UsbW::new(self, 9)
    }
}
#[doc = "Set master bits to 1 to mark as a privileged master\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpupatd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmpupatd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bmpupatd0Spec;
impl crate::RegisterSpec for Bmpupatd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmpupatd0::R`](R) reader structure"]
impl crate::Readable for Bmpupatd0Spec {}
#[doc = "`write(|w| ..)` method takes [`bmpupatd0::W`](W) writer structure"]
impl crate::Writable for Bmpupatd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMPUPATD0 to value 0x03ff"]
impl crate::Resettable for Bmpupatd0Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
