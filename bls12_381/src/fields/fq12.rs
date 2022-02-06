use crate::*;
use ark_ff::{MontFp, fields::*};

pub type Fq12 = Fp12<Fq12Config>;

#[derive(Clone, Copy)]
pub struct Fq12Config;

impl Fp12Parameters for Fq12Config {
    type Fp6Params = Fq6Config;

    const NONRESIDUE: Fq6 = MontFp!(Fq6, FQ2_ZERO, FQ2_ONE, FQ2_ZERO);

    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "1"),
            MontFp!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "3850754370037169011952147076051364057158807420970682438676050522613628423219637725072182697113062777891589506424760"),
            MontFp!(Fq, "151655185184498381465642749684540099398075398968325446656007613510403227271200139370504932015952886146304766135027"),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "793479390729215512621379701633421447060886740281060493010456487427281649075476305620758731620351"),
            MontFp!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530"),
            MontFp!(Fq, "1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257"),
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "793479390729215512621379701633421447060886740281060493010456487427281649075476305620758731620350"),
            MontFp!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "3125332594171059424908108096204648978570118281977575435832422631601824034463382777937621250592425535493320683825557"),
            MontFp!(Fq, "877076961050607968509681729531255177986764537961432449499635504522207616027455086505066378536590128544573588734230"),
        ),
        // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "-1"),
            MontFp!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "151655185184498381465642749684540099398075398968325446656007613510403227271200139370504932015952886146304766135027"),
            MontFp!(Fq, "3850754370037169011952147076051364057158807420970682438676050522613628423219637725072182697113062777891589506424760"),
        ),
        // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939436"),
            MontFp!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257"),
            MontFp!(Fq, "2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530"),
        ),
        // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939437"),
            MontFp!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
        MontFp!(Fq2,
            MontFp!(Fq, "877076961050607968509681729531255177986764537961432449499635504522207616027455086505066378536590128544573588734230"),
            MontFp!(Fq, "3125332594171059424908108096204648978570118281977575435832422631601824034463382777937621250592425535493320683825557"),
        ),
    ];
}
