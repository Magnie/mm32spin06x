#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<Apb2rstrSpec>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<Apb2rstrSpec>;
#[doc = "Field `SYSCFG` reader - Syscfg configuration register reset"]
pub type SyscfgR = crate::BitReader;
#[doc = "Field `SYSCFG` writer - Syscfg configuration register reset"]
pub type SyscfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - ADC1 interface reset"]
pub type Adc1R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 interface reset"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1` reader - TIM1 timer reset"]
pub type Tim1R = crate::BitReader;
#[doc = "Field `TIM1` writer - TIM1 timer reset"]
pub type Tim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 reset"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 reset"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - UART1 reset"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - UART1 reset"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - Comparator reset"]
pub type CompR = crate::BitReader;
#[doc = "Field `COMP` writer - Comparator reset"]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14` reader - TIM14 timer reset"]
pub type Tim14R = crate::BitReader;
#[doc = "Field `TIM14` writer - TIM14 timer reset"]
pub type Tim14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16` reader - TIM16 timer reset"]
pub type Tim16R = crate::BitReader;
#[doc = "Field `TIM16` writer - TIM16 timer reset"]
pub type Tim16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17` reader - TIM17 timer reset"]
pub type Tim17R = crate::BitReader;
#[doc = "Field `TIM17` writer - TIM17 timer reset"]
pub type Tim17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCU` reader - DBGMCU reset"]
pub type DbgmcuR = crate::BitReader;
#[doc = "Field `DBGMCU` writer - DBGMCU reset"]
pub type DbgmcuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Syscfg configuration register reset"]
    #[inline(always)]
    pub fn syscfg(&self) -> SyscfgR {
        SyscfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 interface reset"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1(&self) -> Tim1R {
        Tim1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - UART1 reset"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator reset"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM14 timer reset"]
    #[inline(always)]
    pub fn tim14(&self) -> Tim14R {
        Tim14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16(&self) -> Tim16R {
        Tim16R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17(&self) -> Tim17R {
        Tim17R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - DBGMCU reset"]
    #[inline(always)]
    pub fn dbgmcu(&self) -> DbgmcuR {
        DbgmcuR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Syscfg configuration register reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfg(&mut self) -> SyscfgW<Apb2rstrSpec> {
        SyscfgW::new(self, 0)
    }
    #[doc = "Bit 9 - ADC1 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> Adc1W<Apb2rstrSpec> {
        Adc1W::new(self, 9)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1(&mut self) -> Tim1W<Apb2rstrSpec> {
        Tim1W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> Spi1W<Apb2rstrSpec> {
        Spi1W::new(self, 12)
    }
    #[doc = "Bit 14 - UART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> Uart1W<Apb2rstrSpec> {
        Uart1W::new(self, 14)
    }
    #[doc = "Bit 15 - Comparator reset"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> CompW<Apb2rstrSpec> {
        CompW::new(self, 15)
    }
    #[doc = "Bit 16 - TIM14 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim14(&mut self) -> Tim14W<Apb2rstrSpec> {
        Tim14W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim16(&mut self) -> Tim16W<Apb2rstrSpec> {
        Tim16W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim17(&mut self) -> Tim17W<Apb2rstrSpec> {
        Tim17W::new(self, 18)
    }
    #[doc = "Bit 22 - DBGMCU reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgmcu(&mut self) -> DbgmcuW<Apb2rstrSpec> {
        DbgmcuW::new(self, 22)
    }
}
#[doc = "Advanced Peripheral Bus 2 Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstrSpec;
impl crate::RegisterSpec for Apb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for Apb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for Apb2rstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for Apb2rstrSpec {
    const RESET_VALUE: u32 = 0;
}
