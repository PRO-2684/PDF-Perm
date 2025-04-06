//! # `pdf-perm` library crate
//!
//! If you are reading this, you are reading the documentation for the `pdf-perm` library crate. For the cli, kindly refer to the README file.
//!
//! This library crate provides several traits related to [`Permissions`].

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use log::{debug, error, warn};
use lopdf::{
    Document, EncryptionState, EncryptionVersion, Error, Permissions, Result as PdfResult,
};

/// Trait for non-encrypted PDF [`Document`]s, allowing for easy getting and setting of [`Permissions`].
pub trait PdfPerm {
    /// Returns the permissions of the PDF document.
    fn permissions(&self) -> Option<Permissions>;
    /// Sets the permissions of the PDF document.
    ///
    /// # Errors
    ///
    /// Returns [`Error::AlreadyEncrypted`] if the document is encrypted, or other variants if operation fails.
    fn set_permissions(&mut self, permissions: Permissions) -> PdfResult<()>;
}

impl PdfPerm for Document {
    fn permissions(&self) -> Option<Permissions> {
        self.encryption_state
            .as_ref()
            .map(EncryptionState::permissions)
    }
    fn set_permissions(&mut self, permissions: Permissions) -> PdfResult<()> {
        if self.is_encrypted() {
            error!("Does not support setting permissions on encrypted documents");
            return Err(Error::AlreadyEncrypted);
        }
        let version = EncryptionVersion::V1 {
            document: self,
            owner_password: "",
            user_password: "",
            permissions,
        };
        let state: EncryptionState = version.try_into()?;
        debug!("Encryption state: {state:?}");
        self.encrypt(&state)?;

        Ok(())
    }
}

/// Trait for [`Permissions`] to provide parsing and modification functionality.
pub trait PermissionExt {
    /// Parses a character into a [`Permissions`], with only one permission bit set.
    fn from_char(c: char) -> Option<Permissions>;
    /// Parses a string into a [`Permissions`], with multiple permission bits set.
    #[must_use]
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
    /// Applies the given permission modification string. See `Permission` section of the README.
    fn apply_modification(&mut self, modification: &str);
}

impl PermissionExt for Permissions {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'p' => Some(Self::PRINTABLE),
            'm' => Some(Self::MODIFIABLE),
            'c' => Some(Self::COPYABLE),
            'a' => Some(Self::ANNOTABLE),
            'f' => Some(Self::FILLABLE),
            'x' => Some(Self::COPYABLE_FOR_ACCESSIBILITY),
            's' => Some(Self::ASSEMBLABLE),
            'q' => Some(Self::PRINTABLE_IN_HIGH_QUALITY),
            '*' => Some(Self::all()),
            _ => None,
        }
    }
    fn apply_modification(&mut self, modification: &str) {
        let (first, rest) = modification.split_at(1);
        let permission = Self::from_str(rest);
        match first {
            "+" => self.insert(permission),
            "-" => self.remove(permission),
            "=" => *self = permission,
            _ => warn!("Invalid permission modification: {modification}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test `PdfPerm` trait

    // Test `PermissionExt` trait
    #[test]
    fn test_from_char() {
        let printable = Permissions::from_char('p');
        assert_eq!(printable, Some(Permissions::PRINTABLE));
        let modifiable = Permissions::from_char('m');
        assert_eq!(modifiable, Some(Permissions::MODIFIABLE));
        let copyable = Permissions::from_char('c');
        assert_eq!(copyable, Some(Permissions::COPYABLE));
        let not_found = Permissions::from_char('z');
        assert_eq!(not_found, None);
        let all = Permissions::from_char('*');
        assert_eq!(all, Some(Permissions::all()));
    }

    #[test]
    fn test_from_str() {
        let permissions = Permissions::from_str("pmc");
        let expected = Permissions::PRINTABLE | Permissions::MODIFIABLE | Permissions::COPYABLE;
        assert_eq!(permissions, expected);
    }

    #[test]
    fn test_apply_modification() {
        let mut permissions = Permissions::empty();
        permissions.apply_modification("+p");
        assert_eq!(permissions, Permissions::PRINTABLE);
        permissions.apply_modification("-p");
        assert_eq!(permissions, Permissions::empty());
        permissions.apply_modification("=m");
        assert_eq!(permissions, Permissions::MODIFIABLE);
    }
}
