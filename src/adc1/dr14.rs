#[doc = "Register `DR14` reader"]
pub type R = crate::R<Dr14Spec>;
#[doc = "Field `DATA` reader - Transfer data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `OVERRUN` reader - Overrun flag"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `VALID` reader - Valid flag"]
pub type ValidR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Transfer data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 20 - Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Channel 14 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr14Spec;
impl crate::RegisterSpec for Dr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr14::R`](R) reader structure"]
impl crate::Readable for Dr14Spec {}
#[doc = "`reset()` method sets DR14 to value 0"]
impl crate::Resettable for Dr14Spec {
    const RESET_VALUE: u32 = 0;
}
