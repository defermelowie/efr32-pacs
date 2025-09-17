#[doc = "Register `FEMROUTEPEN` reader"]
pub type R = crate::R<FemroutepenSpec>;
#[doc = "Register `FEMROUTEPEN` writer"]
pub type W = crate::W<FemroutepenSpec>;
#[doc = "Field `FEMDATA0PEN` reader - FEM Data0 Pin Enable"]
pub type Femdata0penR = crate::BitReader;
#[doc = "Field `FEMDATA0PEN` writer - FEM Data0 Pin Enable"]
pub type Femdata0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEMDATA1PEN` reader - FEM Data1 Pin Enable"]
pub type Femdata1penR = crate::BitReader;
#[doc = "Field `FEMDATA1PEN` writer - FEM Data1 Pin Enable"]
pub type Femdata1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEMDATA2PEN` reader - FEM Data2 Pin Enable"]
pub type Femdata2penR = crate::BitReader;
#[doc = "Field `FEMDATA2PEN` writer - FEM Data2 Pin Enable"]
pub type Femdata2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEMDATA3PEN` reader - FEM Data3 Pin Enable"]
pub type Femdata3penR = crate::BitReader;
#[doc = "Field `FEMDATA3PEN` writer - FEM Data3 Pin Enable"]
pub type Femdata3penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FEM Data0 Pin Enable"]
    #[inline(always)]
    pub fn femdata0pen(&self) -> Femdata0penR {
        Femdata0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FEM Data1 Pin Enable"]
    #[inline(always)]
    pub fn femdata1pen(&self) -> Femdata1penR {
        Femdata1penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FEM Data2 Pin Enable"]
    #[inline(always)]
    pub fn femdata2pen(&self) -> Femdata2penR {
        Femdata2penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FEM Data3 Pin Enable"]
    #[inline(always)]
    pub fn femdata3pen(&self) -> Femdata3penR {
        Femdata3penR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FEM Data0 Pin Enable"]
    #[inline(always)]
    pub fn femdata0pen(&mut self) -> Femdata0penW<'_, FemroutepenSpec> {
        Femdata0penW::new(self, 0)
    }
    #[doc = "Bit 1 - FEM Data1 Pin Enable"]
    #[inline(always)]
    pub fn femdata1pen(&mut self) -> Femdata1penW<'_, FemroutepenSpec> {
        Femdata1penW::new(self, 1)
    }
    #[doc = "Bit 2 - FEM Data2 Pin Enable"]
    #[inline(always)]
    pub fn femdata2pen(&mut self) -> Femdata2penW<'_, FemroutepenSpec> {
        Femdata2penW::new(self, 2)
    }
    #[doc = "Bit 3 - FEM Data3 Pin Enable"]
    #[inline(always)]
    pub fn femdata3pen(&mut self) -> Femdata3penW<'_, FemroutepenSpec> {
        Femdata3penW::new(self, 3)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`femroutepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`femroutepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FemroutepenSpec;
impl crate::RegisterSpec for FemroutepenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`femroutepen::R`](R) reader structure"]
impl crate::Readable for FemroutepenSpec {}
#[doc = "`write(|w| ..)` method takes [`femroutepen::W`](W) writer structure"]
impl crate::Writable for FemroutepenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FEMROUTEPEN to value 0"]
impl crate::Resettable for FemroutepenSpec {}
