#[doc = "Register `GROUP10_AMR_B` reader"]
pub type R = crate::R<Group10AmrBSpec>;
#[doc = "Register `GROUP10_AMR_B` writer"]
pub type W = crate::W<Group10AmrBSpec>;
#[doc = "Field `AM` reader - Acceptance mask"]
pub type AmR = crate::FieldReader;
#[doc = "Field `AM` writer - Acceptance mask"]
pub type AmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Acceptance mask"]
    #[inline(always)]
    pub fn am(&self) -> AmR {
        AmR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance mask"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AmW<Group10AmrBSpec> {
        AmW::new(self, 0)
    }
}
#[doc = "Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_amr_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_amr_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group10AmrBSpec;
impl crate::RegisterSpec for Group10AmrBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group10_amr_b::R`](R) reader structure"]
impl crate::Readable for Group10AmrBSpec {}
#[doc = "`write(|w| ..)` method takes [`group10_amr_b::W`](W) writer structure"]
impl crate::Writable for Group10AmrBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GROUP10_AMR_B to value 0"]
impl crate::Resettable for Group10AmrBSpec {
    const RESET_VALUE: u32 = 0;
}
