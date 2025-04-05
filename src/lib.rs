//! This library crate provides the [`PdfPerm`] trait for the [`Document`] struct from the `lopdf` crate.

use lopdf::{Document, EncryptionState, EncryptionVersion, Permissions, Result as PdfResult};

/// A trait that provides methods for getting and setting PDF permissions.
pub trait PdfPerm {
    // Required methods
    /// Returns the permissions of the PDF document.
    fn permissions(&self) -> Option<Permissions>;
    /// Sets the permissions of the PDF document.
    fn set_permissions(&mut self, permissions: Permissions) -> PdfResult<()>;
}

impl PdfPerm for Document {
    fn permissions(&self) -> Option<Permissions> {
        self.encryption_state
            .as_ref()
            .map(|state| state.permissions())
    }
    fn set_permissions(&mut self, permissions: Permissions) -> PdfResult<()> {
        let version = EncryptionVersion::V1 {
            document: &self,
            owner_password: "",
            user_password: "",
            permissions,
        };
        let state: EncryptionState = version.try_into()?;
        self.encryption_state.replace(state);

        Ok(())
    }
}
