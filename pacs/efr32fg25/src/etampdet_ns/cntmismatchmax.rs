#[doc = "Register `CNTMISMATCHMAX` reader"]
pub type R = crate::R<CntmismatchmaxSpec>;
#[doc = "Register `CNTMISMATCHMAX` writer"]
pub type W = crate::W<CntmismatchmaxSpec>;
#[doc = "channel 0 filter counter threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chnlcntmismatchmax0 {
    #[doc = "0: Detect filter raise detect flag after seeing 1 event before reset counter expire"]
    DetectFilterThreshold1 = 0,
    #[doc = "1: Detect filter raise detect flag after seeing 2 events before reset counter expire"]
    DetectFilterThreshold2 = 1,
    #[doc = "2: Detect filter raise detect flag after seeing 3 events before reset counter expire"]
    DetectFilterThreshold3 = 2,
    #[doc = "3: Detect filter raise detect flag after seeing 4 events before reset counter expire"]
    DetectFilterThreshold4 = 3,
    #[doc = "4: Detect filter raise detect flag after seeing 5 events before reset counter expire"]
    DetectFilterThreshold5 = 4,
    #[doc = "5: Detect filter raise detect flag after seeing 6 events before reset counter expire"]
    DetectFilterThreshold6 = 5,
    #[doc = "6: Detect filter raise detect flag after seeing 7 events before reset counter expire"]
    DetectFilterThreshold7 = 6,
    #[doc = "7: Detect filter raise detect flag after seeing 8 events before reset counter expire"]
    DetectFilterThreshold8 = 7,
}
impl From<Chnlcntmismatchmax0> for u8 {
    #[inline(always)]
    fn from(variant: Chnlcntmismatchmax0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chnlcntmismatchmax0 {
    type Ux = u8;
}
impl crate::IsEnum for Chnlcntmismatchmax0 {}
#[doc = "Field `CHNLCNTMISMATCHMAX0` reader - channel 0 filter counter threshold"]
pub type Chnlcntmismatchmax0R = crate::FieldReader<Chnlcntmismatchmax0>;
impl Chnlcntmismatchmax0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnlcntmismatchmax0 {
        match self.bits {
            0 => Chnlcntmismatchmax0::DetectFilterThreshold1,
            1 => Chnlcntmismatchmax0::DetectFilterThreshold2,
            2 => Chnlcntmismatchmax0::DetectFilterThreshold3,
            3 => Chnlcntmismatchmax0::DetectFilterThreshold4,
            4 => Chnlcntmismatchmax0::DetectFilterThreshold5,
            5 => Chnlcntmismatchmax0::DetectFilterThreshold6,
            6 => Chnlcntmismatchmax0::DetectFilterThreshold7,
            7 => Chnlcntmismatchmax0::DetectFilterThreshold8,
            _ => unreachable!(),
        }
    }
    #[doc = "Detect filter raise detect flag after seeing 1 event before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold1(&self) -> bool {
        *self == Chnlcntmismatchmax0::DetectFilterThreshold1
    }
    #[doc = "Detect filter raise detect flag after seeing 2 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold2(&self) -> bool {
        *self == Chnlcntmismatchmax0::DetectFilterThreshold2
    }
    #[doc = "Detect filter raise detect flag after seeing 3 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold3(&self) -> bool {
        *self == Chnlcntmismatchmax0::DetectFilterThreshold3
    }
    #[doc = "Detect filter raise detect flag after seeing 4 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold4(&self) -> bool {
        *self == Chnlcntmismatchmax0::DetectFilterThreshold4
    }
    #[doc = "Detect filter raise detect flag after seeing 5 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold5(&self) -> bool {
        *self == Chnlcntmismatchmax0::DetectFilterThreshold5
    }
    #[doc = "Detect filter raise detect flag after seeing 6 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold6(&self) -> bool {
        *self == Chnlcntmismatchmax0::DetectFilterThreshold6
    }
    #[doc = "Detect filter raise detect flag after seeing 7 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold7(&self) -> bool {
        *self == Chnlcntmismatchmax0::DetectFilterThreshold7
    }
    #[doc = "Detect filter raise detect flag after seeing 8 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold8(&self) -> bool {
        *self == Chnlcntmismatchmax0::DetectFilterThreshold8
    }
}
#[doc = "Field `CHNLCNTMISMATCHMAX0` writer - channel 0 filter counter threshold"]
pub type Chnlcntmismatchmax0W<'a, REG> =
    crate::FieldWriter<'a, REG, 3, Chnlcntmismatchmax0, crate::Safe>;
