use crate::error::{PvkrError, Result};

use ring::digest::{Context, SHA256};
use std::io::{Read, Write};

#[derive(Eq, PartialEq)]
pub struct Sha256 {
    inner: [u8; 32],
}

impl Sha256 {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0; 32];
        match reader.read_exact(&mut buf) {
            Ok(_) => Ok(Sha256 { inner: buf }),
            Err(e) => PvkrError::invalid_pvkr_package(format!(
                "Failed to read sha256, probably invalid package.pvkr: {}",
                e
            )),
        }
    }

    pub fn maybe_read_from<R: Read>(reader: &mut R) -> Result<Option<Self>> {
        let mut buf = [0; 32];
        match reader.read(&mut buf) {
            Ok(0) => Ok(None),
            Ok(32) => Ok(Some(Sha256 { inner: buf })),
            Ok(_) => Err(PvkrError::InvalidPvkrPackage(
                "Invalid SHA256 length".into(),
            )),
            Err(e) => Err(e.into()),
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.inner)?;
        Ok(())
    }

    pub fn from_file<P: AsRef<std::path::Path>>(p: P) -> Result<Self> {
        let path = p.as_ref();
        if !path.exists() || !path.is_file() {
            return PvkrError::file_not_found(format!("{:?}", path));
        }
        let f = std::fs::File::open(path)?;
        let mut reader = std::io::BufReader::new(f);
        let mut sha256_builder = Sha256Builder::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer).expect("read error");
            if count == 0 {
                break;
            }
            sha256_builder.update(&buffer[..count]);
        }
        Ok(sha256_builder.finish())
    }
}

impl std::fmt::Display for Sha256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hexstr: String = self
            .inner
            .into_iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{}", hexstr)
    }
}

pub struct Sha256Builder {
    context: Context,
}

impl Sha256Builder {
    pub fn new() -> Self {
        Sha256Builder {
            context: Context::new(&SHA256),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.context.update(data);
    }

    pub fn finish(self) -> Sha256 {
        let digest = self.context.finish();
        let mut buf: [u8; 32] = [0; 32];
        buf.copy_from_slice(digest.as_ref());
        Sha256 { inner: buf }
    }
}
