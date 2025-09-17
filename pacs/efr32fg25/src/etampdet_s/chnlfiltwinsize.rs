#[doc = "Register `CHNLFILTWINSIZE` reader"]
pub type R = crate::R<ChnlfiltwinsizeSpec>;
#[doc = "Register `CHNLFILTWINSIZE` writer"]
pub type W = crate::W<ChnlfiltwinsizeSpec>;
#[doc = "channel 0 Filter moving window size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chnlfiltwinsize0 {
    #[doc = "1: Detect filter moving window size is 2"]
    DetectFilterMovingWinSize2 = 1,
    #[doc = "2: Detect filter moving window size is 3"]
    DetectFilterMovingWinSize3 = 2,
    #[doc = "3: Detect filter moving window size is 4"]
    DetectFilterMovingWinSize4 = 3,
    #[doc = "4: Detect filter moving window size is 5"]
    DetectFilterMovingWinSize5 = 4,
    #[doc = "5: Detect filter moving window size is 6"]
    DetectFilterMovingWinSize6 = 5,
    #[doc = "6: Detect filter moving window size is 7"]
    DetectFilterMovingWinSize7 = 6,
    #[doc = "7: Detect filter moving window size is 8"]
    DetectFilterMovingWinSize8 = 7,
    #[doc = "8: Detect filter moving window size is 9"]
    DetectFilterMovingWinSize9 = 8,
    #[doc = "9: Detect filter moving window size is 10"]
    DetectFilterMovingWinSize10 = 9,
    #[doc = "10: Detect filter moving window size is 11"]
    DetectFilterMovingWinSize11 = 10,
    #[doc = "11: Detect filter moving window size is 12"]
    DetectFilterMovingWinSize12 = 11,
    #[doc = "12: Detect filter moving window size is 13"]
    DetectFilterMovingWinSize13 = 12,
    #[doc = "13: Detect filter moving window size is 14"]
    DetectFilterMovingWinSize14 = 13,
    #[doc = "14: Detect filter moving window size is 15"]
    DetectFilterMovingWinSize15 = 14,
    #[doc = "15: Detect filter moving window size is 16"]
    DetectFilterMovingWinSize16 = 15,
}
impl From<Chnlfiltwinsize0> for u8 {
    #[inline(always)]
    fn from(variant: Chnlfiltwinsize0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chnlfiltwinsize0 {
    type Ux = u8;
}
impl crate::IsEnum for Chnlfiltwinsize0 {}
#[doc = "Field `CHNLFILTWINSIZE0` reader - channel 0 Filter moving window size"]
pub type Chnlfiltwinsize0R = crate::FieldReader<Chnlfiltwinsize0>;
impl Chnlfiltwinsize0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnlfiltwinsize0 {
        match self.bits {
            1 => Chnlfiltwinsize0::DetectFilterMovingWinSize2,
            2 => Chnlfiltwinsize0::DetectFilterMovingWinSize3,
            3 => Chnlfiltwinsize0::DetectFilterMovingWinSize4,
            4 => Chnlfiltwinsize0::DetectFilterMovingWinSize5,
            5 => Chnlfiltwinsize0::DetectFilterMovingWinSize6,
            6 => Chnlfiltwinsize0::DetectFilterMovingWinSize7,
            7 => Chnlfiltwinsize0::DetectFilterMovingWinSize8,
            8 => Chnlfiltwinsize0::DetectFilterMovingWinSize9,
            9 => Chnlfiltwinsize0::DetectFilterMovingWinSize10,
            10 => Chnlfiltwinsize0::DetectFilterMovingWinSize11,
            11 => Chnlfiltwinsize0::DetectFilterMovingWinSize12,
            12 => Chnlfiltwinsize0::DetectFilterMovingWinSize13,
            13 => Chnlfiltwinsize0::DetectFilterMovingWinSize14,
            14 => Chnlfiltwinsize0::DetectFilterMovingWinSize15,
            15 => Chnlfiltwinsize0::DetectFilterMovingWinSize16,
            _ => unreachable!(),
        }
    }
    #[doc = "Detect filter moving window size is 2"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size2(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize2
    }
    #[doc = "Detect filter moving window size is 3"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size3(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize3
    }
    #[doc = "Detect filter moving window size is 4"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size4(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize4
    }
    #[doc = "Detect filter moving window size is 5"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size5(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize5
    }
    #[doc = "Detect filter moving window size is 6"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size6(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize6
    }
    #[doc = "Detect filter moving window size is 7"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size7(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize7
    }
    #[doc = "Detect filter moving window size is 8"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size8(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize8
    }
    #[doc = "Detect filter moving window size is 9"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size9(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize9
    }
    #[doc = "Detect filter moving window size is 10"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size10(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize10
    }
    #[doc = "Detect filter moving window size is 11"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size11(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize11
    }
    #[doc = "Detect filter moving window size is 12"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size12(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize12
    }
    #[doc = "Detect filter moving window size is 13"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size13(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize13
    }
    #[doc = "Detect filter moving window size is 14"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size14(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize14
    }
    #[doc = "Detect filter moving window size is 15"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size15(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize15
    }
    #[doc = "Detect filter moving window size is 16"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size16(&self) -> bool {
        *self == Chnlfiltwinsize0::DetectFilterMovingWinSize16
    }
}
#[doc = "Field `CHNLFILTWINSIZE0` writer - channel 0 Filter moving window size"]
pub type Chnlfiltwinsize0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chnlfiltwinsize0>;
impl<'a, REG> Chnlfiltwinsize0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Detect filter moving window size is 2"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size2(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize2)
    }
    #[doc = "Detect filter moving window size is 3"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size3(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize3)
    }
    #[doc = "Detect filter moving window size is 4"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size4(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize4)
    }
    #[doc = "Detect filter moving window size is 5"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size5(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize5)
    }
    #[doc = "Detect filter moving window size is 6"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size6(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize6)
    }
    #[doc = "Detect filter moving window size is 7"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size7(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize7)
    }
    #[doc = "Detect filter moving window size is 8"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size8(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize8)
    }
    #[doc = "Detect filter moving window size is 9"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size9(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize9)
    }
    #[doc = "Detect filter moving window size is 10"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size10(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize10)
    }
    #[doc = "Detect filter moving window size is 11"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size11(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize11)
    }
    #[doc = "Detect filter moving window size is 12"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size12(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize12)
    }
    #[doc = "Detect filter moving window size is 13"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size13(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize13)
    }
    #[doc = "Detect filter moving window size is 14"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size14(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize14)
    }
    #[doc = "Detect filter moving window size is 15"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size15(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize15)
    }
    #[doc = "Detect filter moving window size is 16"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size16(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize0::DetectFilterMovingWinSize16)
    }
}
#[doc = "channel 1 Filter moving window size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chnlfiltwinsize1 {
    #[doc = "1: Detect filter moving window size is 2"]
    DetectFilterMovingWinSize2 = 1,
    #[doc = "2: Detect filter moving window size is 3"]
    DetectFilterMovingWinSize3 = 2,
    #[doc = "3: Detect filter moving window size is 4"]
    DetectFilterMovingWinSize4 = 3,
    #[doc = "4: Detect filter moving window size is 5"]
    DetectFilterMovingWinSize5 = 4,
    #[doc = "5: Detect filter moving window size is 6"]
    DetectFilterMovingWinSize6 = 5,
    #[doc = "6: Detect filter moving window size is 7"]
    DetectFilterMovingWinSize7 = 6,
    #[doc = "7: Detect filter moving window size is 8"]
    DetectFilterMovingWinSize8 = 7,
    #[doc = "8: Detect filter moving window size is 9"]
    DetectFilterMovingWinSize9 = 8,
    #[doc = "9: Detect filter moving window size is 10"]
    DetectFilterMovingWinSize10 = 9,
    #[doc = "10: Detect filter moving window size is 11"]
    DetectFilterMovingWinSize11 = 10,
    #[doc = "11: Detect filter moving window size is 12"]
    DetectFilterMovingWinSize12 = 11,
    #[doc = "12: Detect filter moving window size is 13"]
    DetectFilterMovingWinSize13 = 12,
    #[doc = "13: Detect filter moving window size is 14"]
    DetectFilterMovingWinSize14 = 13,
    #[doc = "14: Detect filter moving window size is 15"]
    DetectFilterMovingWinSize15 = 14,
    #[doc = "15: Detect filter moving window size is 16"]
    DetectFilterMovingWinSize16 = 15,
}
impl From<Chnlfiltwinsize1> for u8 {
    #[inline(always)]
    fn from(variant: Chnlfiltwinsize1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chnlfiltwinsize1 {
    type Ux = u8;
}
impl crate::IsEnum for Chnlfiltwinsize1 {}
#[doc = "Field `CHNLFILTWINSIZE1` reader - channel 1 Filter moving window size"]
pub type Chnlfiltwinsize1R = crate::FieldReader<Chnlfiltwinsize1>;
impl Chnlfiltwinsize1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnlfiltwinsize1 {
        match self.bits {
            1 => Chnlfiltwinsize1::DetectFilterMovingWinSize2,
            2 => Chnlfiltwinsize1::DetectFilterMovingWinSize3,
            3 => Chnlfiltwinsize1::DetectFilterMovingWinSize4,
            4 => Chnlfiltwinsize1::DetectFilterMovingWinSize5,
            5 => Chnlfiltwinsize1::DetectFilterMovingWinSize6,
            6 => Chnlfiltwinsize1::DetectFilterMovingWinSize7,
            7 => Chnlfiltwinsize1::DetectFilterMovingWinSize8,
            8 => Chnlfiltwinsize1::DetectFilterMovingWinSize9,
            9 => Chnlfiltwinsize1::DetectFilterMovingWinSize10,
            10 => Chnlfiltwinsize1::DetectFilterMovingWinSize11,
            11 => Chnlfiltwinsize1::DetectFilterMovingWinSize12,
            12 => Chnlfiltwinsize1::DetectFilterMovingWinSize13,
            13 => Chnlfiltwinsize1::DetectFilterMovingWinSize14,
            14 => Chnlfiltwinsize1::DetectFilterMovingWinSize15,
            15 => Chnlfiltwinsize1::DetectFilterMovingWinSize16,
            _ => unreachable!(),
        }
    }
    #[doc = "Detect filter moving window size is 2"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size2(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize2
    }
    #[doc = "Detect filter moving window size is 3"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size3(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize3
    }
    #[doc = "Detect filter moving window size is 4"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size4(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize4
    }
    #[doc = "Detect filter moving window size is 5"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size5(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize5
    }
    #[doc = "Detect filter moving window size is 6"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size6(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize6
    }
    #[doc = "Detect filter moving window size is 7"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size7(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize7
    }
    #[doc = "Detect filter moving window size is 8"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size8(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize8
    }
    #[doc = "Detect filter moving window size is 9"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size9(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize9
    }
    #[doc = "Detect filter moving window size is 10"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size10(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize10
    }
    #[doc = "Detect filter moving window size is 11"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size11(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize11
    }
    #[doc = "Detect filter moving window size is 12"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size12(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize12
    }
    #[doc = "Detect filter moving window size is 13"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size13(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize13
    }
    #[doc = "Detect filter moving window size is 14"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size14(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize14
    }
    #[doc = "Detect filter moving window size is 15"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size15(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize15
    }
    #[doc = "Detect filter moving window size is 16"]
    #[inline(always)]
    pub fn is_detect_filter_moving_win_size16(&self) -> bool {
        *self == Chnlfiltwinsize1::DetectFilterMovingWinSize16
    }
}
#[doc = "Field `CHNLFILTWINSIZE1` writer - channel 1 Filter moving window size"]
pub type Chnlfiltwinsize1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Chnlfiltwinsize1>;
impl<'a, REG> Chnlfiltwinsize1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Detect filter moving window size is 2"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size2(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize2)
    }
    #[doc = "Detect filter moving window size is 3"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size3(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize3)
    }
    #[doc = "Detect filter moving window size is 4"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size4(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize4)
    }
    #[doc = "Detect filter moving window size is 5"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size5(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize5)
    }
    #[doc = "Detect filter moving window size is 6"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size6(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize6)
    }
    #[doc = "Detect filter moving window size is 7"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size7(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize7)
    }
    #[doc = "Detect filter moving window size is 8"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size8(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize8)
    }
    #[doc = "Detect filter moving window size is 9"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size9(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize9)
    }
    #[doc = "Detect filter moving window size is 10"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size10(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize10)
    }
    #[doc = "Detect filter moving window size is 11"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size11(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize11)
    }
    #[doc = "Detect filter moving window size is 12"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size12(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize12)
    }
    #[doc = "Detect filter moving window size is 13"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size13(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize13)
    }
    #[doc = "Detect filter moving window size is 14"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size14(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize14)
    }
    #[doc = "Detect filter moving window size is 15"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size15(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize15)
    }
    #[doc = "Detect filter moving window size is 16"]
    #[inline(always)]
    pub fn detect_filter_moving_win_size16(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlfiltwinsize1::DetectFilterMovingWinSize16)
    }
}
impl R {
    #[doc = "Bits 0:3 - channel 0 Filter moving window size"]
    #[inline(always)]
    pub fn chnlfiltwinsize0(&self) -> Chnlfiltwinsize0R {
        Chnlfiltwinsize0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - channel 1 Filter moving window size"]
    #[inline(always)]
    pub fn chnlfiltwinsize1(&self) -> Chnlfiltwinsize1R {
        Chnlfiltwinsize1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - channel 0 Filter moving window size"]
    #[inline(always)]
    pub fn chnlfiltwinsize0(&mut self) -> Chnlfiltwinsize0W<'_, ChnlfiltwinsizeSpec> {
        Chnlfiltwinsize0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - channel 1 Filter moving window size"]
    #[inline(always)]
    pub fn chnlfiltwinsize1(&mut self) -> Chnlfiltwinsize1W<'_, ChnlfiltwinsizeSpec> {
        Chnlfiltwinsize1W::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlfiltwinsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlfiltwinsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlfiltwinsizeSpec;
impl crate::RegisterSpec for ChnlfiltwinsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnlfiltwinsize::R`](R) reader structure"]
impl crate::Readable for ChnlfiltwinsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`chnlfiltwinsize::W`](W) writer structure"]
impl crate::Writable for ChnlfiltwinsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLFILTWINSIZE to value 0"]
impl crate::Resettable for ChnlfiltwinsizeSpec {}
