// Written by: Christopher Gholmieh
// Crates:
use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};

use serde::{Deserialize, Serialize};

use std::fs;

// Crate:
use crate::constants::{AES_ENCRYPTION_KEY, NONCE};

// Structures:
#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    /* Image: */
    pub image: Option<Image>,

    /* Checks: */
    pub checks: Option<Vec<CheckWrapper>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    /* Title: */
    pub title: Option<String>,

    /* User: */
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckWrapper {
    /* Check: */
    pub check: Check,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Check {
    /* Description: */
    pub description: String,

    /* Points: */
    pub points: i8,

    /* Pass: */
    #[serde(default)]
    pub pass: Vec<String>,
}

// Parser:
pub struct Parser {
    /* Path: */
    path: String,

    /* Contents: */
    contents: Option<String>,

    /* Root: */
    root: Option<Root>,
}

// Implementation:
impl Parser {
    // Constructor:
    pub fn new(yaml_path: String) -> Self {
        Self {
            /* Path: */
            path: yaml_path,

            /* Contents: */
            contents: None,

            /* Root: */
            root: None,
        }
    }

    // Methods:
    pub fn decode_encoded_yaml(
        encoded_configuration: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Variables (Assignment):
        // Data:
        let data: Vec<u8> = fs::read(encoded_configuration)?;

        // Parts:
        let (file_nonce, cipher_text) = data.split_at(12);

        // Nonce:
        let nonce = Nonce::from_slice(file_nonce);

        // Cipher:
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&AES_ENCRYPTION_KEY));

        // Bytes:
        let plaintext_bytes: Vec<u8> = cipher.decrypt(nonce, cipher_text).map_err(|e| {
            Box::<dyn std::error::Error>::from(format!("AES decryption error: {:?}", e))
        })?;

        // Text:
        let plaintext: String = String::from_utf8(plaintext_bytes)?;

        // Text:
        Ok(plaintext)
    }

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Variables (Assignment):
        // Raw:
        let raw: String = Self::decode_encoded_yaml(&self.path)?;

        // Logic:
        self.contents = Some(raw);

        // Unit:
        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), String> {
        // Variables (Assignment):
        // Contents:
        if let Some(ref contents) = self.contents {
            // Variables (Assignment):
            // Parsed:
            let parsed: Root = serde_yaml::from_str(contents)
                .map_err(|error| format!("[!] YAML parse error: {}", error))?;

            // Root:
            self.root = Some(parsed);

            // Unit:
            Ok(())
        } else {
            Err("[!] No contents loaded!".to_string())
        }
    }

    pub fn image_title(&self) -> Option<String> {
        if let Some(root) = &self.root {
            if let Some(image) = &root.image {
                if let Some(image_title) = &image.title {
                    return Some(image_title.to_string());
                }
            }
        }

        None
    }

    pub fn checks(&self) -> Result<Vec<Check>, String> {
        if let Some(root) = &self.root {
            if let Some(check_wrappers) = &root.checks {
                // Variables (Assignment):
                // Checks:
                let checks: Vec<Check> = check_wrappers
                    .iter()
                    .map(|wrapper| wrapper.check.clone())
                    .collect();

                // Checks:
                Ok(checks)
            } else {
                Ok(vec![])
            }
        } else {
            Err("[!] No contents loaded!".to_string())
        }
    }
}
