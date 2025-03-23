#[doc = "Register `TIMERAWL` reader"]
pub type R = crate::R<TIMERAWL_SPEC>;
#[doc = "Field `TIMERAWL` reader - "]
pub type TIMERAWL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timerawl(&self) -> TIMERAWL_R {
        TIMERAWL_R::new(self.bits)
    }
}
#[doc = "Raw read from bits 31:0 of time (no side effects)  

You can [`read`](crate::Reg::read) this register and get [`timerawl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERAWL_SPEC;
impl crate::RegisterSpec for TIMERAWL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerawl::R`](R) reader structure"]
impl crate::Readable for TIMERAWL_SPEC {}
#[doc = "`reset()` method sets TIMERAWL to value 0"]
impl crate::Resettable for TIMERAWL_SPEC {}
