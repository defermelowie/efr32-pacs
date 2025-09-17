#[doc = "Register `DCOCTRL` reader"]
pub type R = crate::R<DcoctrlSpec>;
#[doc = "Register `DCOCTRL` writer"]
pub type W = crate::W<DcoctrlSpec>;
#[doc = "DCO Half bias\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcobiashalf {
    #[doc = "0: DISABLE"]
    Disable = 0,
    #[doc = "1: ENABLE"]
    Enable = 1,
}
impl From<Dcobiashalf> for bool {
    #[inline(always)]
    fn from(variant: Dcobiashalf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCOBIASHALF` reader - DCO Half bias"]
pub type DcobiashalfR = crate::BitReader<Dcobiashalf>;
impl DcobiashalfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcobiashalf {
        match self.bits {
            false => Dcobiashalf::Disable,
            true => Dcobiashalf::Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dcobiashalf::Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dcobiashalf::Enable
    }
}
#[doc = "Field `DCOBIASHALF` writer - DCO Half bias"]
pub type DcobiashalfW<'a, REG> = crate::BitWriter<'a, REG, Dcobiashalf>;
impl<'a, REG> DcobiashalfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dcobiashalf::Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dcobiashalf::Enable)
    }
}
impl R {
    #[doc = "Bit 9 - DCO Half bias"]
    #[inline(always)]
    pub fn dcobiashalf(&self) -> DcobiashalfR {
        DcobiashalfR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - DCO Half bias"]
    #[inline(always)]
    pub fn dcobiashalf(&mut self) -> DcobiashalfW<'_, DcoctrlSpec> {
        DcobiashalfW::new(self, 9)
    }
}
#[doc = "DAC Controlled Oscillator\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcoctrlSpec;
impl crate::RegisterSpec for DcoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcoctrl::R`](R) reader structure"]
impl crate::Readable for DcoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcoctrl::W`](W) writer structure"]
impl crate::Writable for DcoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCOCTRL to value 0x0006_0e00"]
impl crate::Resettable for DcoctrlSpec {
    const RESET_VALUE: u32 = 0x0006_0e00;
}
