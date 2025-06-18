#[doc = "Register `ESPISTS` reader"]
pub type R = crate::R<EspistsSpec>;
#[doc = "Register `ESPISTS` writer"]
pub type W = crate::W<EspistsSpec>;
#[doc = "Field `IBRST` reader - In-Band Reset Command Received"]
pub type IbrstR = crate::BitReader;
#[doc = "Field `IBRST` writer - In-Band Reset Command Received"]
pub type IbrstW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CFGUPD` reader - eSPI Configuration Updated"]
pub type CfgupdR = crate::BitReader;
#[doc = "Field `CFGUPD` writer - eSPI Configuration Updated"]
pub type CfgupdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BERR` reader - eSPI Bus Error"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - eSPI Bus Error"]
pub type BerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OOBRX` reader - OOB Data Received"]
pub type OobrxR = crate::BitReader;
#[doc = "Field `OOBRX` writer - OOB Data Received"]
pub type OobrxW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLASHRX` reader - Flash Data Received"]
pub type FlashrxR = crate::BitReader;
#[doc = "Field `FLASHRX` writer - Flash Data Received"]
pub type FlashrxW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLNACS` reader - Flash Non-Automatic Completion Sent"]
pub type FlnacsR = crate::BitReader;
#[doc = "Field `FLNACS` writer - Flash Non-Automatic Completion Sent"]
pub type FlnacsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PERACC` reader - Peripheral Channel Access Detected"]
pub type PeraccR = crate::BitReader;
#[doc = "Field `PERACC` writer - Peripheral Channel Access Detected"]
pub type PeraccW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFRD` reader - Peripheral Channel Transaction Deferred"]
pub type DfrdR = crate::BitReader;
#[doc = "Field `DFRD` writer - Peripheral Channel Transaction Deferred"]
pub type DfrdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VWUPD` reader - Virtual Wire Updated"]
pub type VwupdR = crate::BitReader;
#[doc = "Field `VWUPD` writer - Virtual Wire Updated"]
pub type VwupdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ESPIRST` reader - eSPI_RST Activated"]
pub type EspirstR = crate::BitReader;
#[doc = "Field `ESPIRST` writer - eSPI_RST Activated"]
pub type EspirstW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PLTRST` reader - PLTRST Activated"]
pub type PltrstR = crate::BitReader;
#[doc = "Field `PLTRST` writer - PLTRST Activated"]
pub type PltrstW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BMWBURSTQEMP` reader - Bus Master Burst Mode Write Queue Empty"]
pub type BmwburstqempR = crate::BitReader;
#[doc = "Field `FLAUTORDREQ` reader - Flash Automatic Read Request"]
pub type FlautordreqR = crate::BitReader;
#[doc = "Field `AMERR` reader - Automatic Mode Transfer Error"]
pub type AmerrR = crate::BitReader;
#[doc = "Field `AMERR` writer - Automatic Mode Transfer Error"]
pub type AmerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AMDONE` reader - Automatic Mode Transfer Done"]
pub type AmdoneR = crate::BitReader;
#[doc = "Field `AMDONE` writer - Automatic Mode Transfer Done"]
pub type AmdoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VWUPDW` reader - Virtual Wire Updated Wake-Up"]
pub type VwupdwR = crate::BitReader;
#[doc = "Field `VWUPDW` writer - Virtual Wire Updated Wake-Up"]
pub type VwupdwW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLNPRQS` reader - Flash Non-Posted Request Sent"]
pub type FlnprqsR = crate::BitReader;
#[doc = "Field `FLNPRQS` writer - Flash Non-Posted Request Sent"]
pub type FlnprqsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BMTXDONE` reader - Peripheral Bus Master Data Transmitted"]
pub type BmtxdoneR = crate::BitReader;
#[doc = "Field `BMTXDONE` writer - Peripheral Bus Master Data Transmitted"]
pub type BmtxdoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PBMRX` reader - Peripheral Bus Master Data Received"]
pub type PbmrxR = crate::BitReader;
#[doc = "Field `PBMRX` writer - Peripheral Bus Master Data Received"]
pub type PbmrxW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PMSGRX` reader - Peripheral Message Data Received"]
pub type PmsgrxR = crate::BitReader;
#[doc = "Field `PMSGRX` writer - Peripheral Message Data Received"]
pub type PmsgrxW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BMBURSTERR` reader - Bus Master Burst Mode Read Transfer Error"]
pub type BmbursterrR = crate::BitReader;
#[doc = "Field `BMBURSTERR` writer - Bus Master Burst Mode Read Transfer Error"]
pub type BmbursterrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BMBURSTDONE` reader - Bus Master Burst Mode Read Transfer Done"]
pub type BmburstdoneR = crate::BitReader;
#[doc = "Field `BMBURSTDONE` writer - Bus Master Burst Mode Read Transfer Done"]
pub type BmburstdoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ESPIRST_LVL` reader - eSPI_RST Level"]
pub type EspirstLvlR = crate::BitReader;
#[doc = "Field `FLPRTERR` reader - Flash Protection Error"]
pub type FlprterrR = crate::BitReader;
#[doc = "Field `FLPRTERR` writer - Flash Protection Error"]
pub type FlprterrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLAUTORDSTR` reader - Flash Automatic Read Request Start"]
pub type FlautordstrR = crate::BitReader;
#[doc = "Field `FLAUTORDSTR` writer - Flash Automatic Read Request Start"]
pub type FlautordstrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLAUTORDPND` reader - Flash Automatic Read Request Pending"]
pub type FlautordpndR = crate::BitReader;
#[doc = "Field `FLAUTORDPND` writer - Flash Automatic Read Request Pending"]
pub type FlautordpndW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLAUTORDQEMP` reader - Flash Automatic Read Queue Empty"]
pub type FlautordqempR = crate::BitReader;
#[doc = "Field `FLAUTORDQEMP` writer - Flash Automatic Read Queue Empty"]
pub type FlautordqempW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUTO_RD_DIS_STS` reader - Automatic Read Disable Status"]
pub type AutoRdDisStsR = crate::BitReader;
#[doc = "Field `AUTO_RD_DIS_STS` writer - Automatic Read Disable Status"]
pub type AutoRdDisStsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BMWBURSTERR` reader - Bus Master Burst Mode Write Transfer Error"]
pub type BmwbursterrR = crate::BitReader;
#[doc = "Field `BMWBURSTERR` writer - Bus Master Burst Mode Write Transfer Error"]
pub type BmwbursterrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BMWBURSTDONE` reader - Bus Master Burst Mode Write Transfer Done"]
pub type BmwburstdoneR = crate::BitReader;
#[doc = "Field `BMWBURSTDONE` writer - Bus Master Burst Mode Write Transfer Done"]
pub type BmwburstdoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - In-Band Reset Command Received"]
    #[inline(always)]
    pub fn ibrst(&self) -> IbrstR {
        IbrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - eSPI Configuration Updated"]
    #[inline(always)]
    pub fn cfgupd(&self) -> CfgupdR {
        CfgupdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - eSPI Bus Error"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OOB Data Received"]
    #[inline(always)]
    pub fn oobrx(&self) -> OobrxR {
        OobrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Data Received"]
    #[inline(always)]
    pub fn flashrx(&self) -> FlashrxR {
        FlashrxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Non-Automatic Completion Sent"]
    #[inline(always)]
    pub fn flnacs(&self) -> FlnacsR {
        FlnacsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral Channel Access Detected"]
    #[inline(always)]
    pub fn peracc(&self) -> PeraccR {
        PeraccR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Channel Transaction Deferred"]
    #[inline(always)]
    pub fn dfrd(&self) -> DfrdR {
        DfrdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Virtual Wire Updated"]
    #[inline(always)]
    pub fn vwupd(&self) -> VwupdR {
        VwupdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - eSPI_RST Activated"]
    #[inline(always)]
    pub fn espirst(&self) -> EspirstR {
        EspirstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLTRST Activated"]
    #[inline(always)]
    pub fn pltrst(&self) -> PltrstR {
        PltrstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Master Burst Mode Write Queue Empty"]
    #[inline(always)]
    pub fn bmwburstqemp(&self) -> BmwburstqempR {
        BmwburstqempR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Flash Automatic Read Request"]
    #[inline(always)]
    pub fn flautordreq(&self) -> FlautordreqR {
        FlautordreqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Automatic Mode Transfer Error"]
    #[inline(always)]
    pub fn amerr(&self) -> AmerrR {
        AmerrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Automatic Mode Transfer Done"]
    #[inline(always)]
    pub fn amdone(&self) -> AmdoneR {
        AmdoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Virtual Wire Updated Wake-Up"]
    #[inline(always)]
    pub fn vwupdw(&self) -> VwupdwR {
        VwupdwR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flash Non-Posted Request Sent"]
    #[inline(always)]
    pub fn flnprqs(&self) -> FlnprqsR {
        FlnprqsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral Bus Master Data Transmitted"]
    #[inline(always)]
    pub fn bmtxdone(&self) -> BmtxdoneR {
        BmtxdoneR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral Bus Master Data Received"]
    #[inline(always)]
    pub fn pbmrx(&self) -> PbmrxR {
        PbmrxR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral Message Data Received"]
    #[inline(always)]
    pub fn pmsgrx(&self) -> PmsgrxR {
        PmsgrxR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bus Master Burst Mode Read Transfer Error"]
    #[inline(always)]
    pub fn bmbursterr(&self) -> BmbursterrR {
        BmbursterrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bus Master Burst Mode Read Transfer Done"]
    #[inline(always)]
    pub fn bmburstdone(&self) -> BmburstdoneR {
        BmburstdoneR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - eSPI_RST Level"]
    #[inline(always)]
    pub fn espirst_lvl(&self) -> EspirstLvlR {
        EspirstLvlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Flash Protection Error"]
    #[inline(always)]
    pub fn flprterr(&self) -> FlprterrR {
        FlprterrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Flash Automatic Read Request Start"]
    #[inline(always)]
    pub fn flautordstr(&self) -> FlautordstrR {
        FlautordstrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Flash Automatic Read Request Pending"]
    #[inline(always)]
    pub fn flautordpnd(&self) -> FlautordpndR {
        FlautordpndR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Flash Automatic Read Queue Empty"]
    #[inline(always)]
    pub fn flautordqemp(&self) -> FlautordqempR {
        FlautordqempR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Automatic Read Disable Status"]
    #[inline(always)]
    pub fn auto_rd_dis_sts(&self) -> AutoRdDisStsR {
        AutoRdDisStsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Bus Master Burst Mode Write Transfer Error"]
    #[inline(always)]
    pub fn bmwbursterr(&self) -> BmwbursterrR {
        BmwbursterrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bus Master Burst Mode Write Transfer Done"]
    #[inline(always)]
    pub fn bmwburstdone(&self) -> BmwburstdoneR {
        BmwburstdoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPISTS")
            .field("ibrst", &self.ibrst())
            .field("cfgupd", &self.cfgupd())
            .field("berr", &self.berr())
            .field("oobrx", &self.oobrx())
            .field("flashrx", &self.flashrx())
            .field("flnacs", &self.flnacs())
            .field("peracc", &self.peracc())
            .field("dfrd", &self.dfrd())
            .field("vwupd", &self.vwupd())
            .field("espirst", &self.espirst())
            .field("pltrst", &self.pltrst())
            .field("bmwburstqemp", &self.bmwburstqemp())
            .field("flautordreq", &self.flautordreq())
            .field("amerr", &self.amerr())
            .field("amdone", &self.amdone())
            .field("vwupdw", &self.vwupdw())
            .field("flnprqs", &self.flnprqs())
            .field("bmtxdone", &self.bmtxdone())
            .field("pbmrx", &self.pbmrx())
            .field("pmsgrx", &self.pmsgrx())
            .field("bmbursterr", &self.bmbursterr())
            .field("bmburstdone", &self.bmburstdone())
            .field("espirst_lvl", &self.espirst_lvl())
            .field("flprterr", &self.flprterr())
            .field("flautordstr", &self.flautordstr())
            .field("flautordpnd", &self.flautordpnd())
            .field("flautordqemp", &self.flautordqemp())
            .field("auto_rd_dis_sts", &self.auto_rd_dis_sts())
            .field("bmwbursterr", &self.bmwbursterr())
            .field("bmwburstdone", &self.bmwburstdone())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - In-Band Reset Command Received"]
    #[inline(always)]
    pub fn ibrst(&mut self) -> IbrstW<EspistsSpec> {
        IbrstW::new(self, 0)
    }
    #[doc = "Bit 1 - eSPI Configuration Updated"]
    #[inline(always)]
    pub fn cfgupd(&mut self) -> CfgupdW<EspistsSpec> {
        CfgupdW::new(self, 1)
    }
    #[doc = "Bit 2 - eSPI Bus Error"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<EspistsSpec> {
        BerrW::new(self, 2)
    }
    #[doc = "Bit 3 - OOB Data Received"]
    #[inline(always)]
    pub fn oobrx(&mut self) -> OobrxW<EspistsSpec> {
        OobrxW::new(self, 3)
    }
    #[doc = "Bit 4 - Flash Data Received"]
    #[inline(always)]
    pub fn flashrx(&mut self) -> FlashrxW<EspistsSpec> {
        FlashrxW::new(self, 4)
    }
    #[doc = "Bit 5 - Flash Non-Automatic Completion Sent"]
    #[inline(always)]
    pub fn flnacs(&mut self) -> FlnacsW<EspistsSpec> {
        FlnacsW::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral Channel Access Detected"]
    #[inline(always)]
    pub fn peracc(&mut self) -> PeraccW<EspistsSpec> {
        PeraccW::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Channel Transaction Deferred"]
    #[inline(always)]
    pub fn dfrd(&mut self) -> DfrdW<EspistsSpec> {
        DfrdW::new(self, 7)
    }
    #[doc = "Bit 8 - Virtual Wire Updated"]
    #[inline(always)]
    pub fn vwupd(&mut self) -> VwupdW<EspistsSpec> {
        VwupdW::new(self, 8)
    }
    #[doc = "Bit 9 - eSPI_RST Activated"]
    #[inline(always)]
    pub fn espirst(&mut self) -> EspirstW<EspistsSpec> {
        EspirstW::new(self, 9)
    }
    #[doc = "Bit 10 - PLTRST Activated"]
    #[inline(always)]
    pub fn pltrst(&mut self) -> PltrstW<EspistsSpec> {
        PltrstW::new(self, 10)
    }
    #[doc = "Bit 15 - Automatic Mode Transfer Error"]
    #[inline(always)]
    pub fn amerr(&mut self) -> AmerrW<EspistsSpec> {
        AmerrW::new(self, 15)
    }
    #[doc = "Bit 16 - Automatic Mode Transfer Done"]
    #[inline(always)]
    pub fn amdone(&mut self) -> AmdoneW<EspistsSpec> {
        AmdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Virtual Wire Updated Wake-Up"]
    #[inline(always)]
    pub fn vwupdw(&mut self) -> VwupdwW<EspistsSpec> {
        VwupdwW::new(self, 17)
    }
    #[doc = "Bit 18 - Flash Non-Posted Request Sent"]
    #[inline(always)]
    pub fn flnprqs(&mut self) -> FlnprqsW<EspistsSpec> {
        FlnprqsW::new(self, 18)
    }
    #[doc = "Bit 19 - Peripheral Bus Master Data Transmitted"]
    #[inline(always)]
    pub fn bmtxdone(&mut self) -> BmtxdoneW<EspistsSpec> {
        BmtxdoneW::new(self, 19)
    }
    #[doc = "Bit 20 - Peripheral Bus Master Data Received"]
    #[inline(always)]
    pub fn pbmrx(&mut self) -> PbmrxW<EspistsSpec> {
        PbmrxW::new(self, 20)
    }
    #[doc = "Bit 21 - Peripheral Message Data Received"]
    #[inline(always)]
    pub fn pmsgrx(&mut self) -> PmsgrxW<EspistsSpec> {
        PmsgrxW::new(self, 21)
    }
    #[doc = "Bit 22 - Bus Master Burst Mode Read Transfer Error"]
    #[inline(always)]
    pub fn bmbursterr(&mut self) -> BmbursterrW<EspistsSpec> {
        BmbursterrW::new(self, 22)
    }
    #[doc = "Bit 23 - Bus Master Burst Mode Read Transfer Done"]
    #[inline(always)]
    pub fn bmburstdone(&mut self) -> BmburstdoneW<EspistsSpec> {
        BmburstdoneW::new(self, 23)
    }
    #[doc = "Bit 25 - Flash Protection Error"]
    #[inline(always)]
    pub fn flprterr(&mut self) -> FlprterrW<EspistsSpec> {
        FlprterrW::new(self, 25)
    }
    #[doc = "Bit 26 - Flash Automatic Read Request Start"]
    #[inline(always)]
    pub fn flautordstr(&mut self) -> FlautordstrW<EspistsSpec> {
        FlautordstrW::new(self, 26)
    }
    #[doc = "Bit 27 - Flash Automatic Read Request Pending"]
    #[inline(always)]
    pub fn flautordpnd(&mut self) -> FlautordpndW<EspistsSpec> {
        FlautordpndW::new(self, 27)
    }
    #[doc = "Bit 28 - Flash Automatic Read Queue Empty"]
    #[inline(always)]
    pub fn flautordqemp(&mut self) -> FlautordqempW<EspistsSpec> {
        FlautordqempW::new(self, 28)
    }
    #[doc = "Bit 29 - Automatic Read Disable Status"]
    #[inline(always)]
    pub fn auto_rd_dis_sts(&mut self) -> AutoRdDisStsW<EspistsSpec> {
        AutoRdDisStsW::new(self, 29)
    }
    #[doc = "Bit 30 - Bus Master Burst Mode Write Transfer Error"]
    #[inline(always)]
    pub fn bmwbursterr(&mut self) -> BmwbursterrW<EspistsSpec> {
        BmwbursterrW::new(self, 30)
    }
    #[doc = "Bit 31 - Bus Master Burst Mode Write Transfer Done"]
    #[inline(always)]
    pub fn bmwburstdone(&mut self) -> BmwburstdoneW<EspistsSpec> {
        BmwburstdoneW::new(self, 31)
    }
}
#[doc = "eSPI Status Register (ESPISTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`espists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspistsSpec;
impl crate::RegisterSpec for EspistsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espists::R`](R) reader structure"]
impl crate::Readable for EspistsSpec {}
#[doc = "`write(|w| ..)` method takes [`espists::W`](W) writer structure"]
impl crate::Writable for EspistsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xfeff_87ff;
}
#[doc = "`reset()` method sets ESPISTS to value 0"]
impl crate::Resettable for EspistsSpec {
    const RESET_VALUE: u32 = 0;
}
