#[doc = "Register `GROUP3_ACR_B` reader"]
pub type R = crate::R<Group3AcrBSpec>;
#[doc = "Register `GROUP3_ACR_B` writer"]
pub type W = crate::W<Group3AcrBSpec>;
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
    pub fn ac(&mut self) -> AcW<Group3AcrBSpec> {
        AcW::new(self, 0)
    }
}
#[doc = "Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group3_acr_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group3_acr_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group3AcrBSpec;
impl crate::RegisterSpec for Group3AcrBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group3_acr_b::R`](R) reader structure"]
impl crate::Readable for Group3AcrBSpec {}
#[doc = "`write(|w| ..)` method takes [`group3_acr_b::W`](W) writer structure"]
impl crate::Writable for Group3AcrBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GROUP3_ACR_B to value 0"]
impl crate::Resettable for Group3AcrBSpec {
    const RESET_VALUE: u32 = 0;
}
