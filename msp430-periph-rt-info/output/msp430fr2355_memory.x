MEMORY {
  ROMLIB           : ORIGIN = 0xFAC00, LENGTH = 0x5000
  RAM              : ORIGIN = 0x2000, LENGTH = 0x1000
  ROM (rx)         : ORIGIN = 0x8000, LENGTH = 0x7F80 /* END=0xFF7F, size 32640 */
  VECTORS          : ORIGIN = 0xffa4, LENGTH = 0x005c
}
