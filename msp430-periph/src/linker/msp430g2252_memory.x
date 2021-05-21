MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0100 /* END=0x02FF, size 256 */
  ROM (rx)         : ORIGIN = 0xF800, LENGTH = 0x07E0 /* END=0xFFDF, size 2016 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
