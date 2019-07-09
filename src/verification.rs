//! Verification of digital signatures.

use ring::signature::{verify, ED25519};
use untrusted::Input;

use crate::{signatures::Signature, Error};

/// A digital signature verifier.
pub trait Verifier {
    /// Use a public key to verify a signature against the JSON object that was signed.
    ///
    /// # Parameters
    ///
    /// * public_key: The public key of the key pair used to sign the message.
    /// * signature: The `Signature` to verify.
    /// * message: The message that was signed.
    ///
    /// # Errors
    ///
    /// Returns an error if verification fails.
    fn verify_json(
        &self,
        public_key: &[u8],
        signature: &Signature,
        message: &[u8],
    ) -> Result<(), Error>;
}

/// A verifier for Ed25519 digital signatures.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ed25519Verifier;

impl Verifier for Ed25519Verifier {
    fn verify_json(
        &self,
        public_key: &[u8],
        signature: &Signature,
        message: &[u8],
    ) -> Result<(), Error> {
        verify(
            &ED25519,
            Input::from(public_key),
            Input::from(message),
            Input::from(signature.as_bytes()),
        )
        .map_err(|_| Error::new("signature verification failed"))
    }
}