#[doc = "Register `FIFO6D980` reader"]
pub type R = crate::R<Fifo6d980Spec>;
#[doc = "Register `FIFO6D980` writer"]
pub type W = crate::W<Fifo6d980Spec>;
#[doc = "Field `FIFO6D` reader - EP 6 Data"]
pub type Fifo6dR = crate::BitReader;
#[doc = "Field `FIFO6D` writer - EP 6 Data"]
pub type Fifo6dW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EP 6 Data"]
    #[inline(always)]
    pub fn fifo6d(&self) -> Fifo6dR {
        Fifo6dR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EP 6 Data"]
    #[inline(always)]
    pub fn fifo6d(&mut self) -> Fifo6dW<'_, Fifo6d980Spec> {
        Fifo6dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo6d980::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo6d980::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo6d980Spec;
impl crate::RegisterSpec for Fifo6d980Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo6d980::R`](R) reader structure"]
impl crate::Readable for Fifo6d980Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo6d980::W`](W) writer structure"]
impl crate::Writable for Fifo6d980Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO6D980 to value 0"]
impl crate::Resettable for Fifo6d980Spec {}
