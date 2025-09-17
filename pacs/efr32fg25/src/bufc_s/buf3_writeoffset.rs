#[doc = "Register `BUF3_WRITEOFFSET` reader"]
pub type R = crate::R<Buf3WriteoffsetSpec>;
#[doc = "Register `BUF3_WRITEOFFSET` writer"]
pub type W = crate::W<Buf3WriteoffsetSpec>;
#[doc = "Field `WRITEOFFSET` reader - Write Offset"]
pub type WriteoffsetR = crate::FieldReader<u16>;
#[doc = "Field `WRITEOFFSET` writer - Write Offset"]
pub type WriteoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Write Offset"]
    #[inline(always)]
    pub fn writeoffset(&self) -> WriteoffsetR {
        WriteoffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Write Offset"]
    #[inline(always)]
    pub fn writeoffset(&mut self) -> WriteoffsetW<'_, Buf3WriteoffsetSpec> {
        WriteoffsetW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_writeoffset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_writeoffset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf3WriteoffsetSpec;
impl crate::RegisterSpec for Buf3WriteoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf3_writeoffset::R`](R) reader structure"]
impl crate::Readable for Buf3WriteoffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`buf3_writeoffset::W`](W) writer structure"]
impl crate::Writable for Buf3WriteoffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF3_WRITEOFFSET to value 0"]
impl crate::Resettable for Buf3WriteoffsetSpec {}
