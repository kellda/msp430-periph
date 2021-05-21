MEMORY {
  ROMLIB           : ORIGIN = 0xFAC00, LENGTH = 0x5000
  RAM              : ORIGIN = 0x2000, LENGTH = 0x1000
  VECTORS          : ORIGIN = 0xffa4, LENGTH = 0x005c
}
