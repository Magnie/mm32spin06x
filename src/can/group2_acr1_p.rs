#[doc = "Register `GROUP2_ACR1_P` reader"]
pub type R = crate::R<Group2Acr1PSpec>;
#[doc = "Register `GROUP2_ACR1_P` writer"]
pub type W = crate::W<Group2Acr1PSpec>;
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
    pub fn ac(&mut self) -> AcW<Group2Acr1PSpec> {
        AcW::new(self, 0)
    }
}
#[doc = "Peli Acceptance Code register1\n\nYou can [`read`](crate::Reg::read) this register and get [`group2_acr1_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group2_acr1_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group2Acr1PSpec;
impl crate::RegisterSpec for Group2Acr1PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group2_acr1_p::R`](R) reader structure"]
impl crate::Readable for Group2Acr1PSpec {}
#[doc = "`write(|w| ..)` method takes [`group2_acr1_p::W`](W) writer structure"]
impl crate::Writable for Group2Acr1PSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GROUP2_ACR1_P to value 0"]
impl crate::Resettable for Group2Acr1PSpec {
    const RESET_VALUE: u32 = 0;
}
