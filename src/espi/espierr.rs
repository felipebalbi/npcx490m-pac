#[doc = "Register `ESPIERR` reader"]
pub type R = crate::R<EspierrSpec>;
#[doc = "Register `ESPIERR` writer"]
pub type W = crate::W<EspierrSpec>;
#[doc = "Field `INVCMD` reader - Invalid Command Type"]
pub type InvcmdR = crate::BitReader;
#[doc = "Field `INVCMD` writer - Invalid Command Type"]
pub type InvcmdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INVCYC` reader - Invalid Cycle Type"]
pub type InvcycR = crate::BitReader;
#[doc = "Field `INVCYC` writer - Invalid Cycle Type"]
pub type InvcycW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CRCERR` reader - Transaction CRC Error"]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - Transaction CRC Error"]
pub type CrcerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ABCOMP` reader - Abnormal Completion"]
pub type AbcompR = crate::BitReader;
#[doc = "Field `ABCOMP` writer - Abnormal Completion"]
pub type AbcompW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PROTERR` reader - Protocol Error"]
pub type ProterrR = crate::BitReader;
#[doc = "Field `PROTERR` writer - Protocol Error"]
pub type ProterrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BADSIZE` reader - Bad Size"]
pub type BadsizeR = crate::BitReader;
#[doc = "Field `BADSIZE` writer - Bad Size"]
pub type BadsizeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NPBADALN` reader - Non-Posted Peripheral Channel Bad Address Alignment"]
pub type NpbadalnR = crate::BitReader;
#[doc = "Field `NPBADALN` writer - Non-Posted Peripheral Channel Bad Address Alignment"]
pub type NpbadalnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PCBADALN` reader - Posted Peripheral Channel Bad Address Alignment"]
pub type PcbadalnR = crate::BitReader;
#[doc = "Field `PCBADALN` writer - Posted Peripheral Channel Bad Address Alignment"]
pub type PcbadalnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLBADALN` reader - Target-Attached Flash Bad Address Alignment"]
pub type FlbadalnR = crate::BitReader;
#[doc = "Field `FLBADALN` writer - Target-Attached Flash Bad Address Alignment"]
pub type FlbadalnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNCMD` reader - Unsupported Command or Cycle Type"]
pub type UncmdR = crate::BitReader;
#[doc = "Field `UNCMD` writer - Unsupported Command or Cycle Type"]
pub type UncmdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EXTRACYC` reader - Extra eSPI Clock Cycles"]
pub type ExtracycR = crate::BitReader;
#[doc = "Field `EXTRACYC` writer - Extra eSPI Clock Cycles"]
pub type ExtracycW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VWERR` reader - Virtual Channel Access Error"]
pub type VwerrR = crate::BitReader;
#[doc = "Field `VWERR` writer - Virtual Channel Access Error"]
pub type VwerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UNPBM` reader - Unsuccessful Bus Master Completion"]
pub type UnpbmR = crate::BitReader;
#[doc = "Field `UNPBM` writer - Unsuccessful Bus Master Completion"]
pub type UnpbmW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UNFLASH` reader - Unsuccessful Flash Completion"]
pub type UnflashR = crate::BitReader;
#[doc = "Field `UNFLASH` writer - Unsuccessful Flash Completion"]
pub type UnflashW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invalid Command Type"]
    #[inline(always)]
    pub fn invcmd(&self) -> InvcmdR {
        InvcmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalid Cycle Type"]
    #[inline(always)]
    pub fn invcyc(&self) -> InvcycR {
        InvcycR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction CRC Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Abnormal Completion"]
    #[inline(always)]
    pub fn abcomp(&self) -> AbcompR {
        AbcompR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protocol Error"]
    #[inline(always)]
    pub fn proterr(&self) -> ProterrR {
        ProterrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bad Size"]
    #[inline(always)]
    pub fn badsize(&self) -> BadsizeR {
        BadsizeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non-Posted Peripheral Channel Bad Address Alignment"]
    #[inline(always)]
    pub fn npbadaln(&self) -> NpbadalnR {
        NpbadalnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Posted Peripheral Channel Bad Address Alignment"]
    #[inline(always)]
    pub fn pcbadaln(&self) -> PcbadalnR {
        PcbadalnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Target-Attached Flash Bad Address Alignment"]
    #[inline(always)]
    pub fn flbadaln(&self) -> FlbadalnR {
        FlbadalnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Unsupported Command or Cycle Type"]
    #[inline(always)]
    pub fn uncmd(&self) -> UncmdR {
        UncmdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Extra eSPI Clock Cycles"]
    #[inline(always)]
    pub fn extracyc(&self) -> ExtracycR {
        ExtracycR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Virtual Channel Access Error"]
    #[inline(always)]
    pub fn vwerr(&self) -> VwerrR {
        VwerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Unsuccessful Bus Master Completion"]
    #[inline(always)]
    pub fn unpbm(&self) -> UnpbmR {
        UnpbmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Unsuccessful Flash Completion"]
    #[inline(always)]
    pub fn unflash(&self) -> UnflashR {
        UnflashR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPIERR")
            .field("invcmd", &self.invcmd())
            .field("invcyc", &self.invcyc())
            .field("crcerr", &self.crcerr())
            .field("abcomp", &self.abcomp())
            .field("proterr", &self.proterr())
            .field("badsize", &self.badsize())
            .field("npbadaln", &self.npbadaln())
            .field("pcbadaln", &self.pcbadaln())
            .field("flbadaln", &self.flbadaln())
            .field("uncmd", &self.uncmd())
            .field("vwerr", &self.vwerr())
            .field("extracyc", &self.extracyc())
            .field("unpbm", &self.unpbm())
            .field("unflash", &self.unflash())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Invalid Command Type"]
    #[inline(always)]
    pub fn invcmd(&mut self) -> InvcmdW<EspierrSpec> {
        InvcmdW::new(self, 0)
    }
    #[doc = "Bit 1 - Invalid Cycle Type"]
    #[inline(always)]
    pub fn invcyc(&mut self) -> InvcycW<EspierrSpec> {
        InvcycW::new(self, 1)
    }
    #[doc = "Bit 2 - Transaction CRC Error"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<EspierrSpec> {
        CrcerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Abnormal Completion"]
    #[inline(always)]
    pub fn abcomp(&mut self) -> AbcompW<EspierrSpec> {
        AbcompW::new(self, 3)
    }
    #[doc = "Bit 4 - Protocol Error"]
    #[inline(always)]
    pub fn proterr(&mut self) -> ProterrW<EspierrSpec> {
        ProterrW::new(self, 4)
    }
    #[doc = "Bit 5 - Bad Size"]
    #[inline(always)]
    pub fn badsize(&mut self) -> BadsizeW<EspierrSpec> {
        BadsizeW::new(self, 5)
    }
    #[doc = "Bit 6 - Non-Posted Peripheral Channel Bad Address Alignment"]
    #[inline(always)]
    pub fn npbadaln(&mut self) -> NpbadalnW<EspierrSpec> {
        NpbadalnW::new(self, 6)
    }
    #[doc = "Bit 7 - Posted Peripheral Channel Bad Address Alignment"]
    #[inline(always)]
    pub fn pcbadaln(&mut self) -> PcbadalnW<EspierrSpec> {
        PcbadalnW::new(self, 7)
    }
    #[doc = "Bit 8 - Target-Attached Flash Bad Address Alignment"]
    #[inline(always)]
    pub fn flbadaln(&mut self) -> FlbadalnW<EspierrSpec> {
        FlbadalnW::new(self, 8)
    }
    #[doc = "Bit 9 - Unsupported Command or Cycle Type"]
    #[inline(always)]
    pub fn uncmd(&mut self) -> UncmdW<EspierrSpec> {
        UncmdW::new(self, 9)
    }
    #[doc = "Bit 10 - Extra eSPI Clock Cycles"]
    #[inline(always)]
    pub fn extracyc(&mut self) -> ExtracycW<EspierrSpec> {
        ExtracycW::new(self, 10)
    }
    #[doc = "Bit 11 - Virtual Channel Access Error"]
    #[inline(always)]
    pub fn vwerr(&mut self) -> VwerrW<EspierrSpec> {
        VwerrW::new(self, 11)
    }
    #[doc = "Bit 14 - Unsuccessful Bus Master Completion"]
    #[inline(always)]
    pub fn unpbm(&mut self) -> UnpbmW<EspierrSpec> {
        UnpbmW::new(self, 14)
    }
    #[doc = "Bit 15 - Unsuccessful Flash Completion"]
    #[inline(always)]
    pub fn unflash(&mut self) -> UnflashW<EspierrSpec> {
        UnflashW::new(self, 15)
    }
}
#[doc = "eSPI Error Status Register (ESPIERR)\n\nYou can [`read`](crate::Reg::read) this register and get [`espierr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espierr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspierrSpec;
impl crate::RegisterSpec for EspierrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espierr::R`](R) reader structure"]
impl crate::Readable for EspierrSpec {}
#[doc = "`write(|w| ..)` method takes [`espierr::W`](W) writer structure"]
impl crate::Writable for EspierrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xceff;
}
#[doc = "`reset()` method sets ESPIERR to value 0"]
impl crate::Resettable for EspierrSpec {
    const RESET_VALUE: u32 = 0;
}
