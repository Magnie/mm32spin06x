#[doc = "Register `DMA_CCR1` reader"]
pub type R = crate::R<DmaCcr1Spec>;
#[doc = "Register `DMA_CCR1` writer"]
pub type W = crate::W<DmaCcr1Spec>;
#[doc = "Field `EN` reader - Channel enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Channel enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIE` reader - Half transfer interrupt enable"]
pub type HtieR = crate::BitReader;
#[doc = "Field `HTIE` writer - Half transfer interrupt enable"]
pub type HtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Data transfer direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Data transfer direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIRC` reader - Circular mode"]
pub type CircR = crate::BitReader;
#[doc = "Field `CIRC` writer - Circular mode"]
pub type CircW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINC` reader - Peripheral increment mode"]
pub type PincR = crate::BitReader;
#[doc = "Field `PINC` writer - Peripheral increment mode"]
pub type PincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINC` reader - Memory increment mode"]
pub type MincR = crate::BitReader;
#[doc = "Field `MINC` writer - Memory increment mode"]
pub type MincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE` reader - Peripheral size"]
pub type PsizeR = crate::FieldReader;
#[doc = "Field `PSIZE` writer - Peripheral size"]
pub type PsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MSIZE` reader - Memory size"]
pub type MsizeR = crate::FieldReader;
#[doc = "Field `MSIZE` writer - Memory size"]
pub type MsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PL` reader - Channel priority level"]
pub type PlR = crate::FieldReader;
#[doc = "Field `PL` writer - Channel priority level"]
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM2MEM` reader - Memory to memory mode"]
pub type Mem2memR = crate::BitReader;
#[doc = "Field `MEM2MEM` writer - Memory to memory mode"]
pub type Mem2memW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARE` reader - Auto Reload"]
pub type AreR = crate::BitReader;
#[doc = "Field `ARE` writer - Auto Reload"]
pub type AreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HtieR {
        HtieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CircR {
        CircR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PincR {
        PincR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MincR {
        MincR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline(always)]
    pub fn psize(&self) -> PsizeR {
        PsizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline(always)]
    pub fn msize(&self) -> MsizeR {
        MsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn mem2mem(&self) -> Mem2memR {
        Mem2memR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto Reload"]
    #[inline(always)]
    pub fn are(&self) -> AreR {
        AreR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<DmaCcr1Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<DmaCcr1Spec> {
        TcieW::new(self, 1)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HtieW<DmaCcr1Spec> {
        HtieW::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TeieW<DmaCcr1Spec> {
        TeieW::new(self, 3)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<DmaCcr1Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CircW<DmaCcr1Spec> {
        CircW::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PincW<DmaCcr1Spec> {
        PincW::new(self, 6)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MincW<DmaCcr1Spec> {
        MincW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PsizeW<DmaCcr1Spec> {
        PsizeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MsizeW<DmaCcr1Spec> {
        MsizeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Channel priority level"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PlW<DmaCcr1Spec> {
        PlW::new(self, 12)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> Mem2memW<DmaCcr1Spec> {
        Mem2memW::new(self, 14)
    }
    #[doc = "Bit 15 - Auto Reload"]
    #[inline(always)]
    #[must_use]
    pub fn are(&mut self) -> AreW<DmaCcr1Spec> {
        AreW::new(self, 15)
    }
}
#[doc = "DMA channel 1 configuration reigster\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCcr1Spec;
impl crate::RegisterSpec for DmaCcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ccr1::R`](R) reader structure"]
impl crate::Readable for DmaCcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_ccr1::W`](W) writer structure"]
impl crate::Writable for DmaCcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CCR1 to value 0"]
impl crate::Resettable for DmaCcr1Spec {
    const RESET_VALUE: u32 = 0;
}
