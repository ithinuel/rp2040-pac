#[doc = "Register `IC_CLR_TX_ABRT` reader"]
pub type R = crate::R<IC_CLR_TX_ABRT_SPEC>;
#[doc = "Field `CLR_TX_ABRT` reader - Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.  

 Reset value: 0x0"]
pub type CLR_TX_ABRT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_tx_abrt(&self) -> CLR_TX_ABRT_R {
        CLR_TX_ABRT_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_CLR_TX_ABRT")
            .field("clr_tx_abrt", &self.clr_tx_abrt())
            .finish()
    }
}
#[doc = "Clear TX_ABRT Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_abrt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_TX_ABRT_SPEC;
impl crate::RegisterSpec for IC_CLR_TX_ABRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_tx_abrt::R`](R) reader structure"]
impl crate::Readable for IC_CLR_TX_ABRT_SPEC {}
#[doc = "`reset()` method sets IC_CLR_TX_ABRT to value 0"]
impl crate::Resettable for IC_CLR_TX_ABRT_SPEC {}
