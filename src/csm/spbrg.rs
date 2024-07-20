#[doc = "Register `SPBRG` reader"]
pub type R = crate::R<SpbrgSpec>;
#[doc = "Register `SPBRG` writer"]
pub type W = crate::W<SpbrgSpec>;
#[doc = "Field `SPBRG` reader - baud rate control register for simple data"]
pub type SpbrgR = crate::FieldReader<u16>;
#[doc = "Field `SPBRG` writer - baud rate control register for simple data"]
pub type SpbrgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - baud rate control register for simple data"]
    #[inline(always)]
    pub fn spbrg(&self) -> SpbrgR {
        SpbrgR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - baud rate control register for simple data"]
    #[inline(always)]
    #[must_use]
    pub fn spbrg(&mut self) -> SpbrgW<SpbrgSpec> {
        SpbrgW::new(self, 0)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`spbrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpbrgSpec;
impl crate::RegisterSpec for SpbrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spbrg::R`](R) reader structure"]
impl crate::Readable for SpbrgSpec {}
#[doc = "`write(|w| ..)` method takes [`spbrg::W`](W) writer structure"]
impl crate::Writable for SpbrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPBRG to value 0x02"]
impl crate::Resettable for SpbrgSpec {
    const RESET_VALUE: u32 = 0x02;
}
