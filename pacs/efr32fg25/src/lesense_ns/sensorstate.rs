#[doc = "Register `SENSORSTATE` reader"]
pub type R = crate::R<SensorstateSpec>;
#[doc = "Field `SENSORSTATE` reader - Sensor State"]
pub type SensorstateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Sensor State"]
    #[inline(always)]
    pub fn sensorstate(&self) -> SensorstateR {
        SensorstateR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Decoder input register\n\nYou can [`read`](crate::Reg::read) this register and get [`sensorstate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SensorstateSpec;
impl crate::RegisterSpec for SensorstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sensorstate::R`](R) reader structure"]
impl crate::Readable for SensorstateSpec {}
#[doc = "`reset()` method sets SENSORSTATE to value 0"]
impl crate::Resettable for SensorstateSpec {}
