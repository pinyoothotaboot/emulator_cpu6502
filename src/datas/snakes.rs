/**
0600: 20 06 06 20 38 06 20 0d 06 20 2a 06 60 a9 02 85
0610: 02 a9 04 85 03 a9 11 85 10 a9 10 85 12 a9 0f 85
0620: 14 a9 04 85 11 85 13 85 15 60 a5 fe 85 00 a5 fe
0630: 29 03 18 69 02 85 01 60 20 4d 06 20 8d 06 20 c3
0640: 06 20 19 07 20 20 07 20 2d 07 4c 38 06 a5 ff c9
0650: 77 f0 0d c9 64 f0 14 c9 73 f0 1b c9 61 f0 22 60
0660: a9 04 24 02 d0 26 a9 01 85 02 60 a9 08 24 02 d0
0670: 1b a9 02 85 02 60 a9 01 24 02 d0 10 a9 04 85 02
0680: 60 a9 02 24 02 d0 05 a9 08 85 02 60 60 20 94 06
0690: 20 a8 06 60 a5 00 c5 10 d0 0d a5 01 c5 11 d0 07
06a0: e6 03 e6 03 20 2a 06 60 a2 02 b5 10 c5 10 d0 06
06b0: b5 11 c5 11 f0 09 e8 e8 e4 03 f0 06 4c aa 06 4c
06c0: 35 07 60 a6 03 ca 8a b5 10 95 12 ca 10 f9 a5 02
06d0: 4a b0 09 4a b0 19 4a b0 1f 4a b0 2f a5 10 38 e9
06e0: 20 85 10 90 01 60 c6 11 a9 01 c5 11 f0 28 60 e6
06f0: 10 a9 1f 24 10 f0 1f 60 a5 10 18 69 20 85 10 b0
0700: 01 60 e6 11 a9 06 c5 11 f0 0c 60 c6 10 a5 10 29
0710: 1f c9 1f f0 01 60 4c 35 07 a0 00 a5 fe 91 00 60
0720: a6 03 a9 00 81 10 a2 00 a9 01 81 10 60 a2 00 ea
0730: ea ca d0 fb 60
*/
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SNAKE_MEMORY_MAPS: [u8;65536] = {
        let mut memory : [u8;65536] = [0x00;65536];
        //
        // $0600    20 06 06  JSR $0606
        // $0603    20 38 06  JSR $0638
        // $0606    20 0d 06  JSR $060d
        // $0609    20 2a 06  JSR $062a
        // $060c    60        RTS
        // $060d    a9 02     LDA #$02
        // $060f    85 02     STA $02
        //
        memory[0x0600] = 0x20;
        memory[0x0601] = 0x06;
        memory[0x0602] = 0x06;
        memory[0x0603] = 0x20;
        memory[0x0604] = 0x38;
        memory[0x0605] = 0x06;
        memory[0x0606] = 0x20;
        memory[0x0607] = 0x0d;
        memory[0x0608] = 0x06;
        memory[0x0609] = 0x20;
        memory[0x060A] = 0x2a;
        memory[0x060B] = 0x06;
        memory[0x060C] = 0x60;
        memory[0x060D] = 0xa9;
        memory[0x060E] = 0x02;
        memory[0x060F] = 0x85;
        //
        // $0611    a9 04     LDA #$04
        // $0613    85 03     STA $03
        // $0615    a9 11     LDA #$11
        // $0617    85 10     STA $10
        // $0619    a9 10     LDA #$10
        //
        memory[0x0610] = 0x02;
        memory[0x0611] = 0xa9;
        memory[0x0612] = 0x04;
        memory[0x0613] = 0x85;
        memory[0x0614] = 0x03;
        memory[0x0615] = 0xa9;
        memory[0x0616] = 0x11;
        memory[0x0617] = 0x85;
        memory[0x0618] = 0x10;
        memory[0x0619] = 0xa9;
        memory[0x061A] = 0x10;
        memory[0x061B] = 0x85;
        memory[0x061C] = 0x12;
        memory[0x061D] = 0xa9;
        memory[0x061E] = 0x0f;
        memory[0x061F] = 0x85;
        //
        // $061b    85 12     STA $12
        // $061d    a9 0f     LDA #$0f
        // $061f    85 14     STA $14
        // $0621    a9 04     LDA #$04
        // $0623    85 11     STA $11
        // $0625    85 13     STA $13
        // $0627    85 15     STA $15
        // $0629    60        RTS
        //
        memory[0x0620] = 0x14;
        memory[0x0621] = 0xa9;
        memory[0x0622] = 0x04;
        memory[0x0623] = 0x85;
        memory[0x0624] = 0x11;
        memory[0x0625] = 0x85;
        memory[0x0626] = 0x13;
        memory[0x0627] = 0x85;
        memory[0x0628] = 0x15;
        memory[0x0629] = 0x60;
        memory[0x062A] = 0xa5;
        memory[0x062B] = 0xfe;
        memory[0x062C] = 0x85;
        memory[0x062D] = 0x00;
        memory[0x062E] = 0xa5;
        memory[0x062F] = 0xfe;
        //
        // $0630    29 03     AND #$03
        // $0632    18        CLC
        // $0633    69 02     ADC #$02
        // $0635    85 01     STA $01
        // $0637    60        RTS
        // $0638    20 4d 06  JSR $064d
        // $063b    20 8d 06  JSR $068d
        // $063e    20 c3 06  JSR $06c3
        //
        memory[0x0630] = 0x29;
        memory[0x0631] = 0x03;
        memory[0x0632] = 0x18;
        memory[0x0633] = 0x69;
        memory[0x0634] = 0x02;
        memory[0x0635] = 0x85;
        memory[0x0636] = 0x01;
        memory[0x0637] = 0x60;
        memory[0x0638] = 0x20;
        memory[0x0639] = 0x4d;
        memory[0x063A] = 0x06;
        memory[0x063B] = 0x20;
        memory[0x063C] = 0x8d;
        memory[0x063D] = 0x06;
        memory[0x063E] = 0x20;
        memory[0x063F] = 0xc3;
        //
        // $0641    20 19 07  JSR $0719
        // $0644    20 20 07  JSR $0720
        // $0647    20 2d 07  JSR $072d
        // $064a    4c 38 06  JMP $0638
        // $064d    a5 ff     LDA $ff
        // $064f    c9 77     CMP #$77
        //
        memory[0x0640] = 0x06;
        memory[0x0641] = 0x20;
        memory[0x0642] = 0x19;
        memory[0x0643] = 0x07;
        memory[0x0644] = 0x20;
        memory[0x0645] = 0x20;
        memory[0x0646] = 0x07;
        memory[0x0647] = 0x20;
        memory[0x0648] = 0x2d;
        memory[0x0649] = 0x07;
        memory[0x064A] = 0x4c;
        memory[0x064B] = 0x38;
        memory[0x064C] = 0x06;
        memory[0x064D] = 0xa5;
        memory[0x064E] = 0xff;
        memory[0x064F] = 0xc9;
        //
        // $0651    f0 0d     BEQ $0660
        // $0653    c9 64     CMP #$64
        // $0655    f0 14     BEQ $066b
        // $0657    c9 73     CMP #$73
        // $0659    f0 1b     BEQ $0676
        // $065b    c9 61     CMP #$61
        // $065d    f0 22     BEQ $0681
        // $065f    60        RTS
        // $0660    a9 04     LDA #$04
        //
        memory[0x0650] = 0x77;
        memory[0x0651] = 0xf0;
        memory[0x0652] = 0x0d;
        memory[0x0653] = 0xc9;
        memory[0x0654] = 0x64;
        memory[0x0655] = 0xf0;
        memory[0x0656] = 0x14;
        memory[0x0657] = 0xc9;
        memory[0x0658] = 0x73;
        memory[0x0659] = 0xf0;
        memory[0x065A] = 0x1b;
        memory[0x065B] = 0xc9;
        memory[0x065C] = 0x61;
        memory[0x065D] = 0xf0;
        memory[0x065E] = 0x22;
        memory[0x065F] = 0x60;
        //
        // $0660    a9 04     LDA #$04
        // $0662    24 02     BIT $02
        // $0664    d0 26     BNE $068c
        // $0666    a9 01     LDA #$01
        // $0668    85 02     STA $02
        // $066a    60        RTS
        // $066b    a9 08     LDA #$08
        // $066d    24 02     BIT $02
        // $066f    d0 1b     BNE $068c
        //
        memory[0x0660] = 0xa9;
        memory[0x0661] = 0x04;
        memory[0x0662] = 0x24;
        memory[0x0663] = 0x02;
        memory[0x0664] = 0xd0;
        memory[0x0665] = 0x26;
        memory[0x0666] = 0xa9;
        memory[0x0667] = 0x01;
        memory[0x0668] = 0x85;
        memory[0x0669] = 0x02;
        memory[0x066A] = 0x60;
        memory[0x066B] = 0xa9;
        memory[0x066C] = 0x08;
        memory[0x066D] = 0x24;
        memory[0x066E] = 0x02;
        memory[0x066F] = 0xd0;
        //
        // $0671    a9 02     LDA #$02
        // $0673    85 02     STA $02
        // $0675    60        RTS
        // $0676    a9 01     LDA #$01
        // $0678    24 02     BIT $02
        // $067a    d0 10     BNE $068c
        // $067c    a9 04     LDA #$04
        // $067e    85 02     STA $02
        //
        memory[0x0670] = 0x1b;
        memory[0x0671] = 0xa9;
        memory[0x0672] = 0x02;
        memory[0x0673] = 0x85;
        memory[0x0674] = 0x02;
        memory[0x0675] = 0x60;
        memory[0x0676] = 0xa9;
        memory[0x0677] = 0x01;
        memory[0x0678] = 0x24;
        memory[0x0679] = 0x02;
        memory[0x067A] = 0xd0;
        memory[0x067B] = 0x10;
        memory[0x067C] = 0xa9;
        memory[0x067D] = 0x04;
        memory[0x067E] = 0x85;
        memory[0x067F] = 0x02;
        //
        // $0680    60        RTS
        // $0681    a9 02     LDA #$02
        // $0683    24 02     BIT $02
        // $0685    d0 05     BNE $068c
        // $0687    a9 08     LDA #$08
        // $0689    85 02     STA $02
        // $068b    60        RTS
        // $068c    60        RTS
        // $068d    20 94 06  JSR $0694
        //
        memory[0x0680] = 0x60;
        memory[0x0681] = 0xa9;
        memory[0x0682] = 0x02;
        memory[0x0683] = 0x24;
        memory[0x0684] = 0x02;
        memory[0x0685] = 0xd0;
        memory[0x0686] = 0x05;
        memory[0x0687] = 0xa9;
        memory[0x0688] = 0x08;
        memory[0x0689] = 0x85;
        memory[0x068A] = 0x02;
        memory[0x068B] = 0x60;
        memory[0x068C] = 0x60;
        memory[0x068D] = 0x20;
        memory[0x068E] = 0x94;
        memory[0x068F] = 0x06;
        //
        // $0690    20 a8 06  JSR $06a8
        // $0693    60        RTS
        // $0694    a5 00     LDA $00
        // $0696    c5 10     CMP $10
        // $0698    d0 0d     BNE $06a7
        // $069a    a5 01     LDA $01
        // $069c    c5 11     CMP $11
        // $069e    d0 07     BNE $06a7
        //
        memory[0x0690] = 0x20;
        memory[0x0691] = 0xa8;
        memory[0x0692] = 0x06;
        memory[0x0693] = 0x60;
        memory[0x0694] = 0xa5;
        memory[0x0695] = 0x00;
        memory[0x0696] = 0xc5;
        memory[0x0697] = 0x10;
        memory[0x0698] = 0xd0;
        memory[0x0699] = 0x0d;
        memory[0x069A] = 0xa5;
        memory[0x069B] = 0x01;
        memory[0x069C] = 0xc5;
        memory[0x069D] = 0x11;
        memory[0x069E] = 0xd0;
        memory[0x069F] = 0x07;
        //
        // $06a0    e6 03     INC $03
        // $06a2    e6 03     INC $03
        // $06a4    20 2a 06  JSR $062a
        // $06a7    60        RTS
        // $06a8    a2 02     LDX #$02
        // $06aa    b5 10     LDA $10,X
        // $06ac    c5 10     CMP $10
        // $06ae    d0 06     BNE $06b6
        //
        memory[0x06A0] = 0xe6;
        memory[0x06A1] = 0x03;
        memory[0x06A2] = 0xe6;
        memory[0x06A3] = 0x03;
        memory[0x06A4] = 0x20;
        memory[0x06A5] = 0x2a;
        memory[0x06A6] = 0x06;
        memory[0x06A7] = 0x60;
        memory[0x06A8] = 0xa2;
        memory[0x06A9] = 0x02;
        memory[0x06AA] = 0xb5;
        memory[0x06AB] = 0x10;
        memory[0x06AC] = 0xc5;
        memory[0x06AD] = 0x10;
        memory[0x06AE] = 0xd0;
        memory[0x06AF] = 0x06;
        //
        // $06b0    b5 11     LDA $11,X
        // $06b2    c5 11     CMP $11
        // $06b4    f0 09     BEQ $06bf
        // $06b6    e8        INX
        // $06b7    e8        INX
        // $06b8    e4 03     CPX $03
        // $06ba    f0 06     BEQ $06c2
        // $06bc    4c aa 06  JMP $06aa
        // $06bf    4c 35 07  JMP $0735
        //
        memory[0x06B0] = 0xb5;
        memory[0x06B1] = 0x11;
        memory[0x06B2] = 0xc5;
        memory[0x06B3] = 0x11;
        memory[0x06B4] = 0xf0;
        memory[0x06B5] = 0x09;
        memory[0x06B6] = 0xe8;
        memory[0x06B7] = 0xe8;
        memory[0x06B8] = 0xe4;
        memory[0x06B9] = 0x03;
        memory[0x06BA] = 0xf0;
        memory[0x06BB] = 0x06;
        memory[0x06BC] = 0x4c;
        memory[0x06BD] = 0xaa;
        memory[0x06BE] = 0x06;
        memory[0x06BF] = 0x4c;
        //
        // $06c2    60        RTS
        // $06c3    a6 03     LDX $03
        // $06c5    ca        DEX
        // $06c6    8a        TXA
        // $06c7    b5 10     LDA $10,X
        // $06c9    95 12     STA $12,X
        // $06cb    ca        DEX
        // $06cc    10 f9     BPL $06c7
        // $06ce    a5 02     LDA $02
        //
        memory[0x06C0] = 0x35;
        memory[0x06C1] = 0x07;
        memory[0x06C2] = 0x60;
        memory[0x06C3] = 0xa6;
        memory[0x06C4] = 0x03;
        memory[0x06C5] = 0xca;
        memory[0x06C6] = 0x8a;
        memory[0x06C7] = 0xb5;
        memory[0x06C8] = 0x10;
        memory[0x06C9] = 0x95;
        memory[0x06CA] = 0x12;
        memory[0x06CB] = 0xca;
        memory[0x06CC] = 0x10;
        memory[0x06CD] = 0xf9;
        memory[0x06CE] = 0xa5;
        memory[0x06CF] = 0x02;
        //
        // $06d0    4a        LSR A
        // $06d1    b0 09     BCS $06dc
        // $06d3    4a        LSR A
        // $06d4    b0 19     BCS $06ef
        // $06d6    4a        LSR A
        // $06d7    b0 1f     BCS $06f8
        // $06d9    4a        LSR A
        // $06da    b0 2f     BCS $070b
        // $06dc    a5 10     LDA $10
        // $06de    38        SEC
        // $06df    e9 20     SBC #$20
        //
        memory[0x06D0] = 0x4a;
        memory[0x06D1] = 0xb0;
        memory[0x06D2] = 0x09;
        memory[0x06D3] = 0x4a;
        memory[0x06D4] = 0xb0;
        memory[0x06D5] = 0x19;
        memory[0x06D6] = 0x4a;
        memory[0x06D7] = 0xb0;
        memory[0x06D8] = 0x1f;
        memory[0x06D9] = 0x4a;
        memory[0x06DA] = 0xb0;
        memory[0x06DB] = 0x2f;
        memory[0x06DC] = 0xa5;
        memory[0x06DD] = 0x10;
        memory[0x06DE] = 0x38;
        memory[0x06DF] = 0xe9;
        //
        // $06e1    85 10     STA $10
        // $06e3    90 01     BCC $06e6
        // $06e5    60        RTS
        // $06e6    c6 11     DEC $11
        // $06e8    a9 01     LDA #$01
        // $06ea    c5 11     CMP $11
        // $06ec    f0 28     BEQ $0716
        // $06ee    60        RTS
        // $06ef    e6 10     INC $10
        //
        memory[0x06E0] = 0x20;
        memory[0x06E1] = 0x85;
        memory[0x06E2] = 0x10;
        memory[0x06E3] = 0x90;
        memory[0x06E4] = 0x01;
        memory[0x06E5] = 0x60;
        memory[0x06E6] = 0xc6;
        memory[0x06E7] = 0x11;
        memory[0x06E8] = 0xa9;
        memory[0x06E9] = 0x01;
        memory[0x06EA] = 0xc5;
        memory[0x06EB] = 0x11;
        memory[0x06EC] = 0xf0;
        memory[0x06ED] = 0x28;
        memory[0x06EE] = 0x60;
        memory[0x06EF] = 0xe6;
        //
        // $06f1    a9 1f     LDA #$1f
        // $06f3    24 10     BIT $10
        // $06f5    f0 1f     BEQ $0716
        // $06f7    60        RTS
        // $06f8    a5 10     LDA $10
        // $06fa    18        CLC
        // $06fb    69 20     ADC #$20
        // $06fd    85 10     STA $10
        // $06ff    b0 01     BCS $0702
        //
        memory[0x06F0] = 0x10;
        memory[0x06F1] = 0xa9;
        memory[0x06F2] = 0x1f;
        memory[0x06F3] = 0x24;
        memory[0x06F4] = 0x10;
        memory[0x06F5] = 0xf0;
        memory[0x06F6] = 0x1f;
        memory[0x06F7] = 0x60;
        memory[0x06F8] = 0xa5;
        memory[0x06F9] = 0x10;
        memory[0x06FA] = 0x18;
        memory[0x06FB] = 0x69;
        memory[0x06FC] = 0x20;
        memory[0x06FD] = 0x85;
        memory[0x06FE] = 0x10;
        memory[0x06FF] = 0xb0;
        //
        // $0701    60        RTS
        // $0702    e6 11     INC $11
        // $0704    a9 06     LDA #$06
        // $0706    c5 11     CMP $11
        // $0708    f0 0c     BEQ $0716
        // $070a    60        RTS
        // $070b    c6 10     DEC $10
        // $070d    a5 10     LDA $10
        // $070f    29 1f     AND #$1f
        //
        memory[0x0700] = 0x01;
        memory[0x0701] = 0x60;
        memory[0x0702] = 0xe6;
        memory[0x0703] = 0x11;
        memory[0x0704] = 0xa9;
        memory[0x0705] = 0x06;
        memory[0x0706] = 0xc5;
        memory[0x0707] = 0x11;
        memory[0x0708] = 0xf0;
        memory[0x0709] = 0x0c;
        memory[0x070A] = 0x60;
        memory[0x070B] = 0xc6;
        memory[0x070C] = 0x10;
        memory[0x070D] = 0xa5;
        memory[0x070E] = 0x10;
        memory[0x070F] = 0x29;
        //
        // $0711    c9 1f     CMP #$1f
        // $0713    f0 01     BEQ $0716
        // $0715    60        RTS
        // $0716    4c 35 07  JMP $0735
        // $0719    a0 00     LDY #$00
        // $071b    a5 fe     LDA $fe
        // $071d    91 00     STA ($00),Y
        // $071f    60        RTS
        //
        memory[0x0710] = 0x1f;
        memory[0x0711] = 0xc9;
        memory[0x0712] = 0x1f;
        memory[0x0713] = 0xf0;
        memory[0x0714] = 0x01;
        memory[0x0715] = 0x60;
        memory[0x0716] = 0x4c;
        memory[0x0717] = 0x35;
        memory[0x0718] = 0x07;
        memory[0x0719] = 0xa0;
        memory[0x071A] = 0x00;
        memory[0x071B] = 0xa5;
        memory[0x071C] = 0xfe;
        memory[0x071D] = 0x91;
        memory[0x071E] = 0x00;
        memory[0x071F] = 0x60;
        //
        // $0720    a6 03     LDX $03
        // $0722    a9 00     LDA #$00
        // $0724    81 10     STA ($10,X)
        // $0726    a2 00     LDX #$00
        // $0728    a9 01     LDA #$01
        // $072a    81 10     STA ($10,X)
        // $072c    60        RTS
        // $072d    a2 00     LDX #$00
        // $072f    ea        NOP
        //
        memory[0x0720] = 0xa6;
        memory[0x0721] = 0x03;
        memory[0x0722] = 0xa9;
        memory[0x0723] = 0x00;
        memory[0x0724] = 0x81;
        memory[0x0725] = 0x10;
        memory[0x0726] = 0xa2;
        memory[0x0727] = 0x00;
        memory[0x0728] = 0xa9;
        memory[0x0729] = 0x01;
        memory[0x072A] = 0x81;
        memory[0x072B] = 0x10;
        memory[0x072C] = 0x60;
        memory[0x072D] = 0xa2;
        memory[0x072E] = 0x00;
        memory[0x072F] = 0xea;
        //
        // $0730    ea        NOP
        // $0731    ca        DEX
        // $0732    d0 fb     BNE $072f
        // $0734    60        RTS
        //
        memory[0x0730] = 0xea;
        memory[0x0731] = 0xca;
        memory[0x0732] = 0xd0;
        memory[0x0733] = 0xfb;
        memory[0x0734] = 0x60;

        return memory;
    };
}
