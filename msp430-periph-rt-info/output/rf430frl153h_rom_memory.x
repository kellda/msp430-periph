MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x0200 /* END=0x1DFF, size 512 */
  ROM (rx)         : ORIGIN = 0xF840, LENGTH = 0x0790 /* END=0xFFCF, size 1936 */
  VECTORS          : ORIGIN = 0xffde, LENGTH = 0x0022
}
