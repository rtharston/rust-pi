//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.section .text._start

_start:
    // Wait for events forever (for now, we'll do cooler things soon!)
.L_wait_forever:
    wfe
    b       .L_wait_forever

.size   _start, . - _start
.type   _start, function
.global _start