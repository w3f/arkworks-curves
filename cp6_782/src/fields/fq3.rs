use crate::{fields::FQ_ZERO, Fq};
use ark_ff::{
    MontFp,
    fields::fp3::{Fp3, Fp3Parameters},
    Field,
};

pub type Fq3 = Fp3<Fq3Config>;

pub struct Fq3Config;

impl Fp3Parameters for Fq3Config {
    type Fp = Fq;

    /// NONRESIDUE = 13
    #[rustfmt::skip]
    const NONRESIDUE: Fq = MontFp!(Fq, "13");

    const TWO_ADICITY: u32 = 3;

    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: &'static [u64] = &[
        0x62730e2cd2029617,
        0x660647f735cb88cf,
        0x274359d60784f69d,
        0x83067194eb102629,
        0x54ea4a12a9381160,
        0xade0b24e398dac25,
        0xb476ae9f927e81cb,
        0x220fd4a9178adc3b,
        0x57e0cb9b0569745b,
        0xba15024addc8f52e,
        0x145b9bc116144ab6,
        0x6bc2260726e88b15,
        0x51da6bf151066474,
        0x9fd1b3190f6320cf,
        0x2097bfb7bf4167b0,
        0x27c35b1e7e628e09,
        0x94f80c9d623dd9bb,
        0x20bfa6d5bf31e7d3,
        0x19fb862c049d3a8,
        0xdf4c5efe04c0cec1,
        0x32c9a8abe9b50297,
        0x268d5c2076b44f0a,
        0x76027ec67b23ca21,
        0x248d61e0c45d270,
        0x419cd0d1d6be027e,
        0xbcd8dc3b1986ef18,
        0x73093d8719c862c2,
        0x651d60f8f9f6fcd9,
        0x8dabebe38a09b261,
        0xfa85b5a9e180cd3f,
        0x6a97fc618f319fb7,
        0xce08b93a5652a8e1,
        0x37525cbc4ba24cf9,
        0xb104c580df9d2150,
        0x1407c1bfe240a89d,
        0x34c96a73372daf9a,
        0x2b87fda171,
    ];

    #[rustfmt::skip]
    const QUADRATIC_NONRESIDUE_TO_T: (Fq, Fq, Fq) = (
        MontFp!(Fq, "5759691735434357221228070840130186543101559976323700017469395641639510585333061695996665166662748527158637897523704071820491869715512532675375604262649010727161924084052120196921150869218319839231115277876207074651754402338718419191428"),
        FQ_ZERO,
        FQ_ZERO,
    );

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP3_C1: &'static [Fq] = &[
        MontFp!(Fq, "1"),
        MontFp!(Fq, "2416169158604010336818399199316106389588878314690767988978701685873498866746813334102117883272276610365242925950967572554030909749205624998805208910209389668659757274773858916683688639755413288353778854399286396639505385648830027756861"),
        MontFp!(Fq, "19953705140271686593528343007184948545186721150606416593204794941773275185711211206130361134875604609812811649452874332003866805473427708373329547516091672819022569300184729084448897691707139947053705234905346679611277243843727293068715"),
    ];

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP3_C2: &'static [Fq] = &[
        MontFp!(Fq, "1"),
        MontFp!(Fq, "19953705140271686593528343007184948545186721150606416593204794941773275185711211206130361134875604609812811649452874332003866805473427708373329547516091672819022569300184729084448897691707139947053705234905346679611277243843727293068715"),
        MontFp!(Fq, "2416169158604010336818399199316106389588878314690767988978701685873498866746813334102117883272276610365242925950967572554030909749205624998805208910209389668659757274773858916683688639755413288353778854399286396639505385648830027756861"),
    ];

    #[inline(always)]
    fn mul_fp_by_nonresidue(fe: &Self::Fp) -> Self::Fp {
        let original = *fe;
        let mut four_fe = fe.double();
        four_fe.double_in_place();
        let eight_fe = four_fe.double();
        eight_fe + &four_fe + &original
    }
}
