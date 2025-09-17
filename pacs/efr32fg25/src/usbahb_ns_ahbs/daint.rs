#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DaintSpec>;
#[doc = "Field `INEPINT0` reader - EP0 IRQ"]
pub type Inepint0R = crate::BitReader;
#[doc = "Field `INEPINT1` reader - EP1 IRQ"]
pub type Inepint1R = crate::BitReader;
#[doc = "Field `INEPINT2` reader - EP2 IRQ"]
pub type Inepint2R = crate::BitReader;
#[doc = "Field `INEPINT3` reader - EP3 IRQ"]
pub type Inepint3R = crate::BitReader;
#[doc = "Field `INEPINT4` reader - EP4 IRQ"]
pub type Inepint4R = crate::BitReader;
#[doc = "Field `INEPINT5` reader - EP5 IRQ"]
pub type Inepint5R = crate::BitReader;
#[doc = "Field `INEPINT6` reader - EP6 IRQ"]
pub type Inepint6R = crate::BitReader;
#[doc = "Field `INEPINT7` reader - EP7 IRQ"]
pub type Inepint7R = crate::BitReader;
#[doc = "Field `INEPINT8` reader - EP8 IRQ"]
pub type Inepint8R = crate::BitReader;
#[doc = "Field `INEPINT9` reader - EP9 IRQ"]
pub type Inepint9R = crate::BitReader;
#[doc = "Field `OUTEPINT0` reader - EP0 OUT IRQ"]
pub type Outepint0R = crate::BitReader;
#[doc = "Field `OUTEPINT1` reader - EP1 OUT IRQ"]
pub type Outepint1R = crate::BitReader;
#[doc = "Field `OUTEPINT2` reader - EP2 OUT IRQ"]
pub type Outepint2R = crate::BitReader;
#[doc = "Field `OUTEPINT3` reader - EP3 OUT IRQ"]
pub type Outepint3R = crate::BitReader;
#[doc = "Field `OUTEPINT4` reader - EP4 OUT IRQ"]
pub type Outepint4R = crate::BitReader;
#[doc = "Field `OUTEPINT5` reader - EP5 OUT IRQ"]
pub type Outepint5R = crate::BitReader;
#[doc = "Field `OUTEPINT6` reader - EP6 OUT IRQ"]
pub type Outepint6R = crate::BitReader;
#[doc = "Field `OUTEPINT7` reader - EP7 OUT IRQ"]
pub type Outepint7R = crate::BitReader;
#[doc = "Field `OUTEPINT8` reader - EP8 OUT IRQ"]
pub type Outepint8R = crate::BitReader;
#[doc = "Field `OUTEPINT9` reader - EP9 OUT IRQ"]
pub type Outepint9R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EP0 IRQ"]
    #[inline(always)]
    pub fn inepint0(&self) -> Inepint0R {
        Inepint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EP1 IRQ"]
    #[inline(always)]
    pub fn inepint1(&self) -> Inepint1R {
        Inepint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP2 IRQ"]
    #[inline(always)]
    pub fn inepint2(&self) -> Inepint2R {
        Inepint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP3 IRQ"]
    #[inline(always)]
    pub fn inepint3(&self) -> Inepint3R {
        Inepint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP4 IRQ"]
    #[inline(always)]
    pub fn inepint4(&self) -> Inepint4R {
        Inepint4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EP5 IRQ"]
    #[inline(always)]
    pub fn inepint5(&self) -> Inepint5R {
        Inepint5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EP6 IRQ"]
    #[inline(always)]
    pub fn inepint6(&self) -> Inepint6R {
        Inepint6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EP7 IRQ"]
    #[inline(always)]
    pub fn inepint7(&self) -> Inepint7R {
        Inepint7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EP8 IRQ"]
    #[inline(always)]
    pub fn inepint8(&self) -> Inepint8R {
        Inepint8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EP9 IRQ"]
    #[inline(always)]
    pub fn inepint9(&self) -> Inepint9R {
        Inepint9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - EP0 OUT IRQ"]
    #[inline(always)]
    pub fn outepint0(&self) -> Outepint0R {
        Outepint0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EP1 OUT IRQ"]
    #[inline(always)]
    pub fn outepint1(&self) -> Outepint1R {
        Outepint1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EP2 OUT IRQ"]
    #[inline(always)]
    pub fn outepint2(&self) -> Outepint2R {
        Outepint2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EP3 OUT IRQ"]
    #[inline(always)]
    pub fn outepint3(&self) -> Outepint3R {
        Outepint3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - EP4 OUT IRQ"]
    #[inline(always)]
    pub fn outepint4(&self) -> Outepint4R {
        Outepint4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EP5 OUT IRQ"]
    #[inline(always)]
    pub fn outepint5(&self) -> Outepint5R {
        Outepint5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EP6 OUT IRQ"]
    #[inline(always)]
    pub fn outepint6(&self) -> Outepint6R {
        Outepint6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EP7 OUT IRQ"]
    #[inline(always)]
    pub fn outepint7(&self) -> Outepint7R {
        Outepint7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EP8 OUT IRQ"]
    #[inline(always)]
    pub fn outepint8(&self) -> Outepint8R {
        Outepint8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EP9 OUT IRQ"]
    #[inline(always)]
    pub fn outepint9(&self) -> Outepint9R {
        Outepint9R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaintSpec;
impl crate::RegisterSpec for DaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DaintSpec {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DaintSpec {}
