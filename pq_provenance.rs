#![allow(unused)]
// pq_provenance.rs v2.0
// Post-Quantum Cryptography provenance signer for Master_Ring0.md
// Replaces GPG/RSA with Dilithium (lattice) + SPHINCS+ (hash-based) for quantum resistance
// OpenSSF-aligned: produces detached signature .sig file verifiable with pqcrypto tools

use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage, DetachedSignature};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn load_or_generate_keys(secret_key_path: &str, public_key_path: &str) -> io::Result<(
    pqcrypto_dilithium::dilithium2::SecretKey,
    pqcrypto_sphincsplus::sphincssha256128ssimple::SecretKey,
    pqcrypto_dilithium::dilithium2::PublicKey,
    pqcrypto_sphincsplus::sphincssha256128ssimple::PublicKey
)> {
    let (d_pk_dummy, d_sk_dummy) = pqcrypto_dilithium::dilithium2::keypair();
    let d_sk_len = d_sk_dummy.as_bytes().len();
    let d_pk_len = d_pk_dummy.as_bytes().len();

    let (s_pk_dummy, s_sk_dummy) = pqcrypto_sphincsplus::sphincssha256128ssimple::keypair();
    let s_sk_len = s_sk_dummy.as_bytes().len();
    let s_pk_len = s_pk_dummy.as_bytes().len();

    if Path::new(secret_key_path).exists() && Path::new(public_key_path).exists() {
        let mut sk_bytes = Vec::new();
        File::open(secret_key_path)?.read_to_end(&mut sk_bytes)?;
        let mut pk_bytes = Vec::new();
        File::open(public_key_path)?.read_to_end(&mut pk_bytes)?;

        if sk_bytes.len() == d_sk_len + s_sk_len && pk_bytes.len() == d_pk_len + s_pk_len {
            let d_sk = pqcrypto_dilithium::dilithium2::SecretKey::from_bytes(&sk_bytes[..d_sk_len])
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;
            let s_sk = pqcrypto_sphincsplus::sphincssha256128ssimple::SecretKey::from_bytes(&sk_bytes[d_sk_len..])
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;
            let d_pk = pqcrypto_dilithium::dilithium2::PublicKey::from_bytes(&pk_bytes[..d_pk_len])
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;
            let s_pk = pqcrypto_sphincsplus::sphincssha256128ssimple::PublicKey::from_bytes(&pk_bytes[d_pk_len..])
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;
            return Ok((d_sk, s_sk, d_pk, s_pk));
        }
    }

    // Generate new keypair
    let (d_pk, d_sk) = pqcrypto_dilithium::dilithium2::keypair();
    let (s_pk, s_sk) = pqcrypto_sphincsplus::sphincssha256128ssimple::keypair();

    let mut sk_bytes = d_sk.as_bytes().to_vec();
    sk_bytes.extend_from_slice(s_sk.as_bytes());
    File::create(secret_key_path)?.write_all(&sk_bytes)?;

    let mut pk_bytes = d_pk.as_bytes().to_vec();
    pk_bytes.extend_from_slice(s_pk.as_bytes());
    File::create(public_key_path)?.write_all(&pk_bytes)?;

    Ok((d_sk, s_sk, d_pk, s_pk))
}

pub fn load_public_keys(public_key_path: &str) -> io::Result<(
    pqcrypto_dilithium::dilithium2::PublicKey,
    pqcrypto_sphincsplus::sphincssha256128ssimple::PublicKey
)> {
    let (d_pk_dummy, _) = pqcrypto_dilithium::dilithium2::keypair();
    let d_pk_len = d_pk_dummy.as_bytes().len();

    let (s_pk_dummy, _) = pqcrypto_sphincsplus::sphincssha256128ssimple::keypair();
    let s_pk_len = s_pk_dummy.as_bytes().len();

    let mut pk_bytes = Vec::new();
    File::open(public_key_path)?.read_to_end(&mut pk_bytes)?;

    if pk_bytes.len() != d_pk_len + s_pk_len {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid public key file size"));
    }

    let d_pk = pqcrypto_dilithium::dilithium2::PublicKey::from_bytes(&pk_bytes[..d_pk_len])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;
    let s_pk = pqcrypto_sphincsplus::sphincssha256128ssimple::PublicKey::from_bytes(&pk_bytes[d_pk_len..])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;

    Ok((d_pk, s_pk))
}

pub fn sign_pq(data: &[u8], secret_key_path: &str) -> Vec<u8> {
    let public_key_path = format!("{}.pub", secret_key_path);
    let (d_sk, s_sk, _, _) = load_or_generate_keys(secret_key_path, &public_key_path)
        .expect("Failed to load or generate signing keypair");

    let signature = pqcrypto_dilithium::dilithium2::detached_sign(data, &d_sk);
    let sphincs_sig = pqcrypto_sphincsplus::sphincssha256128ssimple::detached_sign(data, &s_sk);

    let mut sig_blob = signature.as_bytes().to_vec();
    sig_blob.extend_from_slice(sphincs_sig.as_bytes());
    sig_blob
}

