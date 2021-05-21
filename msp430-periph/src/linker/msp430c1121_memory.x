MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0100 /* END=0x02FF, size 256 */
  ROM (rx)         : ORIGIN = 0xF000, LENGTH = 0x0FE0 /* END=0xFFDF, size 4064 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
