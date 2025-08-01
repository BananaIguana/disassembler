#[allow(unused_macros)]
macro_rules! make_arm64_instruction {
    (
        $name:ident {
            // make sf optional…
            $( sf     = $sf:literal, )?
            // …but op is still required
            op     = $op:literal,
            S      = $S:literal,
            sh     = $sh:literal,
            imm12  = $imm12_start:literal - $imm12_end:literal,
            Rn     = $Rn_start:literal   - $Rn_end:literal,
            Rd     = $Rd_start:literal   - $Rd_end:literal $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub struct $name {
            // only emit this field if sf was passed
            $( pub sf:    u32, )?
            pub op:    u32,
            pub s:     u32,
            pub sh:    u32,
            pub imm12: u32,
            pub source:    u32,
            pub destination:    u32,
        }
        

        impl $name {
            pub fn from(instruction: u32) -> Self
            {
                const fn mask(width: u32, shift: u32) -> u32
                {
                    ((1 << width) - 1) << shift
                }

                // conditionally bind sf
                $( let sf    = (instruction & mask(1, $sf)) >> $sf; )?
                let op    = (instruction & mask(1, $op)) >> $op;
                let s     = (instruction & mask(1, $S))  >> $S;
                let sh    = (instruction & mask(1, $sh)) >> $sh;
                let imm12 = (instruction & mask($imm12_end - $imm12_start + 1, $imm12_start))
                               >> $imm12_start;
                let Rn    = (instruction & mask($Rn_end   - $Rn_start   + 1, $Rn_start))
                               >> $Rn_start;
                let Rd    = (instruction & mask($Rd_end   - $Rd_start   + 1, $Rd_start))
                               >> $Rd_start;

                $name {
                    // only include sf in the struct initializer if it was bound
                    $( sf, )?
                    op,
                    s,
                    sh,
                    imm12,
                    Rn,
                    Rd,
                }
            }
        }
    }
}