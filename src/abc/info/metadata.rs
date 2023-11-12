use crate::{error::Result, StreamReader, StreamWriter};

#[derive(PartialEq, Debug)]
pub struct MetadataItem {
    pub key: u32,
    pub value: u32,
}

#[derive(PartialEq, Debug)]
pub struct Metadata {
    pub name: u32,
    pub items: Vec<MetadataItem>,
}

impl MetadataItem {
    pub fn keyless(&self) -> bool {
        self.key == 0
    }
}

impl Metadata {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let name = stream.read_u30()?;
        let count = stream.read_u30()?;
        let mut items = Vec::with_capacity(count as usize);

        for _ in 0..count {
            items.push(MetadataItem {
                key: stream.read_u30()?,
                value: 0,
            });
        }
        for item in &mut items {
            item.value = stream.read_u30()?;
        }

        Ok(Self { name, items })
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.name)?;
        stream.write_u30(self.items.len() as u32)?;

        for item in &self.items {
            stream.write_u30(item.key)?;
            stream.write_u30(item.value)?;
        }
        Ok(())
    }
}
