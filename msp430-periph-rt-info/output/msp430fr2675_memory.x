MEMORY {
  ROMLIB           : ORIGIN = 0xC0000, LENGTH = 0x4000
  RAM              : ORIGIN = 0x2000, LENGTH = 0x1800
  VECTORS          : ORIGIN = 0xffa4, LENGTH = 0x005c
}
