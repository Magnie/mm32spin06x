#[doc = "Register `GROUP12_ACR_B` reader"]
pub type R = crate::R<Group12AcrBSpec>;
#[doc = "Register `GROUP12_ACR_B` writer"]
pub type W = crate::W<Group12AcrBSpec>;
#[doc = "Field `AC` reader - Acceptance code"]
pub type AcR = crate::FieldReader;
#[doc = "Field `AC` writer - Acceptance code"]
pub type AcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Acceptance code"]
    #[inline(always)]
    pub fn ac(&self) -> AcR {
        AcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance code"]
    #[inline(always)]
    #[must_use]
    pub fn ac(&mut self) -> AcW<Group12AcrBSpec> {
        AcW::new(self, 0)
    }
}
#[doc = "Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_acr_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_acr_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group12AcrBSpec;
impl crate::RegisterSpec for Group12AcrBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group12_acr_b::R`](R) reader structure"]
impl crate::Readable for Group12AcrBSpec {}
#[doc = "`write(|w| ..)` method takes [`group12_acr_b::W`](W) writer structure"]
impl crate::Writable for Group12AcrBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GROUP12_ACR_B to value 0"]
impl crate::Resettable for Group12AcrBSpec {
    const RESET_VALUE: u32 = 0;
}
