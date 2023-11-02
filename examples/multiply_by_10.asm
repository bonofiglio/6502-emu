LDA #10
JSR MUL10
JMP END

MUL10:  
    ASL
    STA TEMP
    ASL
    ASL
    CLC
    ADC TEMP
    RTS
TEMP:
    .byte 0

END:
    STA $6000

