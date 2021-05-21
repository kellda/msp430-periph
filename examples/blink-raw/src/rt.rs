use core::panic::PanicInfo;

// This is our initialisation code. We will make so that it is the very first thing that will run
global_asm!(
    r#"
  ; `.section` (along with the linker script) tells the linker to put this at the beginning of the ROM
  .section .start, "ax"
  ; Name this initialisation code `_start`
  .global _start
  .type _start,%function
_start:
  ; Set up the stack pointer. `_stack_start` comes from the linker script.
  mov #_stack_start, sp
  ; Fall into `main`
"#
);

extern "msp430-interrupt" {
    // The above initialisation code
    fn _start() -> !;
}


#[link_section = ".start"]
// The main function. It will be called from assembly, so it needs to be `extern`. It also never returns.

// This prevents `__RESET_VECTOR` from being optimised out
#[used]
// Set `_start` as the reset vector, i.e. the thing that will run when the CPU is reset.
#[link_section = ".__RESET_VECTOR"]
static __RESET_VECTOR: unsafe extern "msp430-interrupt" fn() -> ! = _start;

// Set up a dummy panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// What to do if the program aborts
#[no_mangle]
extern "C" fn abort() {
    loop {}
}

// The default interrupt handler
#[no_mangle]
extern "msp430-interrupt" fn DefaultHandler() -> () {
    loop {}
}

// This prevents `__INTERRUPTS` from being optimised out
#[used]
// Set `DefaultHandler` as the interrupt handler for all interrupts.
#[link_section = ".vector_table.interrupts"]
static __INTERRUPTS: [unsafe extern "msp430-interrupt" fn(); 25] = [DefaultHandler; 25];
