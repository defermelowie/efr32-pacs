#[doc = "Register `CH7_DST` reader"]
pub type R = crate::R<Ch7DstSpec>;
#[doc = "Register `CH7_DST` writer"]
pub type W = crate::W<Ch7DstSpec>;
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
    pub fn addr(&mut self) -> AddrW<'_, Ch7DstSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7DstSpec;
impl crate::RegisterSpec for Ch7DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_dst::R`](R) reader structure"]
impl crate::Readable for Ch7DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7_dst::W`](W) writer structure"]
impl crate::Writable for Ch7DstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7_DST to value 0"]
impl crate::Resettable for Ch7DstSpec {}
