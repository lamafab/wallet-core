// TODO: Should breez export this?
use bitcoin::bech32::{u5, FromBase32, ToBase32};
use bitcoin::hashes::{sha256, Hash};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use lightning_invoice::{RawDataPart, RawHrp, RawInvoice, SignedRawInvoice};

/*
From: vls-core

```
    // Sign a BOLT-11 invoice
    pub(crate) fn do_sign_invoice

```
 */
fn sign_invoice(invoice: RawInvoice, secret_key: SecretKey) -> Result<SignedRawInvoice, ()> {
    let hrp_bytes = invoice.hrp.to_string().as_bytes().to_vec();
    let data_bytes = invoice.data.to_base32();

    let hrp: RawHrp = String::from_utf8(hrp_bytes.to_vec())
        .unwrap()
        .parse()
        .unwrap();
    let data = RawDataPart::from_base32(&data_bytes).unwrap();
    let raw_invoice = RawInvoice { hrp, data };

    let invoice_preimage = construct_invoice_preimage(&hrp_bytes, &data_bytes);
    let secp_ctx = Secp256k1::signing_only();
    let hash = sha256::Hash::hash(&invoice_preimage);
    let message = bitcoin::secp256k1::Message::from_slice(hash.as_ref()).unwrap();
    let sig = secp_ctx.sign_ecdsa_recoverable(&message, &secret_key);

    let signed = raw_invoice.sign::<_, ()>(|_| Ok(sig)).unwrap();

    Ok(signed)
}

/// Construct the invoice's HRP and signatureless data into a preimage to be hashed.
pub fn construct_invoice_preimage(hrp_bytes: &[u8], data_without_signature: &[u5]) -> Vec<u8> {
    let mut preimage = Vec::<u8>::from(hrp_bytes);

    let mut data_part = Vec::from(data_without_signature);
    let overhang = (data_part.len() * 5) % 8;
    if overhang > 0 {
        // add padding if data does not end at a byte boundary
        data_part.push(u5::try_from_u8(0).unwrap());

        // if overhang is in (1..3) we need to add u5(0) padding two times
        if overhang < 3 {
            data_part.push(u5::try_from_u8(0).unwrap());
        }
    }

    preimage.extend_from_slice(
        &Vec::<u8>::from_base32(&data_part)
            .expect("No padding error may occur due to appended zero above."),
    );
    preimage
}
