#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `SW` reader - System clock switch"]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - System clock switch"]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SwsR = crate::FieldReader;
#[doc = "Field `SWS` writer - System clock switch status"]
pub type SwsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HPRE` reader - AHB Prescaler"]
pub type HpreR = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB Prescaler"]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE1` reader - APB low-speed prescaler(APB1)"]
pub type Ppre1R = crate::FieldReader;
#[doc = "Field `PPRE1` writer - APB low-speed prescaler(APB1)"]
pub type Ppre1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPRE2` reader - APB high-speed prescaler(APB2)"]
pub type Ppre2R = crate::FieldReader;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler(APB2)"]
pub type Ppre2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLK48SEL` reader - USB 48M clock selection"]
pub type Clk48selR = crate::BitReader;
#[doc = "Field `CLK48SEL` writer - USB 48M clock selection"]
pub type Clk48selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PllsrcR = crate::BitReader;
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PllsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry"]
pub type PllxtpreR = crate::BitReader;
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry"]
pub type PllxtpreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRPE` reader - USB prescaler"]
pub type UsbrpeR = crate::FieldReader;
#[doc = "Field `USBRPE` writer - USB prescaler"]
pub type UsbrpeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCO` reader - Microcontroller clock output"]
pub type McoR = crate::FieldReader;
#[doc = "Field `MCO` writer - Microcontroller clock output"]
pub type McoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLDN` reader - PLL configuration factor expansion"]
pub type PlldnR = crate::FieldReader;
#[doc = "Field `PLLDN` writer - PLL configuration factor expansion"]
pub type PlldnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB Prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB low-speed prescaler(APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> Ppre1R {
        Ppre1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler(APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> Ppre2R {
        Ppre2R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 15 - USB 48M clock selection"]
    #[inline(always)]
    pub fn clk48sel(&self) -> Clk48selR {
        Clk48selR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PllxtpreR {
        PllxtpreR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 22:23 - USB prescaler"]
    #[inline(always)]
    pub fn usbrpe(&self) -> UsbrpeR {
        UsbrpeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&self) -> McoR {
        McoR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - PLL configuration factor expansion"]
    #[inline(always)]
    pub fn plldn(&self) -> PlldnR {
        PlldnR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SwW<CfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SwsW<CfgrSpec> {
        SwsW::new(self, 2)
    }
    #[doc = "Bits 4:7 - AHB Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HpreW<CfgrSpec> {
        HpreW::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB low-speed prescaler(APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> Ppre1W<CfgrSpec> {
        Ppre1W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler(APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> Ppre2W<CfgrSpec> {
        Ppre2W::new(self, 11)
    }
    #[doc = "Bit 15 - USB 48M clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn clk48sel(&mut self) -> Clk48selW<CfgrSpec> {
        Clk48selW::new(self, 15)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PllsrcW<CfgrSpec> {
        PllsrcW::new(self, 16)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    #[must_use]
    pub fn pllxtpre(&mut self) -> PllxtpreW<CfgrSpec> {
        PllxtpreW::new(self, 17)
    }
    #[doc = "Bits 22:23 - USB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn usbrpe(&mut self) -> UsbrpeW<CfgrSpec> {
        UsbrpeW::new(self, 22)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    #[must_use]
    pub fn mco(&mut self) -> McoW<CfgrSpec> {
        McoW::new(self, 24)
    }
    #[doc = "Bits 30:31 - PLL configuration factor expansion"]
    #[inline(always)]
    #[must_use]
    pub fn plldn(&mut self) -> PlldnW<CfgrSpec> {
        PlldnW::new(self, 30)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0;
}
