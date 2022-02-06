use ark_ff::{
    biginteger::{BigInt, BigInteger384 as BigInteger},
    fields::*,
};

pub struct FqConfig;
pub type Fq = Fp384<MontBackend<FqConfig, 6>>;

impl ark_ff::MontConfig<6> for FqConfig {
    /// MODULUS = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
    #[rustfmt::skip]
    const MODULUS: BigInteger = ark_ff::BigInt!("258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177");

    /// GENERATOR = 15
    #[rustfmt::skip]
    const GENERATOR: Fq = ark_ff::MontFp!(Fq, "15");

    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = ark_ff::MontFp!(Fq, "32863578547254505029601261939868325669770508939375122462904745766352256812585773382134936404344547323199885654433");
}

#[allow(dead_code)]
pub const FQ_ONE: Fq = Fq::new(FqConfig::R);
#[allow(dead_code)]
pub const FQ_ZERO: Fq = Fq::new(BigInt::new([0, 0, 0, 0, 0, 0]));
