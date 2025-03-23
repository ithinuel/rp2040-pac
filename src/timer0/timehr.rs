#[doc = "Register `TIMEHR` reader"]
pub type R = crate::R<TIMEHR_SPEC>;
#[doc = "Field `TIMEHR` reader - "]
pub type TIMEHR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timehr(&self) -> TIMEHR_R {
        TIMEHR_R::new(self.bits)
    }
}
#[doc = "Read from bits 63:32 of time  
 always read timelr before timehr  

You can [`read`](crate::Reg::read) this register and get [`timehr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEHR_SPEC;
impl crate::RegisterSpec for TIMEHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timehr::R`](R) reader structure"]
impl crate::Readable for TIMEHR_SPEC {}
#[doc = "`reset()` method sets TIMEHR to value 0"]
impl crate::Resettable for TIMEHR_SPEC {}
