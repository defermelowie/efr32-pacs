#[doc = "Register `BUF0_CTRL` reader"]
pub type R = crate::R<Buf0CtrlSpec>;
#[doc = "Register `BUF0_CTRL` writer"]
pub type W = crate::W<Buf0CtrlSpec>;
#[doc = "Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "0: Sets Buffer size to 64 bytes"]
    Size64 = 0,
    #[doc = "1: Sets Buffer size to 128 bytes"]
    Size128 = 1,
    #[doc = "2: Sets Buffer size to 256 bytes"]
    Size256 = 2,
    #[doc = "3: Sets Buffer size to 512 bytes"]
    Size512 = 3,
    #[doc = "4: Sets Buffer size to 1024 bytes"]
    Size1024 = 4,
    #[doc = "5: Sets Buffer size to 2048 bytes"]
    Size2048 = 5,
    #[doc = "6: Sets Buffer size to 4096 bytes"]
    Size4096 = 6,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
impl crate::IsEnum for Size {}
#[doc = "Field `SIZE` reader - Buffer Size"]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size> {
        match self.bits {
            0 => Some(Size::Size64),
            1 => Some(Size::Size128),
            2 => Some(Size::Size256),
            3 => Some(Size::Size512),
            4 => Some(Size::Size1024),
            5 => Some(Size::Size2048),
            6 => Some(Size::Size4096),
            _ => None,
        }
    }
    #[doc = "Sets Buffer size to 64 bytes"]
    #[inline(always)]
    pub fn is_size64(&self) -> bool {
        *self == Size::Size64
    }
    #[doc = "Sets Buffer size to 128 bytes"]
    #[inline(always)]
    pub fn is_size128(&self) -> bool {
        *self == Size::Size128
    }
    #[doc = "Sets Buffer size to 256 bytes"]
    #[inline(always)]
    pub fn is_size256(&self) -> bool {
        *self == Size::Size256
    }
    #[doc = "Sets Buffer size to 512 bytes"]
    #[inline(always)]
    pub fn is_size512(&self) -> bool {
        *self == Size::Size512
    }
    #[doc = "Sets Buffer size to 1024 bytes"]
    #[inline(always)]
    pub fn is_size1024(&self) -> bool {
        *self == Size::Size1024
    }
    #[doc = "Sets Buffer size to 2048 bytes"]
    #[inline(always)]
    pub fn is_size2048(&self) -> bool {
        *self == Size::Size2048
    }
    #[doc = "Sets Buffer size to 4096 bytes"]
    #[inline(always)]
    pub fn is_size4096(&self) -> bool {
        *self == Size::Size4096
    }
}
#[doc = "Field `SIZE` writer - Buffer Size"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Size>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sets Buffer size to 64 bytes"]
    #[inline(always)]
    pub fn size64(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size64)
    }
    #[doc = "Sets Buffer size to 128 bytes"]
    #[inline(always)]
    pub fn size128(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size128)
    }
    #[doc = "Sets Buffer size to 256 bytes"]
    #[inline(always)]
    pub fn size256(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size256)
    }
    #[doc = "Sets Buffer size to 512 bytes"]
    #[inline(always)]
    pub fn size512(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size512)
    }
    #[doc = "Sets Buffer size to 1024 bytes"]
    #[inline(always)]
    pub fn size1024(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size1024)
    }
    #[doc = "Sets Buffer size to 2048 bytes"]
    #[inline(always)]
    pub fn size2048(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size2048)
    }
    #[doc = "Sets Buffer size to 4096 bytes"]
    #[inline(always)]
    pub fn size4096(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size4096)
    }
}
impl R {
    #[doc = "Bits 0:2 - Buffer Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Buffer Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, Buf0CtrlSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf0CtrlSpec;
impl crate::RegisterSpec for Buf0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf0_ctrl::R`](R) reader structure"]
impl crate::Readable for Buf0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`buf0_ctrl::W`](W) writer structure"]
impl crate::Writable for Buf0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF0_CTRL to value 0"]
impl crate::Resettable for Buf0CtrlSpec {}