pub fn verify_pq(data: &[u8], signature: &[u8], public_key_path: &str) -> bool {
    let (d_pk, s_pk) = match load_public_keys(public_key_path) {
        Ok(keys) => keys,
        Err(_) => return false,
    };

    let (_, d_sk_dummy) = pqcrypto_dilithium::dilithium2::keypair();
    let d_sig_len = pqcrypto_dilithium::dilithium2::detached_sign(b"", &d_sk_dummy).as_bytes().len();
    if signature.len() < d_sig_len {
        return false;
    }
    let (d_sig_bytes, s_sig_bytes) = signature.split_at(d_sig_len);

    let d_sig = match pqcrypto_dilithium::dilithium2::DetachedSignature::from_bytes(d_sig_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let s_sig = match pqcrypto_sphincsplus::sphincssha256128ssimple::DetachedSignature::from_bytes(s_sig_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let d_ok = pqcrypto_dilithium::dilithium2::verify_detached_signature(&d_sig, data, &d_pk).is_ok();
    let s_ok = pqcrypto_sphincsplus::sphincssha256128ssimple::verify_detached_signature(&s_sig, data, &s_pk).is_ok();

    d_ok && s_ok
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: pq_provenance <file> [verify]");
        std::process::exit(1);
    }
    let filepath = &args[1];
    let mut f = File::open(filepath).unwrap();
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();

    let secret_key_path = "secure_key.bin";
    let public_key_path = "secure_key.bin.pub";

    if args.len() >= 3 && args[2] == "verify" {
        let sigpath = format!("{}.sig", filepath);
        let mut sigf = File::open(&sigpath).unwrap();
        let mut signature = Vec::new();
        sigf.read_to_end(&mut signature).unwrap();

        if verify_pq(&data, &signature, public_key_path) {
            println!("[✓] PQ signature VERIFIED successfully.");
        } else {
            println!("[❌] PQ signature VERIFICATION FAILED.");
            std::process::exit(1);
        }
    } else {
        let sig = sign_pq(&data, secret_key_path);
        let mut sigf = File::create(format!("{}.sig", filepath)).unwrap();
        sigf.write_all(&sig).unwrap();
        println!("[Ring-0 v2.0] PQ provenance signature written (Dilithium + SPHINCS+)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let data = b"Khawrizm Sovereign Data Refinery Verification Payload";
        let secret_key_path = "test_secure_key.bin";
        let public_key_path = "test_secure_key.bin.pub";

        // Clean up previous files if any
        let _ = std::fs::remove_file(secret_key_path);
        let _ = std::fs::remove_file(public_key_path);

        let sig = sign_pq(data, secret_key_path);
        assert!(!sig.is_empty(), "Signature should not be empty");

        let verified = verify_pq(data, &sig, public_key_path);
        assert!(verified, "Valid signature verification should succeed");

        // Verify with corrupted data
        let corrupted_data = b"Khawrizm Sovereign Data Refinery Verification Payload!";
        let verified_corrupted = verify_pq(corrupted_data, &sig, public_key_path);
        assert!(!verified_corrupted, "Corrupted data verification should fail");

        // Verify with corrupted signature
        let mut corrupted_sig = sig.clone();
        if !corrupted_sig.is_empty() {
            corrupted_sig[0] ^= 0xFF;
        }
        let verified_bad_sig = verify_pq(data, &corrupted_sig, public_key_path);
        assert!(!verified_bad_sig, "Corrupted signature verification should fail");

        // Clean up
        let _ = std::fs::remove_file(secret_key_path);
        let _ = std::fs::remove_file(public_key_path);
    }

    #[test]
    fn test_signature_fuzzing() {
        let data = b"Khawrizm Sovereign OS Fuzzing Payload";
        let secret_key_path = "test_fuzz_key.bin";
        let public_key_path = "test_fuzz_key.bin.pub";

        // Generate a valid keypair first to ensure verify_pq has keys to read
        let _ = std::fs::remove_file(secret_key_path);
        let _ = std::fs::remove_file(public_key_path);
        let _ = load_or_generate_keys(secret_key_path, public_key_path);

        // Minimal seedable XORshift PRNG
        let mut seed: u64 = 0x123456789ABCDEF;
        for _ in 0..100 {
            let mut rand_sig = vec![0u8; 4096];
            for byte in rand_sig.iter_mut() {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                *byte = (seed & 0xFF) as u8;
            }
            let verified = verify_pq(data, &rand_sig, public_key_path);
            assert!(!verified, "Random fuzz signature must fail verification");
        }

        // Clean up
        let _ = std::fs::remove_file(secret_key_path);
        let _ = std::fs::remove_file(public_key_path);
    }
}