impl<'a, REG> Chnlcntmismatchmax0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Detect filter raise detect flag after seeing 1 event before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold1(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax0::DetectFilterThreshold1)
    }
    #[doc = "Detect filter raise detect flag after seeing 2 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold2(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax0::DetectFilterThreshold2)
    }
    #[doc = "Detect filter raise detect flag after seeing 3 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold3(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax0::DetectFilterThreshold3)
    }
    #[doc = "Detect filter raise detect flag after seeing 4 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold4(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax0::DetectFilterThreshold4)
    }
    #[doc = "Detect filter raise detect flag after seeing 5 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold5(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax0::DetectFilterThreshold5)
    }
    #[doc = "Detect filter raise detect flag after seeing 6 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold6(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax0::DetectFilterThreshold6)
    }
    #[doc = "Detect filter raise detect flag after seeing 7 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold7(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax0::DetectFilterThreshold7)
    }
    #[doc = "Detect filter raise detect flag after seeing 8 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold8(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax0::DetectFilterThreshold8)
    }
}
#[doc = "channel 1 filter counter threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chnlcntmismatchmax1 {
    #[doc = "0: Detect filter raise detect flag after seeing 1 event before reset counter expire"]
    DetectFilterThreshold1 = 0,
    #[doc = "1: Detect filter raise detect flag after seeing 2 events before reset counter expire"]
    DetectFilterThreshold2 = 1,
    #[doc = "2: Detect filter raise detect flag after seeing 3 events before reset counter expire"]
    DetectFilterThreshold3 = 2,
    #[doc = "3: Detect filter raise detect flag after seeing 4 events before reset counter expire"]
    DetectFilterThreshold4 = 3,
    #[doc = "4: Detect filter raise detect flag after seeing 5 events before reset counter expire"]
    DetectFilterThreshold5 = 4,
    #[doc = "5: Detect filter raise detect flag after seeing 6 events before reset counter expire"]
    DetectFilterThreshold6 = 5,
    #[doc = "6: Detect filter raise detect flag after seeing 7 events before reset counter expire"]
    DetectFilterThreshold7 = 6,
    #[doc = "7: Detect filter raise detect flag after seeing 8 events before reset counter expire"]
    DetectFilterThreshold8 = 7,
}
impl From<Chnlcntmismatchmax1> for u8 {
    #[inline(always)]
    fn from(variant: Chnlcntmismatchmax1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chnlcntmismatchmax1 {
    type Ux = u8;
}
impl crate::IsEnum for Chnlcntmismatchmax1 {}
#[doc = "Field `CHNLCNTMISMATCHMAX1` reader - channel 1 filter counter threshold"]
pub type Chnlcntmismatchmax1R = crate::FieldReader<Chnlcntmismatchmax1>;
impl Chnlcntmismatchmax1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnlcntmismatchmax1 {
        match self.bits {
            0 => Chnlcntmismatchmax1::DetectFilterThreshold1,
            1 => Chnlcntmismatchmax1::DetectFilterThreshold2,
            2 => Chnlcntmismatchmax1::DetectFilterThreshold3,
            3 => Chnlcntmismatchmax1::DetectFilterThreshold4,
            4 => Chnlcntmismatchmax1::DetectFilterThreshold5,
            5 => Chnlcntmismatchmax1::DetectFilterThreshold6,
            6 => Chnlcntmismatchmax1::DetectFilterThreshold7,
            7 => Chnlcntmismatchmax1::DetectFilterThreshold8,
            _ => unreachable!(),
        }
    }
    #[doc = "Detect filter raise detect flag after seeing 1 event before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold1(&self) -> bool {
        *self == Chnlcntmismatchmax1::DetectFilterThreshold1
    }
    #[doc = "Detect filter raise detect flag after seeing 2 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold2(&self) -> bool {
        *self == Chnlcntmismatchmax1::DetectFilterThreshold2
    }
    #[doc = "Detect filter raise detect flag after seeing 3 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold3(&self) -> bool {
        *self == Chnlcntmismatchmax1::DetectFilterThreshold3
    }
    #[doc = "Detect filter raise detect flag after seeing 4 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold4(&self) -> bool {
        *self == Chnlcntmismatchmax1::DetectFilterThreshold4
    }
    #[doc = "Detect filter raise detect flag after seeing 5 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold5(&self) -> bool {
        *self == Chnlcntmismatchmax1::DetectFilterThreshold5
    }
    #[doc = "Detect filter raise detect flag after seeing 6 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold6(&self) -> bool {
        *self == Chnlcntmismatchmax1::DetectFilterThreshold6
    }
    #[doc = "Detect filter raise detect flag after seeing 7 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold7(&self) -> bool {
        *self == Chnlcntmismatchmax1::DetectFilterThreshold7
    }
    #[doc = "Detect filter raise detect flag after seeing 8 events before reset counter expire"]
    #[inline(always)]
    pub fn is_detect_filter_threshold8(&self) -> bool {
        *self == Chnlcntmismatchmax1::DetectFilterThreshold8
    }
}
#[doc = "Field `CHNLCNTMISMATCHMAX1` writer - channel 1 filter counter threshold"]
pub type Chnlcntmismatchmax1W<'a, REG> =
    crate::FieldWriter<'a, REG, 3, Chnlcntmismatchmax1, crate::Safe>;
