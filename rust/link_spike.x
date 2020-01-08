SECTIONS
{
  .htif :
  {
    INPUT_SECTION_FLAGS (SHF_WRITE) *(.htif .htif.*)
  } > HTIF
}
