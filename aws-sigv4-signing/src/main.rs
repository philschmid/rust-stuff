use hex_literal::hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

// /// Takes a message and signs it using AWS secret, time, region keys and service keys.
// fn sign_string(
//     string_to_sign: &str,
//     secret: &str,
//     date: NaiveDate,
//     region: &str,
//     service: &str,
// ) -> String {
//     let date_str = date.format("%Y%m%d").to_string();
//     let date_hmac = hmac(format!("AWS4{}", secret).as_bytes(), date_str.as_bytes())
//         .finalize()
//         .into_bytes();
//     let region_hmac = hmac(date_hmac.as_ref(), region.as_bytes())
//         .finalize()
//         .into_bytes();
//     let service_hmac = hmac(region_hmac.as_ref(), service.as_bytes())
//         .finalize()
//         .into_bytes();
//     let signing_hmac = hmac(service_hmac.as_ref(), b"aws4_request")
//         .finalize()
//         .into_bytes();
//     hex::encode(
//         hmac(signing_hmac.as_ref(), string_to_sign.as_bytes())
//             .finalize()
//             .into_bytes(),
//     )
fn main() {
    // Create alias for HMAC-SHA256
    let secret = "ABC";
    let message = "DEF";
    let mut hmac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("failed to create hmac");
    hmac.update(message.as_bytes());

    // let secret = format!("AWS4{}", secret);
    // let secret = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
}
