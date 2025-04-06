//! # `pdf-perm` library crate
//!
//! If you are reading this, you are reading the documentation for the `pdf-perm` library crate. For the cli, kindly refer to the README file.
//!
//! This library crate provides several traits related to [`Permissions`].

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use bitflags::Flags;
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

// TODO: Decoupling `ShortFlags` into a separate crate `short-flags`?
/// Trait for [`Flags`] to provide short flag functionality.
pub trait ShortFlags: Flags + Copy {
    // Required constant
    /// The set of defined short flags. Must be of the same length as [`Flags::FLAGS`].
    const SHORT_FLAGS: &'static [char];

    // Provided methods
    /// Parses a character into a [`Flags`].
    fn from_char(c: char) -> Option<Self> {
        if c == '*' {
            return Some(Self::all());
        }
        let index = Self::SHORT_FLAGS.iter().position(|&flag| flag == c)?;
        Some(Self::FLAGS.get(index)?.value().clone())
    }
    /// Parses a string into self, with given short flags.
    #[must_use]
    fn from_str(s: &str) -> Self {
        let mut flags = Self::empty();
        for c in s.chars() {
            if let Some(flag) = Self::from_char(c) {
                flags.insert(flag);
            } else {
                warn!("Invalid permission character: {c}");
            }
        }
        flags
    }
    /// Applies the given modification string.
    fn apply_modification(&mut self, modification: &str) {
        let (first, rest) = modification.split_at(1);
        let flags_mod = Self::from_str(rest);
        match first {
            "+" => self.insert(flags_mod),
            "-" => self.remove(flags_mod),
            "=" => *self = flags_mod,
            _ => warn!("Invalid modification: {modification}"),
        }
    }
    /// Returns a concise summary of the flags, with short flag for set flags and `-` for unset flags.
    fn summary(&self) -> String {
        let mut summary = String::with_capacity(Self::SHORT_FLAGS.len());
        for (short, flag) in Self::SHORT_FLAGS.iter().zip(Self::FLAGS) {
            if self.contains(*flag.value()) {
                summary.push(*short);
            } else {
                summary.push('-');
            }
        }
        summary
    }
}

impl ShortFlags for Permissions {
    const SHORT_FLAGS: &'static [char] = &['p', 'm', 'c', 'a', 'f', 'x', 's', 'q'];
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test `PdfPerm` trait

    // Test `ShortFlags` trait
    #[test]
    fn test_from_str_1() {
        let permissions = Permissions::from_str("pmc");
        let expected = Permissions::PRINTABLE | Permissions::MODIFIABLE | Permissions::COPYABLE;
        assert_eq!(permissions, expected);
    }

    #[test]
    fn test_from_str_2() {
        let permissions = Permissions::from_str("*");
        let expected = Permissions::all();
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
