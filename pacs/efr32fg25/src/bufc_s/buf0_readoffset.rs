#[doc = "Register `BUF0_READOFFSET` reader"]
pub type R = crate::R<Buf0ReadoffsetSpec>;
#[doc = "Register `BUF0_READOFFSET` writer"]
pub type W = crate::W<Buf0ReadoffsetSpec>;
#[doc = "Field `READOFFSET` reader - Read Offset"]
pub type ReadoffsetR = crate::FieldReader<u16>;
#[doc = "Field `READOFFSET` writer - Read Offset"]
pub type ReadoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Read Offset"]
    #[inline(always)]
    pub fn readoffset(&self) -> ReadoffsetR {
        ReadoffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Read Offset"]
    #[inline(always)]
    pub fn readoffset(&mut self) -> ReadoffsetW<'_, Buf0ReadoffsetSpec> {
        ReadoffsetW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_readoffset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_readoffset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf0ReadoffsetSpec;
impl crate::RegisterSpec for Buf0ReadoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf0_readoffset::R`](R) reader structure"]
impl crate::Readable for Buf0ReadoffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`buf0_readoffset::W`](W) writer structure"]
impl crate::Writable for Buf0ReadoffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF0_READOFFSET to value 0"]
impl crate::Resettable for Buf0ReadoffsetSpec {}
