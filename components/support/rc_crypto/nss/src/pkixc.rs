/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::util::ensure_nss_initialized;
use crate::error::*;

use nss_sys::PRErrorCode;

pub fn verify_code_signing_certificate_chain(certificates: Vec<&[u8]>, seconds_since_epoch: u64, root_sha256_hash: &[u8], hostname: &str) -> Result<(), > {
    ensure_nss_initialized();

    let mut cert_lens: Vec<u16> = vec![];
    for certificate in &certificates {
        cert_lens.push(certificate.len() as u16);
    }

    // I cannot figure out how to get rid of `mut` here, because of
    // ``const uint8_t** certificates`` param in nss_sys.
    let mut p_certificates: Vec<_> = certificates.iter()
        .map(|c| c.as_ptr())
        .collect();

    let mut out: PRErrorCode = 0;

    let result = unsafe {
        nss_sys::VerifyCodeSigningCertificateChain(
            p_certificates.as_mut_ptr(),
            cert_lens.as_ptr(),
            certificates.len(),
            seconds_since_epoch,
            root_sha256_hash.as_ptr(),
            hostname.as_ptr(),
            hostname.len(),
            &mut out,
        )
    };

    if !result {
        return Err(ErrorKind::NSSError(out, "invalid chain of trust".into()).into());
    }

    Ok(())
}
