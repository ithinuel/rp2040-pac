#[doc = "Register `TIMELR` reader"]
pub type R = crate::R<TIMELR_SPEC>;
#[doc = "Field `TIMELR` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TIMELR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timelr(&self) -> TIMELR_R {
        TIMELR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMELR").finish()
    }
}
#[doc = "Read from bits 31:0 of time  

You can [`read`](crate::Reg::read) this register and get [`timelr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMELR_SPEC;
impl crate::RegisterSpec for TIMELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timelr::R`](R) reader structure"]
impl crate::Readable for TIMELR_SPEC {}
#[doc = "`reset()` method sets TIMELR to value 0"]
impl crate::Resettable for TIMELR_SPEC {}
