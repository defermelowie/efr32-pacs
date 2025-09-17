#[doc = "Register `BUF0_THRESHOLDCTRL` reader"]
pub type R = crate::R<Buf0ThresholdctrlSpec>;
#[doc = "Register `BUF0_THRESHOLDCTRL` writer"]
pub type W = crate::W<Buf0ThresholdctrlSpec>;
#[doc = "Field `THRESHOLD` reader - Buffer Threshold Value"]
pub type ThresholdR = crate::FieldReader<u16>;
#[doc = "Field `THRESHOLD` writer - Buffer Threshold Value"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Buffer Threshold Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thresholdmode {
    #[doc = "0: THRESHOLDIF will be set if BYTES is larger than THRESHOLD"]
    Larger = 0,
    #[doc = "1: THRESHOLDIF will be set if BYTES is less than or equal to THRESHOLD"]
    Lessorequal = 1,
}
impl From<Thresholdmode> for bool {
    #[inline(always)]
    fn from(variant: Thresholdmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THRESHOLDMODE` reader - Buffer Threshold Mode"]
pub type ThresholdmodeR = crate::BitReader<Thresholdmode>;
impl ThresholdmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thresholdmode {
        match self.bits {
            false => Thresholdmode::Larger,
            true => Thresholdmode::Lessorequal,
        }
    }
    #[doc = "THRESHOLDIF will be set if BYTES is larger than THRESHOLD"]
    #[inline(always)]
    pub fn is_larger(&self) -> bool {
        *self == Thresholdmode::Larger
    }
    #[doc = "THRESHOLDIF will be set if BYTES is less than or equal to THRESHOLD"]
    #[inline(always)]
    pub fn is_lessorequal(&self) -> bool {
        *self == Thresholdmode::Lessorequal
    }
}
#[doc = "Field `THRESHOLDMODE` writer - Buffer Threshold Mode"]
pub type ThresholdmodeW<'a, REG> = crate::BitWriter<'a, REG, Thresholdmode>;
impl<'a, REG> ThresholdmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "THRESHOLDIF will be set if BYTES is larger than THRESHOLD"]
    #[inline(always)]
    pub fn larger(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdmode::Larger)
    }
    #[doc = "THRESHOLDIF will be set if BYTES is less than or equal to THRESHOLD"]
    #[inline(always)]
    pub fn lessorequal(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdmode::Lessorequal)
    }
}
impl R {
    #[doc = "Bits 0:12 - Buffer Threshold Value"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Buffer Threshold Mode"]
    #[inline(always)]
    pub fn thresholdmode(&self) -> ThresholdmodeR {
        ThresholdmodeR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Buffer Threshold Value"]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, Buf0ThresholdctrlSpec> {
        ThresholdW::new(self, 0)
    }
    #[doc = "Bit 13 - Buffer Threshold Mode"]
    #[inline(always)]
    pub fn thresholdmode(&mut self) -> ThresholdmodeW<'_, Buf0ThresholdctrlSpec> {
        ThresholdmodeW::new(self, 13)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_thresholdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_thresholdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf0ThresholdctrlSpec;
impl crate::RegisterSpec for Buf0ThresholdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf0_thresholdctrl::R`](R) reader structure"]
impl crate::Readable for Buf0ThresholdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`buf0_thresholdctrl::W`](W) writer structure"]
impl crate::Writable for Buf0ThresholdctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF0_THRESHOLDCTRL to value 0"]
impl crate::Resettable for Buf0ThresholdctrlSpec {}
