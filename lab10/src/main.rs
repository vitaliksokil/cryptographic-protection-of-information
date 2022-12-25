use rand::RngCore;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use jubjub::{AffinePoint, Base, ExtendedPoint, Fr, Scalar};


fn main() {
    let pk = PrivateKey::new(&mut rand::thread_rng());
    let pubkey = PublicKey::derive(pk);

    dbg!(pk,pubkey);

    let message = "Signed Message!!!";
    let (r, s, computed_scalar) = pubkey.sign(&mut rand::thread_rng(), message, pk);

    dbg!(r,s,computed_scalar);

    let z_scalar = gen_message_scalar(message);

    dbg!(z_scalar);

    let valid = pubkey.verify_sig(r, s, z_scalar);

    dbg!(valid);
    println!("Signature verification result: {}  ", valid);
}

const FULL_GENERATOR: AffinePoint = AffinePoint::from_raw_unchecked(
    Base::from_raw([
        0xe4b3_d35d_f1a7_adfe,
        0xcaf5_5d1b_29bf_81af,
        0x8b0f_03dd_d60a_8187,
        0x62ed_cbb8_bf37_87c8,
    ]),
    Base::from_raw([0xb, 0x0, 0x0, 0x0]),
);

pub fn generate_random_scalar<T: RngCore>(rng: &mut T) -> Scalar {
    let mut random_bytes = [0u8; 64];
    rng.fill_bytes(&mut random_bytes);
    return Scalar::from_bytes_wide(&random_bytes);
}


#[derive(Default, Clone, Copy, Debug)]
pub struct PrivateKey(Scalar);

impl PrivateKey {
    pub fn new<T: RngCore>(rng: &mut T) -> Self {
        return PrivateKey(generate_random_scalar(rng));
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct PublicKey(ExtendedPoint);

impl PublicKey {
    pub fn derive(pk: PrivateKey) -> Self {
        return PublicKey(FULL_GENERATOR * pk.0);
    }

    pub fn sign<T: RngCore>(self, rng: &mut T, message: &str, pk: PrivateKey) -> (Fr, Fr, Fr) {
        let z_scalar = gen_message_scalar(message);
        let mut k;
        assert!(self.0 == FULL_GENERATOR * pk.0, "Invalid private key");

        loop {
            k = generate_random_scalar(rng);
            let curve_point = FULL_GENERATOR * k;

            if curve_point.is_identity().unwrap_u8() == 0 {
                let affine = AffinePoint::from(curve_point);
                let possible_r = Scalar::from_bytes(&affine.get_u().to_bytes());

                if possible_r.is_some().unwrap_u8() == 1 {
                    let r = possible_r.unwrap();
                    let inner = z_scalar + r.mul(&pk.0);
                    let s = k.invert().unwrap().mul(&inner);

                    if self.verify_sig(r, s, z_scalar) {
                        return (r, s, z_scalar);
                    }
                }
            }
        }
    }

    pub fn verify_sig(self, r: Fr, s: Fr, z_scalar: Fr) -> bool {
        let s_invert = &s.invert().unwrap();
        let u_1 = z_scalar.mul(s_invert);
        let u_2 = r.mul(s_invert);

        let point = (FULL_GENERATOR * u_1) + (self.0 * u_2);
        let possible_r = Scalar::from_bytes(&AffinePoint::from(point).get_u().to_bytes());

        if possible_r.is_some().unwrap_u8() == 1 {
            return r == possible_r.unwrap();
        }

        return false;
    }
}

pub fn gen_message_scalar(message: &str) -> Fr {
    let mut hasher = Sha256::new();
    hasher.input_str(message);
    let hex = hasher.result_str();
    let e = hex.as_bytes();
    let z: [u8; 64] = e[0..64].try_into().unwrap();
    Scalar::from_bytes_wide(&z)
}