impl<'a, REG> Chnlcntmismatchmax1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Detect filter raise detect flag after seeing 1 event before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold1(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax1::DetectFilterThreshold1)
    }
    #[doc = "Detect filter raise detect flag after seeing 2 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold2(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax1::DetectFilterThreshold2)
    }
    #[doc = "Detect filter raise detect flag after seeing 3 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold3(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax1::DetectFilterThreshold3)
    }
    #[doc = "Detect filter raise detect flag after seeing 4 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold4(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax1::DetectFilterThreshold4)
    }
    #[doc = "Detect filter raise detect flag after seeing 5 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold5(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax1::DetectFilterThreshold5)
    }
    #[doc = "Detect filter raise detect flag after seeing 6 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold6(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax1::DetectFilterThreshold6)
    }
    #[doc = "Detect filter raise detect flag after seeing 7 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold7(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax1::DetectFilterThreshold7)
    }
    #[doc = "Detect filter raise detect flag after seeing 8 events before reset counter expire"]
    #[inline(always)]
    pub fn detect_filter_threshold8(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcntmismatchmax1::DetectFilterThreshold8)
    }
}
impl R {
    #[doc = "Bits 0:2 - channel 0 filter counter threshold"]
    #[inline(always)]
    pub fn chnlcntmismatchmax0(&self) -> Chnlcntmismatchmax0R {
        Chnlcntmismatchmax0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - channel 1 filter counter threshold"]
    #[inline(always)]
    pub fn chnlcntmismatchmax1(&self) -> Chnlcntmismatchmax1R {
        Chnlcntmismatchmax1R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - channel 0 filter counter threshold"]
    #[inline(always)]
    pub fn chnlcntmismatchmax0(&mut self) -> Chnlcntmismatchmax0W<'_, CntmismatchmaxSpec> {
        Chnlcntmismatchmax0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - channel 1 filter counter threshold"]
    #[inline(always)]
    pub fn chnlcntmismatchmax1(&mut self) -> Chnlcntmismatchmax1W<'_, CntmismatchmaxSpec> {
        Chnlcntmismatchmax1W::new(self, 3)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntmismatchmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntmismatchmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntmismatchmaxSpec;
impl crate::RegisterSpec for CntmismatchmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntmismatchmax::R`](R) reader structure"]
impl crate::Readable for CntmismatchmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`cntmismatchmax::W`](W) writer structure"]
impl crate::Writable for CntmismatchmaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTMISMATCHMAX to value 0"]
impl crate::Resettable for CntmismatchmaxSpec {}
