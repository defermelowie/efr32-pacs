#[doc = "Register `RFIMDCDCCTRL1` reader"]
pub type R = crate::R<Rfimdcdcctrl1Spec>;
#[doc = "Register `RFIMDCDCCTRL1` writer"]
pub type W = crate::W<Rfimdcdcctrl1Spec>;
#[doc = "Field `DCDCDIVEN` reader - DCDC DIV Enable"]
pub type DcdcdivenR = crate::BitReader;
#[doc = "Field `DCDCDIVEN` writer - DCDC DIV Enable"]
pub type DcdcdivenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCDIVINVEN` reader - DCDC DIV Inverter Enable"]
pub type DcdcdivinvenR = crate::BitReader;
#[doc = "Field `DCDCDIVINVEN` writer - DCDC DIV Inverter Enable"]
pub type DcdcdivinvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DCDC DIV Ratio\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcdcdivratio {
    #[doc = "0: Dividing master_rf clk by 8, D=50%"]
    Divratio8 = 0,
    #[doc = "1: Dividing master_rf clk by 9, D=44.4%"]
    Divratio9 = 1,
    #[doc = "2: Dividing master_rf clk by 10, D=40%"]
    Divratio10 = 2,
    #[doc = "3: Dividing master_rf clk by 11, D=36.4%"]
    Divratio11 = 3,
    #[doc = "4: Dividing master_rf clk by 12, D=50%"]
    Divratio12 = 4,
    #[doc = "5: Dividing master_rf clk by 13, D=46.2%"]
    Divratio13 = 5,
    #[doc = "6: Dividing master_rf clk by 14, D=42.9%"]
    Divratio14 = 6,
    #[doc = "7: Dividing master_rf clk by 15, D=40%"]
    Divratio15 = 7,
    #[doc = "8: Dividing master_rf clk by 16, D=50%"]
    Divratio16 = 8,
    #[doc = "9: Dividing master_rf clk by 17, D=47.1%"]
    Divratio17 = 9,
    #[doc = "10: Dividing master_rf clk by 18, D=44.4%"]
    Divratio18 = 10,
    #[doc = "11: Dividing master_rf clk by 19, D=42.1%"]
    Divratio19 = 11,
    #[doc = "12: Dividing master_rf clk by 20, D=60%"]
    Divratio20 = 12,
    #[doc = "13: Dividing master_rf clk by 21, D=57.1%"]
    Divratio21 = 13,
    #[doc = "14: Dividing master_rf clk by 22, D=54.5%"]
    Divratio22 = 14,
    #[doc = "15: Dividing master_rf clk by 23, D=52.2%"]
    Divratio23 = 15,
}
impl From<Dcdcdivratio> for u8 {
    #[inline(always)]
    fn from(variant: Dcdcdivratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcdcdivratio {
    type Ux = u8;
}
impl crate::IsEnum for Dcdcdivratio {}
#[doc = "Field `DCDCDIVRATIO` reader - DCDC DIV Ratio"]
pub type DcdcdivratioR = crate::FieldReader<Dcdcdivratio>;
impl DcdcdivratioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcdcdivratio {
        match self.bits {
            0 => Dcdcdivratio::Divratio8,
            1 => Dcdcdivratio::Divratio9,
            2 => Dcdcdivratio::Divratio10,
            3 => Dcdcdivratio::Divratio11,
            4 => Dcdcdivratio::Divratio12,
            5 => Dcdcdivratio::Divratio13,
            6 => Dcdcdivratio::Divratio14,
            7 => Dcdcdivratio::Divratio15,
            8 => Dcdcdivratio::Divratio16,
            9 => Dcdcdivratio::Divratio17,
            10 => Dcdcdivratio::Divratio18,
            11 => Dcdcdivratio::Divratio19,
            12 => Dcdcdivratio::Divratio20,
            13 => Dcdcdivratio::Divratio21,
            14 => Dcdcdivratio::Divratio22,
            15 => Dcdcdivratio::Divratio23,
            _ => unreachable!(),
        }
    }
    #[doc = "Dividing master_rf clk by 8, D=50%"]
    #[inline(always)]
    pub fn is_divratio8(&self) -> bool {
        *self == Dcdcdivratio::Divratio8
    }
    #[doc = "Dividing master_rf clk by 9, D=44.4%"]
    #[inline(always)]
    pub fn is_divratio9(&self) -> bool {
        *self == Dcdcdivratio::Divratio9
    }
    #[doc = "Dividing master_rf clk by 10, D=40%"]
    #[inline(always)]
    pub fn is_divratio10(&self) -> bool {
        *self == Dcdcdivratio::Divratio10
    }
    #[doc = "Dividing master_rf clk by 11, D=36.4%"]
    #[inline(always)]
    pub fn is_divratio11(&self) -> bool {
        *self == Dcdcdivratio::Divratio11
    }
    #[doc = "Dividing master_rf clk by 12, D=50%"]
    #[inline(always)]
    pub fn is_divratio12(&self) -> bool {
        *self == Dcdcdivratio::Divratio12
    }
    #[doc = "Dividing master_rf clk by 13, D=46.2%"]
    #[inline(always)]
    pub fn is_divratio13(&self) -> bool {
        *self == Dcdcdivratio::Divratio13
    }
    #[doc = "Dividing master_rf clk by 14, D=42.9%"]
    #[inline(always)]
    pub fn is_divratio14(&self) -> bool {
        *self == Dcdcdivratio::Divratio14
    }
    #[doc = "Dividing master_rf clk by 15, D=40%"]
    #[inline(always)]
    pub fn is_divratio15(&self) -> bool {
        *self == Dcdcdivratio::Divratio15
    }
    #[doc = "Dividing master_rf clk by 16, D=50%"]
    #[inline(always)]
    pub fn is_divratio16(&self) -> bool {
        *self == Dcdcdivratio::Divratio16
    }
    #[doc = "Dividing master_rf clk by 17, D=47.1%"]
    #[inline(always)]
    pub fn is_divratio17(&self) -> bool {
        *self == Dcdcdivratio::Divratio17
    }
    #[doc = "Dividing master_rf clk by 18, D=44.4%"]
    #[inline(always)]
    pub fn is_divratio18(&self) -> bool {
        *self == Dcdcdivratio::Divratio18
    }
    #[doc = "Dividing master_rf clk by 19, D=42.1%"]
    #[inline(always)]
    pub fn is_divratio19(&self) -> bool {
        *self == Dcdcdivratio::Divratio19
    }
    #[doc = "Dividing master_rf clk by 20, D=60%"]
    #[inline(always)]
    pub fn is_divratio20(&self) -> bool {
        *self == Dcdcdivratio::Divratio20
    }
    #[doc = "Dividing master_rf clk by 21, D=57.1%"]
    #[inline(always)]
    pub fn is_divratio21(&self) -> bool {
        *self == Dcdcdivratio::Divratio21
    }
    #[doc = "Dividing master_rf clk by 22, D=54.5%"]
    #[inline(always)]
    pub fn is_divratio22(&self) -> bool {
        *self == Dcdcdivratio::Divratio22
    }
    #[doc = "Dividing master_rf clk by 23, D=52.2%"]
    #[inline(always)]
    pub fn is_divratio23(&self) -> bool {
        *self == Dcdcdivratio::Divratio23
    }
}
#[doc = "Field `DCDCDIVRATIO` writer - DCDC DIV Ratio"]
pub type DcdcdivratioW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dcdcdivratio, crate::Safe>;
impl<'a, REG> DcdcdivratioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dividing master_rf clk by 8, D=50%"]
    #[inline(always)]
    pub fn divratio8(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio8)
    }
    #[doc = "Dividing master_rf clk by 9, D=44.4%"]
    #[inline(always)]
    pub fn divratio9(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio9)
    }
    #[doc = "Dividing master_rf clk by 10, D=40%"]
    #[inline(always)]
    pub fn divratio10(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio10)
    }
    #[doc = "Dividing master_rf clk by 11, D=36.4%"]
    #[inline(always)]
    pub fn divratio11(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio11)
    }
    #[doc = "Dividing master_rf clk by 12, D=50%"]
    #[inline(always)]
    pub fn divratio12(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio12)
    }
    #[doc = "Dividing master_rf clk by 13, D=46.2%"]
    #[inline(always)]
    pub fn divratio13(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio13)
    }
    #[doc = "Dividing master_rf clk by 14, D=42.9%"]
    #[inline(always)]
    pub fn divratio14(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio14)
    }
    #[doc = "Dividing master_rf clk by 15, D=40%"]
    #[inline(always)]
    pub fn divratio15(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio15)
    }
    #[doc = "Dividing master_rf clk by 16, D=50%"]
    #[inline(always)]
    pub fn divratio16(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio16)
    }
    #[doc = "Dividing master_rf clk by 17, D=47.1%"]
    #[inline(always)]
    pub fn divratio17(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio17)
    }
    #[doc = "Dividing master_rf clk by 18, D=44.4%"]
    #[inline(always)]
    pub fn divratio18(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio18)
    }
    #[doc = "Dividing master_rf clk by 19, D=42.1%"]
    #[inline(always)]
    pub fn divratio19(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio19)
    }
    #[doc = "Dividing master_rf clk by 20, D=60%"]
    #[inline(always)]
    pub fn divratio20(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio20)
    }
    #[doc = "Dividing master_rf clk by 21, D=57.1%"]
    #[inline(always)]
    pub fn divratio21(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio21)
    }
    #[doc = "Dividing master_rf clk by 22, D=54.5%"]
    #[inline(always)]
    pub fn divratio22(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio22)
    }
    #[doc = "Dividing master_rf clk by 23, D=52.2%"]
    #[inline(always)]
    pub fn divratio23(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcdivratio::Divratio23)
    }
}
impl R {
    #[doc = "Bit 0 - DCDC DIV Enable"]
    #[inline(always)]
    pub fn dcdcdiven(&self) -> DcdcdivenR {
        DcdcdivenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDC DIV Inverter Enable"]
    #[inline(always)]
    pub fn dcdcdivinven(&self) -> DcdcdivinvenR {
        DcdcdivinvenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - DCDC DIV Ratio"]
    #[inline(always)]
    pub fn dcdcdivratio(&self) -> DcdcdivratioR {
        DcdcdivratioR::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC DIV Enable"]
    #[inline(always)]
    pub fn dcdcdiven(&mut self) -> DcdcdivenW<'_, Rfimdcdcctrl1Spec> {
        DcdcdivenW::new(self, 0)
    }
    #[doc = "Bit 1 - DCDC DIV Inverter Enable"]
    #[inline(always)]
    pub fn dcdcdivinven(&mut self) -> DcdcdivinvenW<'_, Rfimdcdcctrl1Spec> {
        DcdcdivinvenW::new(self, 1)
    }
    #[doc = "Bits 2:5 - DCDC DIV Ratio"]
    #[inline(always)]
    pub fn dcdcdivratio(&mut self) -> DcdcdivratioW<'_, Rfimdcdcctrl1Spec> {
        DcdcdivratioW::new(self, 2)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimdcdcctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfimdcdcctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfimdcdcctrl1Spec;
impl crate::RegisterSpec for Rfimdcdcctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfimdcdcctrl1::R`](R) reader structure"]
impl crate::Readable for Rfimdcdcctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rfimdcdcctrl1::W`](W) writer structure"]
impl crate::Writable for Rfimdcdcctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFIMDCDCCTRL1 to value 0x14"]
impl crate::Resettable for Rfimdcdcctrl1Spec {
    const RESET_VALUE: u32 = 0x14;
}
