use crate::bitfield;
use paste::paste;

// Decoder1
bitfield! {
    struct Decoder1(u128);
    get_sty_cpy_mem, set_sty_cpy_mem: 0;
    get_t3_ind_y, set_t3_ind_y: 1;
    get_t2_abs_y, set_t2_abs_y: 2;
    get_t0_iny_dey, set_t0_iny_dey: 3;
}
bitfield! {
    struct Decoder2(u8);
    get, set: 0;
}

struct Decoder {
    ir: u8,
    d1: Decoder1,
    d2: Decoder2,
}

macro_rules! decoder_link {
    ($name:ident, $var:ident) => {
        paste! {
            fn [<get_ $name>](&self) -> bool {
                self.$var.[<get_ $name>]()
            }
            fn [<set_ $name>](&self, value: bool) {
                self.$var.[<set_ $name>](value)
            }
        }
    };
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder {
            ir: 0,
            d1: Decoder1 { 0: 0 },
            d2: Decoder2 { 0: 0 },
        }
    }
    decoder_link!(sty_cpy_mem, d1);
    decoder_link!(t3_ind_y, d1);
    decoder_link!(t2_abs_y, d1);
    decoder_link!(t0_iny_dey, d1);
}
