#[doc = "Register `CLKPRESCVAL` reader"]
pub type R = crate::R<ClkprescvalSpec>;
#[doc = "Register `CLKPRESCVAL` writer"]
pub type W = crate::W<ClkprescvalSpec>;
#[doc = "Lower part of divider binary counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lowerpresc {
    #[doc = "0: Divider is bypassed"]
    Bypass = 0,
    #[doc = "1: Divide by 2"]
    DivideBy2 = 1,
    #[doc = "2: Divide by 3"]
    DivideBy3 = 2,
    #[doc = "3: Divide by 4"]
    DivideBy4 = 3,
    #[doc = "4: Divide by 5"]
    DivideBy5 = 4,
    #[doc = "5: Divide by 6"]
    DivideBy6 = 5,
    #[doc = "6: Divide by 7"]
    DivideBy7 = 6,
    #[doc = "7: Divide by 8"]
    DivideBy8 = 7,
    #[doc = "8: Divide by 9"]
    DivideBy9 = 8,
    #[doc = "9: Divide by 10"]
    DivideBy10 = 9,
    #[doc = "10: Divide by 11"]
    DivideBy11 = 10,
    #[doc = "11: Divide by 12"]
    DivideBy12 = 11,
    #[doc = "12: Divide by 13"]
    DivideBy13 = 12,
    #[doc = "13: Divide by 14"]
    DivideBy14 = 13,
    #[doc = "14: Divide by 15"]
    DivideBy15 = 14,
    #[doc = "15: Divide by 16"]
    DivideBy16 = 15,
    #[doc = "16: Divide by 17"]
    DivideBy17 = 16,
    #[doc = "17: Divide by 18"]
    DivideBy18 = 17,
    #[doc = "18: Divide by 19"]
    DivideBy19 = 18,
    #[doc = "19: Divide by 20"]
    DivideBy20 = 19,
    #[doc = "20: Divide by 21"]
    DivideBy21 = 20,
    #[doc = "21: Divide by 22"]
    DivideBy22 = 21,
    #[doc = "22: Divide by 23"]
    DivideBy23 = 22,
    #[doc = "23: Divide by 24"]
    DivideBy24 = 23,
    #[doc = "24: Divide by 25"]
    DivideBy25 = 24,
    #[doc = "25: Divide by 26"]
    DivideBy26 = 25,
    #[doc = "26: Divide by 27"]
    DivideBy27 = 26,
    #[doc = "27: Divide by 28"]
    DivideBy28 = 27,
    #[doc = "28: Divide by 29"]
    DivideBy29 = 28,
    #[doc = "29: Divide by 30"]
    DivideBy30 = 29,
    #[doc = "30: Divide by 31"]
    DivideBy31 = 30,
    #[doc = "31: Divide by 32"]
    DivideBy32 = 31,
    #[doc = "32: Divide by 33"]
    DivideBy33 = 32,
    #[doc = "33: Divide by 34"]
    DivideBy34 = 33,
    #[doc = "34: Divide by 35"]
    DivideBy35 = 34,
    #[doc = "35: Divide by 36"]
    DivideBy36 = 35,
    #[doc = "36: Divide by 37"]
    DivideBy37 = 36,
    #[doc = "37: Divide by 38"]
    DivideBy38 = 37,
    #[doc = "38: Divide by 39"]
    DivideBy39 = 38,
    #[doc = "39: Divide by 40"]
    DivideBy40 = 39,
    #[doc = "40: Divide by 41"]
    DivideBy41 = 40,
    #[doc = "41: Divide by 42"]
    DivideBy42 = 41,
    #[doc = "42: Divide by 43"]
    DivideBy43 = 42,
    #[doc = "43: Divide by 44"]
    DivideBy44 = 43,
    #[doc = "44: Divide by 45"]
    DivideBy45 = 44,
    #[doc = "45: Divide by 46"]
    DivideBy46 = 45,
    #[doc = "46: Divide by 47"]
    DivideBy47 = 46,
    #[doc = "47: Divide by 48"]
    DivideBy48 = 47,
    #[doc = "48: Divide by 49"]
    DivideBy49 = 48,
    #[doc = "49: Divide by 50"]
    DivideBy50 = 49,
    #[doc = "50: Divide by 51"]
    DivideBy51 = 50,
    #[doc = "51: Divide by 52"]
    DivideBy52 = 51,
    #[doc = "52: Divide by 53"]
    DivideBy53 = 52,
    #[doc = "53: Divide by 54"]
    DivideBy54 = 53,
    #[doc = "54: Divide by 55"]
    DivideBy55 = 54,
    #[doc = "55: Divide by 56"]
    DivideBy56 = 55,
    #[doc = "56: Divide by 57"]
    DivideBy57 = 56,
    #[doc = "57: Divide by 58"]
    DivideBy58 = 57,
    #[doc = "58: Divide by 59"]
    DivideBy59 = 58,
    #[doc = "59: Divide by 60"]
    DivideBy60 = 59,
    #[doc = "60: Divide by 61"]
    DivideBy61 = 60,
    #[doc = "61: Divide by 62"]
    DivideBy62 = 61,
    #[doc = "62: Divide by 63"]
    DivideBy63 = 62,
    #[doc = "63: Divide by 64"]
    DivideBy64 = 63,
}
impl From<Lowerpresc> for u8 {
    #[inline(always)]
    fn from(variant: Lowerpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lowerpresc {
    type Ux = u8;
}
impl crate::IsEnum for Lowerpresc {}
#[doc = "Field `LOWERPRESC` reader - Lower part of divider binary counter"]
pub type LowerprescR = crate::FieldReader<Lowerpresc>;
impl LowerprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lowerpresc {
        match self.bits {
            0 => Lowerpresc::Bypass,
            1 => Lowerpresc::DivideBy2,
            2 => Lowerpresc::DivideBy3,
            3 => Lowerpresc::DivideBy4,
            4 => Lowerpresc::DivideBy5,
            5 => Lowerpresc::DivideBy6,
            6 => Lowerpresc::DivideBy7,
            7 => Lowerpresc::DivideBy8,
            8 => Lowerpresc::DivideBy9,
            9 => Lowerpresc::DivideBy10,
            10 => Lowerpresc::DivideBy11,
            11 => Lowerpresc::DivideBy12,
            12 => Lowerpresc::DivideBy13,
            13 => Lowerpresc::DivideBy14,
            14 => Lowerpresc::DivideBy15,
            15 => Lowerpresc::DivideBy16,
            16 => Lowerpresc::DivideBy17,
            17 => Lowerpresc::DivideBy18,
            18 => Lowerpresc::DivideBy19,
            19 => Lowerpresc::DivideBy20,
            20 => Lowerpresc::DivideBy21,
            21 => Lowerpresc::DivideBy22,
            22 => Lowerpresc::DivideBy23,
            23 => Lowerpresc::DivideBy24,
            24 => Lowerpresc::DivideBy25,
            25 => Lowerpresc::DivideBy26,
            26 => Lowerpresc::DivideBy27,
            27 => Lowerpresc::DivideBy28,
            28 => Lowerpresc::DivideBy29,
            29 => Lowerpresc::DivideBy30,
            30 => Lowerpresc::DivideBy31,
            31 => Lowerpresc::DivideBy32,
            32 => Lowerpresc::DivideBy33,
            33 => Lowerpresc::DivideBy34,
            34 => Lowerpresc::DivideBy35,
            35 => Lowerpresc::DivideBy36,
            36 => Lowerpresc::DivideBy37,
            37 => Lowerpresc::DivideBy38,
            38 => Lowerpresc::DivideBy39,
            39 => Lowerpresc::DivideBy40,
            40 => Lowerpresc::DivideBy41,
            41 => Lowerpresc::DivideBy42,
            42 => Lowerpresc::DivideBy43,
            43 => Lowerpresc::DivideBy44,
            44 => Lowerpresc::DivideBy45,
            45 => Lowerpresc::DivideBy46,
            46 => Lowerpresc::DivideBy47,
            47 => Lowerpresc::DivideBy48,
            48 => Lowerpresc::DivideBy49,
            49 => Lowerpresc::DivideBy50,
            50 => Lowerpresc::DivideBy51,
            51 => Lowerpresc::DivideBy52,
            52 => Lowerpresc::DivideBy53,
            53 => Lowerpresc::DivideBy54,
            54 => Lowerpresc::DivideBy55,
            55 => Lowerpresc::DivideBy56,
            56 => Lowerpresc::DivideBy57,
            57 => Lowerpresc::DivideBy58,
            58 => Lowerpresc::DivideBy59,
            59 => Lowerpresc::DivideBy60,
            60 => Lowerpresc::DivideBy61,
            61 => Lowerpresc::DivideBy62,
            62 => Lowerpresc::DivideBy63,
            63 => Lowerpresc::DivideBy64,
            _ => unreachable!(),
        }
    }
    #[doc = "Divider is bypassed"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Lowerpresc::Bypass
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_divide_by2(&self) -> bool {
        *self == Lowerpresc::DivideBy2
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn is_divide_by3(&self) -> bool {
        *self == Lowerpresc::DivideBy3
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == Lowerpresc::DivideBy4
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn is_divide_by5(&self) -> bool {
        *self == Lowerpresc::DivideBy5
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_divide_by6(&self) -> bool {
        *self == Lowerpresc::DivideBy6
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn is_divide_by7(&self) -> bool {
        *self == Lowerpresc::DivideBy7
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == Lowerpresc::DivideBy8
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn is_divide_by9(&self) -> bool {
        *self == Lowerpresc::DivideBy9
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn is_divide_by10(&self) -> bool {
        *self == Lowerpresc::DivideBy10
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn is_divide_by11(&self) -> bool {
        *self == Lowerpresc::DivideBy11
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn is_divide_by12(&self) -> bool {
        *self == Lowerpresc::DivideBy12
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn is_divide_by13(&self) -> bool {
        *self == Lowerpresc::DivideBy13
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn is_divide_by14(&self) -> bool {
        *self == Lowerpresc::DivideBy14
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn is_divide_by15(&self) -> bool {
        *self == Lowerpresc::DivideBy15
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == Lowerpresc::DivideBy16
    }
    #[doc = "Divide by 17"]
    #[inline(always)]
    pub fn is_divide_by17(&self) -> bool {
        *self == Lowerpresc::DivideBy17
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn is_divide_by18(&self) -> bool {
        *self == Lowerpresc::DivideBy18
    }
    #[doc = "Divide by 19"]
    #[inline(always)]
    pub fn is_divide_by19(&self) -> bool {
        *self == Lowerpresc::DivideBy19
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn is_divide_by20(&self) -> bool {
        *self == Lowerpresc::DivideBy20
    }
    #[doc = "Divide by 21"]
    #[inline(always)]
    pub fn is_divide_by21(&self) -> bool {
        *self == Lowerpresc::DivideBy21
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn is_divide_by22(&self) -> bool {
        *self == Lowerpresc::DivideBy22
    }
    #[doc = "Divide by 23"]
    #[inline(always)]
    pub fn is_divide_by23(&self) -> bool {
        *self == Lowerpresc::DivideBy23
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn is_divide_by24(&self) -> bool {
        *self == Lowerpresc::DivideBy24
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn is_divide_by25(&self) -> bool {
        *self == Lowerpresc::DivideBy25
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn is_divide_by26(&self) -> bool {
        *self == Lowerpresc::DivideBy26
    }
    #[doc = "Divide by 27"]
    #[inline(always)]
    pub fn is_divide_by27(&self) -> bool {
        *self == Lowerpresc::DivideBy27
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn is_divide_by28(&self) -> bool {
        *self == Lowerpresc::DivideBy28
    }
    #[doc = "Divide by 29"]
    #[inline(always)]
    pub fn is_divide_by29(&self) -> bool {
        *self == Lowerpresc::DivideBy29
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn is_divide_by30(&self) -> bool {
        *self == Lowerpresc::DivideBy30
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn is_divide_by31(&self) -> bool {
        *self == Lowerpresc::DivideBy31
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == Lowerpresc::DivideBy32
    }
    #[doc = "Divide by 33"]
    #[inline(always)]
    pub fn is_divide_by33(&self) -> bool {
        *self == Lowerpresc::DivideBy33
    }
    #[doc = "Divide by 34"]
    #[inline(always)]
    pub fn is_divide_by34(&self) -> bool {
        *self == Lowerpresc::DivideBy34
    }
    #[doc = "Divide by 35"]
    #[inline(always)]
    pub fn is_divide_by35(&self) -> bool {
        *self == Lowerpresc::DivideBy35
    }
    #[doc = "Divide by 36"]
    #[inline(always)]
    pub fn is_divide_by36(&self) -> bool {
        *self == Lowerpresc::DivideBy36
    }
    #[doc = "Divide by 37"]
    #[inline(always)]
    pub fn is_divide_by37(&self) -> bool {
        *self == Lowerpresc::DivideBy37
    }
    #[doc = "Divide by 38"]
    #[inline(always)]
    pub fn is_divide_by38(&self) -> bool {
        *self == Lowerpresc::DivideBy38
    }
    #[doc = "Divide by 39"]
    #[inline(always)]
    pub fn is_divide_by39(&self) -> bool {
        *self == Lowerpresc::DivideBy39
    }
    #[doc = "Divide by 40"]
    #[inline(always)]
    pub fn is_divide_by40(&self) -> bool {
        *self == Lowerpresc::DivideBy40
    }
    #[doc = "Divide by 41"]
    #[inline(always)]
    pub fn is_divide_by41(&self) -> bool {
        *self == Lowerpresc::DivideBy41
    }
    #[doc = "Divide by 42"]
    #[inline(always)]
    pub fn is_divide_by42(&self) -> bool {
        *self == Lowerpresc::DivideBy42
    }
    #[doc = "Divide by 43"]
    #[inline(always)]
    pub fn is_divide_by43(&self) -> bool {
        *self == Lowerpresc::DivideBy43
    }
    #[doc = "Divide by 44"]
    #[inline(always)]
    pub fn is_divide_by44(&self) -> bool {
        *self == Lowerpresc::DivideBy44
    }
    #[doc = "Divide by 45"]
    #[inline(always)]
    pub fn is_divide_by45(&self) -> bool {
        *self == Lowerpresc::DivideBy45
    }
    #[doc = "Divide by 46"]
    #[inline(always)]
    pub fn is_divide_by46(&self) -> bool {
        *self == Lowerpresc::DivideBy46
    }
    #[doc = "Divide by 47"]
    #[inline(always)]
    pub fn is_divide_by47(&self) -> bool {
        *self == Lowerpresc::DivideBy47
    }
    #[doc = "Divide by 48"]
    #[inline(always)]
    pub fn is_divide_by48(&self) -> bool {
        *self == Lowerpresc::DivideBy48
    }
    #[doc = "Divide by 49"]
    #[inline(always)]
    pub fn is_divide_by49(&self) -> bool {
        *self == Lowerpresc::DivideBy49
    }
    #[doc = "Divide by 50"]
    #[inline(always)]
    pub fn is_divide_by50(&self) -> bool {
        *self == Lowerpresc::DivideBy50
    }
    #[doc = "Divide by 51"]
    #[inline(always)]
    pub fn is_divide_by51(&self) -> bool {
        *self == Lowerpresc::DivideBy51
    }
    #[doc = "Divide by 52"]
    #[inline(always)]
    pub fn is_divide_by52(&self) -> bool {
        *self == Lowerpresc::DivideBy52
    }
    #[doc = "Divide by 53"]
    #[inline(always)]
    pub fn is_divide_by53(&self) -> bool {
        *self == Lowerpresc::DivideBy53
    }
    #[doc = "Divide by 54"]
    #[inline(always)]
    pub fn is_divide_by54(&self) -> bool {
        *self == Lowerpresc::DivideBy54
    }
    #[doc = "Divide by 55"]
    #[inline(always)]
    pub fn is_divide_by55(&self) -> bool {
        *self == Lowerpresc::DivideBy55
    }
    #[doc = "Divide by 56"]
    #[inline(always)]
    pub fn is_divide_by56(&self) -> bool {
        *self == Lowerpresc::DivideBy56
    }
    #[doc = "Divide by 57"]
    #[inline(always)]
    pub fn is_divide_by57(&self) -> bool {
        *self == Lowerpresc::DivideBy57
    }
    #[doc = "Divide by 58"]
    #[inline(always)]
    pub fn is_divide_by58(&self) -> bool {
        *self == Lowerpresc::DivideBy58
    }
    #[doc = "Divide by 59"]
    #[inline(always)]
    pub fn is_divide_by59(&self) -> bool {
        *self == Lowerpresc::DivideBy59
    }
    #[doc = "Divide by 60"]
    #[inline(always)]
    pub fn is_divide_by60(&self) -> bool {
        *self == Lowerpresc::DivideBy60
    }
    #[doc = "Divide by 61"]
    #[inline(always)]
    pub fn is_divide_by61(&self) -> bool {
        *self == Lowerpresc::DivideBy61
    }
    #[doc = "Divide by 62"]
    #[inline(always)]
    pub fn is_divide_by62(&self) -> bool {
        *self == Lowerpresc::DivideBy62
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn is_divide_by63(&self) -> bool {
        *self == Lowerpresc::DivideBy63
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == Lowerpresc::DivideBy64
    }
}
#[doc = "Field `LOWERPRESC` writer - Lower part of divider binary counter"]
pub type LowerprescW<'a, REG> = crate::FieldWriter<'a, REG, 6, Lowerpresc, crate::Safe>;
impl<'a, REG> LowerprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divider is bypassed"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::Bypass)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_by2(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_by3(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_by5(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_by6(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_by7(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn divide_by9(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn divide_by10(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy10)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn divide_by11(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy11)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn divide_by12(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy12)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn divide_by13(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy13)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn divide_by14(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy14)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn divide_by15(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy16)
    }
    #[doc = "Divide by 17"]
    #[inline(always)]
    pub fn divide_by17(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy17)
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn divide_by18(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy18)
    }
    #[doc = "Divide by 19"]
    #[inline(always)]
    pub fn divide_by19(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy19)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn divide_by20(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy20)
    }
    #[doc = "Divide by 21"]
    #[inline(always)]
    pub fn divide_by21(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy21)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn divide_by22(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy22)
    }
    #[doc = "Divide by 23"]
    #[inline(always)]
    pub fn divide_by23(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy23)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn divide_by24(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy24)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn divide_by25(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy25)
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn divide_by26(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy26)
    }
    #[doc = "Divide by 27"]
    #[inline(always)]
    pub fn divide_by27(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy27)
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn divide_by28(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy28)
    }
    #[doc = "Divide by 29"]
    #[inline(always)]
    pub fn divide_by29(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy29)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn divide_by30(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy30)
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn divide_by31(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy31)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy32)
    }
    #[doc = "Divide by 33"]
    #[inline(always)]
    pub fn divide_by33(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy33)
    }
    #[doc = "Divide by 34"]
    #[inline(always)]
    pub fn divide_by34(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy34)
    }
    #[doc = "Divide by 35"]
    #[inline(always)]
    pub fn divide_by35(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy35)
    }
    #[doc = "Divide by 36"]
    #[inline(always)]
    pub fn divide_by36(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy36)
    }
    #[doc = "Divide by 37"]
    #[inline(always)]
    pub fn divide_by37(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy37)
    }
    #[doc = "Divide by 38"]
    #[inline(always)]
    pub fn divide_by38(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy38)
    }
    #[doc = "Divide by 39"]
    #[inline(always)]
    pub fn divide_by39(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy39)
    }
    #[doc = "Divide by 40"]
    #[inline(always)]
    pub fn divide_by40(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy40)
    }
    #[doc = "Divide by 41"]
    #[inline(always)]
    pub fn divide_by41(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy41)
    }
    #[doc = "Divide by 42"]
    #[inline(always)]
    pub fn divide_by42(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy42)
    }
    #[doc = "Divide by 43"]
    #[inline(always)]
    pub fn divide_by43(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy43)
    }
    #[doc = "Divide by 44"]
    #[inline(always)]
    pub fn divide_by44(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy44)
    }
    #[doc = "Divide by 45"]
    #[inline(always)]
    pub fn divide_by45(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy45)
    }
    #[doc = "Divide by 46"]
    #[inline(always)]
    pub fn divide_by46(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy46)
    }
    #[doc = "Divide by 47"]
    #[inline(always)]
    pub fn divide_by47(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy47)
    }
    #[doc = "Divide by 48"]
    #[inline(always)]
    pub fn divide_by48(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy48)
    }
    #[doc = "Divide by 49"]
    #[inline(always)]
    pub fn divide_by49(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy49)
    }
    #[doc = "Divide by 50"]
    #[inline(always)]
    pub fn divide_by50(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy50)
    }
    #[doc = "Divide by 51"]
    #[inline(always)]
    pub fn divide_by51(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy51)
    }
    #[doc = "Divide by 52"]
    #[inline(always)]
    pub fn divide_by52(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy52)
    }
    #[doc = "Divide by 53"]
    #[inline(always)]
    pub fn divide_by53(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy53)
    }
    #[doc = "Divide by 54"]
    #[inline(always)]
    pub fn divide_by54(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy54)
    }
    #[doc = "Divide by 55"]
    #[inline(always)]
    pub fn divide_by55(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy55)
    }
    #[doc = "Divide by 56"]
    #[inline(always)]
    pub fn divide_by56(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy56)
    }
    #[doc = "Divide by 57"]
    #[inline(always)]
    pub fn divide_by57(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy57)
    }
    #[doc = "Divide by 58"]
    #[inline(always)]
    pub fn divide_by58(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy58)
    }
    #[doc = "Divide by 59"]
    #[inline(always)]
    pub fn divide_by59(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy59)
    }
    #[doc = "Divide by 60"]
    #[inline(always)]
    pub fn divide_by60(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy60)
    }
    #[doc = "Divide by 61"]
    #[inline(always)]
    pub fn divide_by61(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy61)
    }
    #[doc = "Divide by 62"]
    #[inline(always)]
    pub fn divide_by62(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy62)
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn divide_by63(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy63)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut crate::W<REG> {
        self.variant(Lowerpresc::DivideBy64)
    }
}
#[doc = "Upper part of divider ripple counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Upperpresc {
    #[doc = "0: Ripple counter divider is bypassed"]
    Bypass = 0,
    #[doc = "1: Divide by 2"]
    DivideBy2 = 1,
    #[doc = "2: Divide by 4"]
    DivideBy4 = 2,
    #[doc = "3: Divide by 8"]
    DivideBy8 = 3,
    #[doc = "4: Divide by 16"]
    DivideBy16 = 4,
    #[doc = "5: Divide by 32"]
    DivideBy32 = 5,
    #[doc = "6: Divide by 64"]
    DivideBy64 = 6,
}
impl From<Upperpresc> for u8 {
    #[inline(always)]
    fn from(variant: Upperpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Upperpresc {
    type Ux = u8;
}
impl crate::IsEnum for Upperpresc {}
#[doc = "Field `UPPERPRESC` reader - Upper part of divider ripple counter"]
pub type UpperprescR = crate::FieldReader<Upperpresc>;
impl UpperprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upperpresc {
        match self.bits {
            0 => Upperpresc::Bypass,
            1 => Upperpresc::DivideBy2,
            2 => Upperpresc::DivideBy4,
            3 => Upperpresc::DivideBy8,
            4 => Upperpresc::DivideBy16,
            5 => Upperpresc::DivideBy32,
            6 => Upperpresc::DivideBy64,
            _ => unreachable!(),
        }
    }
    #[doc = "Ripple counter divider is bypassed"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Upperpresc::Bypass
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_divide_by2(&self) -> bool {
        *self == Upperpresc::DivideBy2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == Upperpresc::DivideBy4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == Upperpresc::DivideBy8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == Upperpresc::DivideBy16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == Upperpresc::DivideBy32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == Upperpresc::DivideBy64
    }
}
#[doc = "Field `UPPERPRESC` writer - Upper part of divider ripple counter"]
pub type UpperprescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Upperpresc>;
impl<'a, REG> UpperprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ripple counter divider is bypassed"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Upperpresc::Bypass)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_by2(self) -> &'a mut crate::W<REG> {
        self.variant(Upperpresc::DivideBy2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut crate::W<REG> {
        self.variant(Upperpresc::DivideBy4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut crate::W<REG> {
        self.variant(Upperpresc::DivideBy8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut crate::W<REG> {
        self.variant(Upperpresc::DivideBy16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut crate::W<REG> {
        self.variant(Upperpresc::DivideBy32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut crate::W<REG> {
        self.variant(Upperpresc::DivideBy64)
    }
}
impl R {
    #[doc = "Bits 0:5 - Lower part of divider binary counter"]
    #[inline(always)]
    pub fn lowerpresc(&self) -> LowerprescR {
        LowerprescR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - Upper part of divider ripple counter"]
    #[inline(always)]
    pub fn upperpresc(&self) -> UpperprescR {
        UpperprescR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Lower part of divider binary counter"]
    #[inline(always)]
    pub fn lowerpresc(&mut self) -> LowerprescW<'_, ClkprescvalSpec> {
        LowerprescW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Upper part of divider ripple counter"]
    #[inline(always)]
    pub fn upperpresc(&mut self) -> UpperprescW<'_, ClkprescvalSpec> {
        UpperprescW::new(self, 8)
    }
}
#[doc = "Finial dividing factor = RIPPLECNTUPPER * CNTLOWER\n\nYou can [`read`](crate::Reg::read) this register and get [`clkprescval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkprescval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkprescvalSpec;
impl crate::RegisterSpec for ClkprescvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkprescval::R`](R) reader structure"]
impl crate::Readable for ClkprescvalSpec {}
#[doc = "`write(|w| ..)` method takes [`clkprescval::W`](W) writer structure"]
impl crate::Writable for ClkprescvalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKPRESCVAL to value 0"]
impl crate::Resettable for ClkprescvalSpec {}
