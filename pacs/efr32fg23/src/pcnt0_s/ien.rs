#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `UF` reader - Underflow Interrupt Read Flag"]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - Underflow Interrupt Read Flag"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` reader - Overflow Interrupt Read Flag"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - Overflow Interrupt Read Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` reader - Direction Change Detect Interrupt Flag"]
pub type DircngR = crate::BitReader;
#[doc = "Field `DIRCNG` writer - Direction Change Detect Interrupt Flag"]
pub type DircngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` reader - Auxiliary Overflow Interrupt Read Flag"]
pub type AuxofR = crate::BitReader;
#[doc = "Field `AUXOF` writer - Auxiliary Overflow Interrupt Read Flag"]
pub type AuxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OQSTERR` reader - Oversampling Quad State Err Int Flag"]
pub type OqsterrR = crate::BitReader;
#[doc = "Field `OQSTERR` writer - Oversampling Quad State Err Int Flag"]
pub type OqsterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Underflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dircng(&self) -> DircngR {
        DircngR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn auxof(&self) -> AuxofR {
        AuxofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Oversampling Quad State Err Int Flag"]
    #[inline(always)]
    pub fn oqsterr(&self) -> OqsterrR {
        OqsterrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IenSpec> {
        UfW::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IenSpec> {
        OfW::new(self, 1)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dircng(&mut self) -> DircngW<'_, IenSpec> {
        DircngW::new(self, 2)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn auxof(&mut self) -> AuxofW<'_, IenSpec> {
        AuxofW::new(self, 3)
    }
    #[doc = "Bit 4 - Oversampling Quad State Err Int Flag"]
    #[inline(always)]
    pub fn oqsterr(&mut self) -> OqsterrW<'_, IenSpec> {
        OqsterrW::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
