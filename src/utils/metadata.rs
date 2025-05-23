use metadata::{Metadata, NAME, VERSION};

pub fn title(metadata: &Metadata) -> String {
    let name = metadata.get(NAME).map_or("", |name| name);
    match metadata.get(VERSION) {
        Some(version) => format!("{name} {version}"),
        None => name.to_owned(),
    }
}
