#[doc = "Register `FIFO3D417` reader"]
pub type R = crate::R<Fifo3d417Spec>;
#[doc = "Register `FIFO3D417` writer"]
pub type W = crate::W<Fifo3d417Spec>;
#[doc = "Field `FIFO3D` reader - EP 3 Data"]
pub type Fifo3dR = crate::BitReader;
#[doc = "Field `FIFO3D` writer - EP 3 Data"]
pub type Fifo3dW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EP 3 Data"]
    #[inline(always)]
    pub fn fifo3d(&self) -> Fifo3dR {
        Fifo3dR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EP 3 Data"]
    #[inline(always)]
    pub fn fifo3d(&mut self) -> Fifo3dW<'_, Fifo3d417Spec> {
        Fifo3dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo3d417::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo3d417::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo3d417Spec;
impl crate::RegisterSpec for Fifo3d417Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo3d417::R`](R) reader structure"]
impl crate::Readable for Fifo3d417Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo3d417::W`](W) writer structure"]
impl crate::Writable for Fifo3d417Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO3D417 to value 0"]
impl crate::Resettable for Fifo3d417Spec {}
