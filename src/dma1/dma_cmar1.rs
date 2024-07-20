#[doc = "Register `DMA_CMAR1` reader"]
pub type R = crate::R<DmaCmar1Spec>;
#[doc = "Register `DMA_CMAR1` writer"]
pub type W = crate::W<DmaCmar1Spec>;
#[doc = "Field `MA` reader - Memory address"]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address"]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MaW<DmaCmar1Spec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA channel 1 memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cmar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCmar1Spec;
impl crate::RegisterSpec for DmaCmar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cmar1::R`](R) reader structure"]
impl crate::Readable for DmaCmar1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cmar1::W`](W) writer structure"]
impl crate::Writable for DmaCmar1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CMAR1 to value 0"]
impl crate::Resettable for DmaCmar1Spec {
    const RESET_VALUE: u32 = 0;
}
