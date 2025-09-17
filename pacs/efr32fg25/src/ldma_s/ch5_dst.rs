#[doc = "Register `CH5_DST` reader"]
pub type R = crate::R<Ch5DstSpec>;
#[doc = "Register `CH5_DST` writer"]
pub type W = crate::W<Ch5DstSpec>;
#[doc = "Field `ADDR` reader - Destination Data Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Destination Data Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Ch5DstSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5DstSpec;
impl crate::RegisterSpec for Ch5DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_dst::R`](R) reader structure"]
impl crate::Readable for Ch5DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch5_dst::W`](W) writer structure"]
impl crate::Writable for Ch5DstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH5_DST to value 0"]
impl crate::Resettable for Ch5DstSpec {}
