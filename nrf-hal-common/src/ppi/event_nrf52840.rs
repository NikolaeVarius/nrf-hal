use crate::ppi::Event;

// Event impls
//
// To reproduce, in the pac crate, search
//   `rg 'type EVENTS_.*crate::Reg' --type rust`
// Find (regex):
//   `^src/(.*)\.rs:pub type (.*) = .*$`
// Replace (regex):
//   `impl Event for crate::target::$1::$2 { }`
impl Event for crate::target::nfct::EVENTS_READY {}
impl Event for crate::target::nfct::EVENTS_FIELDDETECTED {}
impl Event for crate::target::nfct::EVENTS_FIELDLOST {}
impl Event for crate::target::nfct::EVENTS_TXFRAMESTART {}
impl Event for crate::target::nfct::EVENTS_TXFRAMEEND {}
impl Event for crate::target::nfct::EVENTS_RXFRAMESTART {}
impl Event for crate::target::nfct::EVENTS_RXFRAMEEND {}
impl Event for crate::target::nfct::EVENTS_ERROR {}
impl Event for crate::target::nfct::EVENTS_RXERROR {}
impl Event for crate::target::nfct::EVENTS_ENDRX {}
impl Event for crate::target::nfct::EVENTS_ENDTX {}
impl Event for crate::target::nfct::EVENTS_AUTOCOLRESSTARTED {}
impl Event for crate::target::nfct::EVENTS_COLLISION {}
impl Event for crate::target::nfct::EVENTS_SELECTED {}
impl Event for crate::target::nfct::EVENTS_STARTED {}
impl Event for crate::target::temp::EVENTS_DATARDY {}
impl Event for crate::target::rng::EVENTS_VALRDY {}
impl Event for crate::target::timer0::EVENTS_COMPARE {}
impl Event for crate::target::uart0::EVENTS_CTS {}
impl Event for crate::target::uart0::EVENTS_NCTS {}
impl Event for crate::target::uart0::EVENTS_RXDRDY {}
impl Event for crate::target::uart0::EVENTS_TXDRDY {}
impl Event for crate::target::uart0::EVENTS_ERROR {}
impl Event for crate::target::uart0::EVENTS_RXTO {}
impl Event for crate::target::clock::EVENTS_HFCLKSTARTED {}
impl Event for crate::target::clock::EVENTS_LFCLKSTARTED {}
impl Event for crate::target::clock::EVENTS_DONE {}
impl Event for crate::target::clock::EVENTS_CTTO {}
impl Event for crate::target::clock::EVENTS_CTSTARTED {}
impl Event for crate::target::clock::EVENTS_CTSTOPPED {}
impl Event for crate::target::gpiote::EVENTS_IN {}
impl Event for crate::target::gpiote::EVENTS_PORT {}
impl Event for crate::target::spis0::EVENTS_END {}
impl Event for crate::target::spis0::EVENTS_ENDRX {}
impl Event for crate::target::spis0::EVENTS_ACQUIRED {}
impl Event for crate::target::spim0::EVENTS_STOPPED {}
impl Event for crate::target::spim0::EVENTS_ENDRX {}
impl Event for crate::target::spim0::EVENTS_END {}
impl Event for crate::target::spim0::EVENTS_ENDTX {}
impl Event for crate::target::spim0::EVENTS_STARTED {}
impl Event for crate::target::power::EVENTS_POFWARN {}
impl Event for crate::target::power::EVENTS_SLEEPENTER {}
impl Event for crate::target::power::EVENTS_SLEEPEXIT {}
impl Event for crate::target::power::EVENTS_USBDETECTED {}
impl Event for crate::target::power::EVENTS_USBREMOVED {}
impl Event for crate::target::power::EVENTS_USBPWRRDY {}
impl Event for crate::target::spi0::EVENTS_READY {}
impl Event for crate::target::wdt::EVENTS_TIMEOUT {}
impl Event for crate::target::egu0::EVENTS_TRIGGERED {}
impl Event for crate::target::rtc0::EVENTS_TICK {}
impl Event for crate::target::rtc0::EVENTS_OVRFLW {}
impl Event for crate::target::rtc0::EVENTS_COMPARE {}
impl Event for crate::target::twi0::EVENTS_STOPPED {}
impl Event for crate::target::twi0::EVENTS_RXDREADY {}
impl Event for crate::target::twi0::EVENTS_TXDSENT {}
impl Event for crate::target::twi0::EVENTS_ERROR {}
impl Event for crate::target::twi0::EVENTS_BB {}
impl Event for crate::target::twi0::EVENTS_SUSPENDED {}
impl Event for crate::target::pdm::EVENTS_STARTED {}
impl Event for crate::target::pdm::EVENTS_STOPPED {}
impl Event for crate::target::pdm::EVENTS_END {}
impl Event for crate::target::lpcomp::EVENTS_READY {}
impl Event for crate::target::lpcomp::EVENTS_DOWN {}
impl Event for crate::target::lpcomp::EVENTS_UP {}
impl Event for crate::target::lpcomp::EVENTS_CROSS {}
impl Event for crate::target::radio::EVENTS_READY {}
impl Event for crate::target::radio::EVENTS_ADDRESS {}
impl Event for crate::target::radio::EVENTS_PAYLOAD {}
impl Event for crate::target::radio::EVENTS_END {}
impl Event for crate::target::radio::EVENTS_DISABLED {}
impl Event for crate::target::radio::EVENTS_DEVMATCH {}
impl Event for crate::target::radio::EVENTS_DEVMISS {}
impl Event for crate::target::radio::EVENTS_RSSIEND {}
impl Event for crate::target::radio::EVENTS_BCMATCH {}
impl Event for crate::target::radio::EVENTS_CRCOK {}
impl Event for crate::target::radio::EVENTS_CRCERROR {}
impl Event for crate::target::radio::EVENTS_FRAMESTART {}
impl Event for crate::target::radio::EVENTS_EDEND {}
impl Event for crate::target::radio::EVENTS_EDSTOPPED {}
impl Event for crate::target::radio::EVENTS_CCAIDLE {}
impl Event for crate::target::radio::EVENTS_CCABUSY {}
impl Event for crate::target::radio::EVENTS_CCASTOPPED {}
impl Event for crate::target::radio::EVENTS_RATEBOOST {}
impl Event for crate::target::radio::EVENTS_TXREADY {}
impl Event for crate::target::radio::EVENTS_RXREADY {}
impl Event for crate::target::radio::EVENTS_MHRMATCH {}
impl Event for crate::target::radio::EVENTS_PHYEND {}
impl Event for crate::target::ecb::EVENTS_ENDECB {}
impl Event for crate::target::ecb::EVENTS_ERRORECB {}
impl Event for crate::target::twim0::EVENTS_STOPPED {}
impl Event for crate::target::twim0::EVENTS_ERROR {}
impl Event for crate::target::twim0::EVENTS_SUSPENDED {}
impl Event for crate::target::twim0::EVENTS_RXSTARTED {}
impl Event for crate::target::twim0::EVENTS_TXSTARTED {}
impl Event for crate::target::twim0::EVENTS_LASTRX {}
impl Event for crate::target::twim0::EVENTS_LASTTX {}
impl Event for crate::target::ccm::EVENTS_ENDKSGEN {}
impl Event for crate::target::ccm::EVENTS_ENDCRYPT {}
impl Event for crate::target::ccm::EVENTS_ERROR {}
impl Event for crate::target::uarte0::EVENTS_CTS {}
impl Event for crate::target::uarte0::EVENTS_NCTS {}
impl Event for crate::target::uarte0::EVENTS_RXDRDY {}
impl Event for crate::target::uarte0::EVENTS_ENDRX {}
impl Event for crate::target::uarte0::EVENTS_TXDRDY {}
impl Event for crate::target::uarte0::EVENTS_ENDTX {}
impl Event for crate::target::uarte0::EVENTS_ERROR {}
impl Event for crate::target::uarte0::EVENTS_RXTO {}
impl Event for crate::target::uarte0::EVENTS_RXSTARTED {}
impl Event for crate::target::uarte0::EVENTS_TXSTARTED {}
impl Event for crate::target::uarte0::EVENTS_TXSTOPPED {}
impl Event for crate::target::i2s::EVENTS_RXPTRUPD {}
impl Event for crate::target::i2s::EVENTS_STOPPED {}
impl Event for crate::target::i2s::EVENTS_TXPTRUPD {}
impl Event for crate::target::twis0::EVENTS_STOPPED {}
impl Event for crate::target::twis0::EVENTS_ERROR {}
impl Event for crate::target::twis0::EVENTS_RXSTARTED {}
impl Event for crate::target::twis0::EVENTS_TXSTARTED {}
impl Event for crate::target::twis0::EVENTS_WRITE {}
impl Event for crate::target::twis0::EVENTS_READ {}
impl Event for crate::target::timer3::EVENTS_COMPARE {}
impl Event for crate::target::qdec::EVENTS_SAMPLERDY {}
impl Event for crate::target::qdec::EVENTS_REPORTRDY {}
impl Event for crate::target::qdec::EVENTS_ACCOF {}
impl Event for crate::target::qdec::EVENTS_DBLRDY {}
impl Event for crate::target::qdec::EVENTS_STOPPED {}
impl Event for crate::target::aar::EVENTS_END {}
impl Event for crate::target::aar::EVENTS_RESOLVED {}
impl Event for crate::target::aar::EVENTS_NOTRESOLVED {}
impl Event for crate::target::comp::EVENTS_READY {}
impl Event for crate::target::comp::EVENTS_DOWN {}
impl Event for crate::target::comp::EVENTS_UP {}
impl Event for crate::target::comp::EVENTS_CROSS {}
impl Event for crate::target::usbd::EVENTS_USBRESET {}
impl Event for crate::target::usbd::EVENTS_STARTED {}
impl Event for crate::target::usbd::EVENTS_ENDEPIN {}
impl Event for crate::target::usbd::EVENTS_EP0DATADONE {}
impl Event for crate::target::usbd::EVENTS_ENDISOIN {}
impl Event for crate::target::usbd::EVENTS_ENDEPOUT {}
impl Event for crate::target::usbd::EVENTS_ENDISOOUT {}
impl Event for crate::target::usbd::EVENTS_SOF {}
impl Event for crate::target::usbd::EVENTS_USBEVENT {}
impl Event for crate::target::usbd::EVENTS_EP0SETUP {}
impl Event for crate::target::usbd::EVENTS_EPDATA {}
impl Event for crate::target::pwm0::EVENTS_STOPPED {}
impl Event for crate::target::pwm0::EVENTS_SEQSTARTED {}
impl Event for crate::target::pwm0::EVENTS_SEQEND {}
impl Event for crate::target::pwm0::EVENTS_PWMPERIODEND {}
impl Event for crate::target::pwm0::EVENTS_LOOPSDONE {}
impl Event for crate::target::saadc::EVENTS_STARTED {}
impl Event for crate::target::saadc::EVENTS_END {}
impl Event for crate::target::saadc::EVENTS_DONE {}
impl Event for crate::target::saadc::EVENTS_RESULTDONE {}
impl Event for crate::target::saadc::EVENTS_CALIBRATEDONE {}
impl Event for crate::target::saadc::EVENTS_STOPPED {}
impl Event for crate::target::qspi::EVENTS_READY {}
