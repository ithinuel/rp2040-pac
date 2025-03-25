#[doc = "Register `FC0_REF_KHZ` reader"]
pub type R = crate::R<FC0_REF_KHZ_SPEC>;
#[doc = "Register `FC0_REF_KHZ` writer"]
pub type W = crate::W<FC0_REF_KHZ_SPEC>;
#[doc = "Field `FC0_REF_KHZ` reader - "]
pub type FC0_REF_KHZ_R = crate::FieldReader<u32>;
#[doc = "Field `FC0_REF_KHZ` writer - "]
pub type FC0_REF_KHZ_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn fc0_ref_khz(&self) -> FC0_REF_KHZ_R {
        FC0_REF_KHZ_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC0_REF_KHZ")
            .field("fc0_ref_khz", &self.fc0_ref_khz())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn fc0_ref_khz(&mut self) -> FC0_REF_KHZ_W<FC0_REF_KHZ_SPEC> {
        FC0_REF_KHZ_W::new(self, 0)
    }
}
#[doc = "Reference clock frequency in kHz  

You can [`read`](crate::Reg::read) this register and get [`fc0_ref_khz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc0_ref_khz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_REF_KHZ_SPEC;
impl crate::RegisterSpec for FC0_REF_KHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_ref_khz::R`](R) reader structure"]
impl crate::Readable for FC0_REF_KHZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc0_ref_khz::W`](W) writer structure"]
impl crate::Writable for FC0_REF_KHZ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC0_REF_KHZ to value 0"]
impl crate::Resettable for FC0_REF_KHZ_SPEC {}
