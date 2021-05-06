use core::panic::PanicInfo;

global_asm!(
    r#"
  .section .start, "ax"
  .global _start
  .type _start,%function
_start:
  mov #_stack_start,r1
"#
);

extern "msp430-interrupt" {
    fn _start() -> !;
}

#[used]
#[link_section = ".__RESET_VECTOR"]
static __RESET_VECTOR: unsafe extern "msp430-interrupt" fn() -> ! = _start;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn abort() {
    loop {}
}

#[no_mangle]
extern "msp430-interrupt" fn DefaultHandler() -> () {
    loop {}
}

#[used]
#[link_section = ".vector_table.interrupts"]
static __INTERRUPTS: [unsafe extern "msp430-interrupt" fn(); 25] = [DefaultHandler; 25];
