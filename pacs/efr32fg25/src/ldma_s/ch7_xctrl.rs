#[doc = "Register `CH7_XCTRL` reader"]
pub type R = crate::R<Ch7XctrlSpec>;
#[doc = "Register `CH7_XCTRL` writer"]
pub type W = crate::W<Ch7XctrlSpec>;
#[doc = "Field `DSTILEN` reader - Destination Interleave"]
pub type DstilenR = crate::BitReader;
#[doc = "Field `DSTILEN` writer - Destination Interleave"]
pub type DstilenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interleave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ilmode {
    #[doc = "0: Address determined by value in rules. Size of WORD"]
    Absolute = 0,
    #[doc = "1: Address determined by adding rules to DST. Size of HALFWORD"]
    Relative16 = 1,
    #[doc = "2: Address determined by adding rules to DST. Size of BYTE"]
    Relative8 = 2,
}
impl From<Ilmode> for u8 {
    #[inline(always)]
    fn from(variant: Ilmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ilmode {
    type Ux = u8;
}
impl crate::IsEnum for Ilmode {}
#[doc = "Field `ILMODE` reader - Interleave Mode"]
pub type IlmodeR = crate::FieldReader<Ilmode>;
impl IlmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ilmode> {
        match self.bits {
            0 => Some(Ilmode::Absolute),
            1 => Some(Ilmode::Relative16),
            2 => Some(Ilmode::Relative8),
            _ => None,
        }
    }
    #[doc = "Address determined by value in rules. Size of WORD"]
    #[inline(always)]
    pub fn is_absolute(&self) -> bool {
        *self == Ilmode::Absolute
    }
    #[doc = "Address determined by adding rules to DST. Size of HALFWORD"]
    #[inline(always)]
    pub fn is_relative16(&self) -> bool {
        *self == Ilmode::Relative16
    }
    #[doc = "Address determined by adding rules to DST. Size of BYTE"]
    #[inline(always)]
    pub fn is_relative8(&self) -> bool {
        *self == Ilmode::Relative8
    }
}
#[doc = "Field `ILMODE` writer - Interleave Mode"]
pub type IlmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ilmode>;
impl<'a, REG> IlmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address determined by value in rules. Size of WORD"]
    #[inline(always)]
    pub fn absolute(self) -> &'a mut crate::W<REG> {
        self.variant(Ilmode::Absolute)
    }
    #[doc = "Address determined by adding rules to DST. Size of HALFWORD"]
    #[inline(always)]
    pub fn relative16(self) -> &'a mut crate::W<REG> {
        self.variant(Ilmode::Relative16)
    }
    #[doc = "Address determined by adding rules to DST. Size of BYTE"]
    #[inline(always)]
    pub fn relative8(self) -> &'a mut crate::W<REG> {
        self.variant(Ilmode::Relative8)
    }
}
#[doc = "Field `BUFFERABLE` reader - Allow AHB buffering"]
pub type BufferableR = crate::BitReader;
#[doc = "Field `BUFFERABLE` writer - Allow AHB buffering"]
pub type BufferableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Destination Interleave"]
    #[inline(always)]
    pub fn dstilen(&self) -> DstilenR {
        DstilenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Interleave Mode"]
    #[inline(always)]
    pub fn ilmode(&self) -> IlmodeR {
        IlmodeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Allow AHB buffering"]
    #[inline(always)]
    pub fn bufferable(&self) -> BufferableR {
        BufferableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Destination Interleave"]
    #[inline(always)]
    pub fn dstilen(&mut self) -> DstilenW<'_, Ch7XctrlSpec> {
        DstilenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Interleave Mode"]
    #[inline(always)]
    pub fn ilmode(&mut self) -> IlmodeW<'_, Ch7XctrlSpec> {
        IlmodeW::new(self, 5)
    }
    #[doc = "Bit 7 - Allow AHB buffering"]
    #[inline(always)]
    pub fn bufferable(&mut self) -> BufferableW<'_, Ch7XctrlSpec> {
        BufferableW::new(self, 7)
    }
}
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_xctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_xctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7XctrlSpec;
impl crate::RegisterSpec for Ch7XctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_xctrl::R`](R) reader structure"]
impl crate::Readable for Ch7XctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7_xctrl::W`](W) writer structure"]
impl crate::Writable for Ch7XctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7_XCTRL to value 0"]
impl crate::Resettable for Ch7XctrlSpec {}
