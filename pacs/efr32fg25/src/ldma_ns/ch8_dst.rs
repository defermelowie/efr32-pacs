#[doc = "Register `CH8_DST` reader"]
pub type R = crate::R<Ch8DstSpec>;
#[doc = "Register `CH8_DST` writer"]
pub type W = crate::W<Ch8DstSpec>;
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
    pub fn addr(&mut self) -> AddrW<'_, Ch8DstSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch8DstSpec;
impl crate::RegisterSpec for Ch8DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch8_dst::R`](R) reader structure"]
impl crate::Readable for Ch8DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch8_dst::W`](W) writer structure"]
impl crate::Writable for Ch8DstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH8_DST to value 0"]
impl crate::Resettable for Ch8DstSpec {}
