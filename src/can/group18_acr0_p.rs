#[doc = "Register `GROUP18_ACR0_P` reader"]
pub type R = crate::R<Group18Acr0PSpec>;
#[doc = "Register `GROUP18_ACR0_P` writer"]
pub type W = crate::W<Group18Acr0PSpec>;
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
    pub fn ac(&mut self) -> AcW<Group18Acr0PSpec> {
        AcW::new(self, 0)
    }
}
#[doc = "Peli Acceptance Code register0\n\nYou can [`read`](crate::Reg::read) this register and get [`group18_acr0_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group18_acr0_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group18Acr0PSpec;
impl crate::RegisterSpec for Group18Acr0PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group18_acr0_p::R`](R) reader structure"]
impl crate::Readable for Group18Acr0PSpec {}
#[doc = "`write(|w| ..)` method takes [`group18_acr0_p::W`](W) writer structure"]
impl crate::Writable for Group18Acr0PSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GROUP18_ACR0_P to value 0"]
impl crate::Resettable for Group18Acr0PSpec {
    const RESET_VALUE: u32 = 0;
}
