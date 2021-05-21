MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0800 /* END=0x09FF, size 2048 */
  ROM (rx)         : ORIGIN = 0x1100, LENGTH = 0xEEC0 /* END=0xFFBF, size 61120 */
  VECTORS          : ORIGIN = 0xffc0, LENGTH = 0x0040
}
