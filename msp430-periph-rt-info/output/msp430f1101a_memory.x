MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0080 /* END=0x027F, size 128 */
  ROM (rx)         : ORIGIN = 0xFC00, LENGTH = 0x03E0 /* END=0xFFDF, size 992 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
