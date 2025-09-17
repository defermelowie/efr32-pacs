#[doc = "Register `BUF2_READOFFSET` reader"]
pub type R = crate::R<Buf2ReadoffsetSpec>;
#[doc = "Register `BUF2_READOFFSET` writer"]
pub type W = crate::W<Buf2ReadoffsetSpec>;
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
    pub fn readoffset(&mut self) -> ReadoffsetW<'_, Buf2ReadoffsetSpec> {
        ReadoffsetW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_readoffset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_readoffset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf2ReadoffsetSpec;
impl crate::RegisterSpec for Buf2ReadoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf2_readoffset::R`](R) reader structure"]
impl crate::Readable for Buf2ReadoffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`buf2_readoffset::W`](W) writer structure"]
impl crate::Writable for Buf2ReadoffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF2_READOFFSET to value 0"]
impl crate::Resettable for Buf2ReadoffsetSpec {}
