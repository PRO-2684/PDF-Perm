//! # `pdf-perm` library crate
//!
//! If you are reading this, you are reading the documentation for the `pdf-perm` library crate. For the cli, kindly refer to the README file.
//!
//! This library crate provides several traits related to [`Permissions`].

use log::{debug, error, warn};
use lopdf::{Document, EncryptionState, EncryptionVersion, Error, Permissions, Result as PdfResult};

/// A trait that provides methods for getting and setting [`Permissions`] on a [`Document`].
pub trait PdfPerm {
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
        if self.is_encrypted() {
            error!("Does not support setting permissions on encrypted documents");
            return Err(Error::AlreadyEncrypted);
        }
        let version = EncryptionVersion::V1 {
            document: &self,
            owner_password: "",
            user_password: "",
            permissions,
        };
        let state: EncryptionState = version.try_into()?;
        debug!("Encryption state: {state:?}");
        self.encryption_state.replace(state);

        Ok(())
    }
}

/// A trait that provides methods for parsing and applying [`Permissions`].
pub trait PermissionExt {
    /// Parses a character into a [`Permissions`], with only one permission bit set.
    fn from_char(c: char) -> Option<Permissions>;
    /// Parses a string into a [`Permissions`], with multiple permission bits set.
    fn from_str(s: &str) -> Permissions {
        let mut permissions = Permissions::empty();
        for c in s.chars() {
            if let Some(permission) = Self::from_char(c) {
                permissions.insert(permission);
            } else {
                warn!("Invalid permission character: {c}");
            }
        }
        permissions
    }
}

impl PermissionExt for Permissions {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'p' => Some(Permissions::PRINTABLE),
            'm' => Some(Permissions::MODIFIABLE),
            'c' => Some(Permissions::COPYABLE),
            'a' => Some(Permissions::ANNOTABLE),
            'f' => Some(Permissions::FILLABLE),
            'x' => Some(Permissions::COPYABLE_FOR_ACCESSIBILITY),
            's' => Some(Permissions::ASSEMBLABLE),
            'q' => Some(Permissions::PRINTABLE_IN_HIGH_QUALITY),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test `PdfPerm` trait

    // Test `PermissionExt` trait
    #[test]
    fn test_permission_ext() {
        // Test single character permissions
        let printable = Permissions::from_char('p');
        assert_eq!(printable, Some(Permissions::PRINTABLE));
        let modifiable = Permissions::from_char('m');
        assert_eq!(modifiable, Some(Permissions::MODIFIABLE));
        let copyable = Permissions::from_char('c');
        assert_eq!(copyable, Some(Permissions::COPYABLE));
        let not_found = Permissions::from_char('z');
        assert_eq!(not_found, None);

        // Test multiple character permissions
        let permissions = Permissions::from_str("pmc");
        assert!(permissions.contains(Permissions::PRINTABLE));
        assert!(permissions.contains(Permissions::MODIFIABLE));
        assert!(permissions.contains(Permissions::COPYABLE));
        assert!(!permissions.contains(Permissions::ANNOTABLE));
    }
}
