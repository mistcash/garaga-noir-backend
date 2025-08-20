use garaga_rs::calldata::full_proof_with_hints::{
    honk::{HonkFlavor, HonkVerificationKey},
    zk_honk::{self, ZKHonkProof},
};

pub fn get_zk_honk_calldata(
    proof_bytes: &[u8],
    public_inputs_bytes: &[u8],
    vk_bytes: &[u8],
    flavor_num: usize,
) -> Result<Vec<String>, String> {
    let proof = ZKHonkProof::from_bytes(&proof_bytes, &public_inputs_bytes)?;
    let vk = HonkVerificationKey::from_bytes(&vk_bytes).map_err(|s| s.to_string())?;
    // Convert usize to HonkFlavor using TryFrom
    let flavor = HonkFlavor::try_from(flavor_num).map_err(|e| e.to_string())?;

    let honk_calldata = zk_honk::get_zk_honk_calldata(&proof, &vk, flavor);

    let honk_calldata_str = honk_calldata?
        .into_iter()
        .map(|biguint| biguint.to_string())
        .collect::<Vec<_>>();
    Ok(honk_calldata_str)
}
