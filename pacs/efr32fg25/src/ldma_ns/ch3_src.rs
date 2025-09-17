#[doc = "Register `CH3_SRC` reader"]
pub type R = crate::R<Ch3SrcSpec>;
#[doc = "Register `CH3_SRC` writer"]
pub type W = crate::W<Ch3SrcSpec>;
#[doc = "Field `ADDR` reader - Source Data Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Source Data Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Ch3SrcSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3SrcSpec;
impl crate::RegisterSpec for Ch3SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_src::R`](R) reader structure"]
impl crate::Readable for Ch3SrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3_src::W`](W) writer structure"]
impl crate::Writable for Ch3SrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3_SRC to value 0"]
impl crate::Resettable for Ch3SrcSpec {}
