MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0100 /* END=0x02FF, size 256 */
  ROM (rx)         : ORIGIN = 0xF800, LENGTH = 0x07DE /* END=0xFFDD, size 2014 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
