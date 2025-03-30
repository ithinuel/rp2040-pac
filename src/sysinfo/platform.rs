#[doc = "Register `PLATFORM` reader"]
pub type R = crate::R<PLATFORM_SPEC>;
#[doc = "Field `FPGA` reader - "]
pub type FPGA_R = crate::BitReader;
#[doc = "Field `ASIC` reader - "]
pub type ASIC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fpga(&self) -> FPGA_R {
        FPGA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn asic(&self) -> ASIC_R {
        ASIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLATFORM")
            .field("asic", &self.asic())
            .field("fpga", &self.fpga())
            .finish()
    }
}
#[doc = "Platform register. Allows software to know what environment it is running in.  

You can [`read`](crate::Reg::read) this register and get [`platform::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLATFORM_SPEC;
impl crate::RegisterSpec for PLATFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`platform::R`](R) reader structure"]
impl crate::Readable for PLATFORM_SPEC {}
#[doc = "`reset()` method sets PLATFORM to value 0"]
impl crate::Resettable for PLATFORM_SPEC {}
