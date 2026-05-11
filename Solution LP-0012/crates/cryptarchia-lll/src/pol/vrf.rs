use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::pol::commitment::hash_bytes32;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeadershipVrfKeypair {
    pub secret: [u8; 32],
    pub public: [u8; 32],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeadershipVrfProof {
    pub output: [u8; 32],
    pub proof: [u8; 64],
    pub public_key: [u8; 32],
}

#[cfg(feature = "std")]
impl serde::Serialize for LeadershipVrfProof {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("LeadershipVrfProof", 3)?;
        st.serialize_field("output", &self.output)?;
        st.serialize_field("proof", &self.proof.as_ref())?;
        st.serialize_field("public_key", &self.public_key)?;
        st.end()
    }
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for LeadershipVrfProof {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Raw {
            output: [u8; 32],
            proof: alloc::vec::Vec<u8>,
            public_key: [u8; 32],
        }
        let r = Raw::deserialize(d)?;
        if r.proof.len() != 64 {
            return Err(serde::de::Error::custom("proof must be 64 bytes"));
        }
        let mut proof = [0u8; 64];
        proof.copy_from_slice(&r.proof);
        Ok(LeadershipVrfProof {
            output: r.output,
            proof,
            public_key: r.public_key,
        })
    }
}

pub fn derive_epoch_vrf_keypair(
    leader_secret: &[u8; 32],
    epoch_seed: &[u8; 32],
) -> LeadershipVrfKeypair {
    let secret = hash_bytes32(
        "cryptarchia/pol/v2/epoch-vrf-secret",
        &[leader_secret, epoch_seed],
    );
    let signing_key = SigningKey::from_bytes(&secret);
    let public = signing_key.verifying_key().to_bytes();

    LeadershipVrfKeypair { secret, public }
}

pub fn prove_vrf(keypair: &LeadershipVrfKeypair, message: &[u8]) -> LeadershipVrfProof {
    let signing_key = SigningKey::from_bytes(&keypair.secret);
    let signature = signing_key.sign(message);
    let output = hash_bytes32(
        "cryptarchia/pol/v2/vrf-output",
        &[signature.to_bytes().as_ref(), message],
    );

    LeadershipVrfProof {
        output,
        proof: signature.to_bytes(),
        public_key: keypair.public,
    }
}

pub fn verify_vrf(
    public_key: &[u8; 32],
    message: &[u8],
    proof: &LeadershipVrfProof,
) -> bool {
    if public_key != &proof.public_key {
        return false;
    }

    let vk = match VerifyingKey::from_bytes(public_key) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let sig = Signature::from_bytes(&proof.proof);

    if vk.verify(message, &sig).is_err() {
        return false;
    }

    let expected = hash_bytes32(
        "cryptarchia/pol/v2/vrf-output",
        &[proof.proof.as_ref(), message],
    );

    expected == proof.output
}
