//! This library crate provides the [`PdfPerm`] wrapper for the [`Document`] struct from the `lopdf` crate.

use lopdf::{Document, EncryptionState, Permissions, Result as PdfResult, Error};

/// A trait that provides methods for getting and setting PDF permissions.
// pub trait PdfPerm {
//     // Required methods
//     /// Returns the encryption version of the PDF document.
//     fn encryption_version(&self) -> Option<i64>;
//     /// Returns the permissions of the PDF document.
//     fn permissions(&self) -> Option<Permissions>;
//     /// Sets the permissions of the PDF document.
//     fn set_permissions(&mut self, permissions: Permissions) -> PdfResult<()>;

//     // Provided methods
//     /// Allows all permissions.
//     fn allow_all_permissions(&mut self) -> PdfResult<()> {
//         let permissions = Permissions::all();
//         self.set_permissions(permissions)
//     }
//     /// Denies all permissions.
//     fn deny_all_permissions(&mut self) -> PdfResult<()> {
//         let permissions = Permissions::empty();
//         self.set_permissions(permissions)
//     }
//     /// Resets the permissions to default.
//     ///
//     /// Should be equivalent to [`allow_all_permissions`](PdfPerm::allow_all_permissions).
//     fn reset_permissions(&mut self) -> PdfResult<()> {
//         let permissions = Permissions::default();
//         self.set_permissions(permissions)
//     }
//     /// Allows the specified permissions.
//     fn allow_permissions(&mut self, permissions: Permissions) -> PdfResult<()> {
//         let current_permissions = self.permissions().unwrap_or_else(|| {
//             println!("No permissions found, using default");
//             Permissions::default()
//         });
//         let new_permissions = current_permissions | permissions;
//         self.set_permissions(new_permissions)
//     }
//     /// Denies the specified permissions.
//     fn deny_permissions(&mut self, permissions: Permissions) -> PdfResult<()> {
//         let current_permissions = self.permissions().unwrap_or_else(|| {
//             println!("No permissions found, using default");
//             Permissions::default()
//         });
//         let new_permissions = current_permissions & !permissions;
//         self.set_permissions(new_permissions)
//     }
// }

// impl PdfPerm for Document {
//     fn encryption_version(&self) -> Option<i64> {
//         self.encryption_state
//             .as_ref()
//             .map(|state| state.version())
//     }
//     fn permissions(&self) -> Option<Permissions> {
//         self.encryption_state.as_ref().map(|state| state.permissions())
//     }
//     fn set_permissions(&mut self, permissions: Permissions) -> PdfResult<()> {
//         // if self.is_encrypted() {
//         //     return Err(Error::AlreadyEncrypted);
//         // }
//         // let mut state = self.encryption_state.take().unwrap_or_default();
//         // state.set_permissions(permissions);
//         // self.encryption_state = Some(state);
//         // Ok(())
//         todo!("Implement set_permissions for Document");
//     }
// }

pub struct PdfPerm<'a> {
    document: &'a Document,
}

#[cfg(test)]
mod tests {
    use super::*;
}
