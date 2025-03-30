#[doc = "Register `INTE1` reader"]
pub type R = crate::R<INTE1_SPEC>;
#[doc = "Register `INTE1` writer"]
pub type W = crate::W<INTE1_SPEC>;
#[doc = "Field `INTE1` reader - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
pub type INTE1_R = crate::FieldReader<u16>;
#[doc = "Field `INTE1` writer - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
pub type INTE1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    #[inline(always)]
    pub fn inte1(&self) -> INTE1_R {
        INTE1_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTE1")
            .field("inte1", &self.inte1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    #[inline(always)]
    pub fn inte1(&mut self) -> INTE1_W<INTE1_SPEC> {
        INTE1_W::new(self, 0)
    }
}
#[doc = "Interrupt Enables for IRQ 1  

You can [`read`](crate::Reg::read) this register and get [`inte1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE1_SPEC;
impl crate::RegisterSpec for INTE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte1::R`](R) reader structure"]
impl crate::Readable for INTE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte1::W`](W) writer structure"]
impl crate::Writable for INTE1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTE1 to value 0"]
impl crate::Resettable for INTE1_SPEC {}
