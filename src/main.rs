use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    #[serde(rename = "CFBundleInfoDictionaryVersion")]
    cfbundle_info_dictionary_version: String,

    #[serde(rename = "band-size")]
    band_size: usize,

    #[serde(rename = "bundle-backingstore-version")]
    bundle_backingstore_version: usize,

    #[serde(rename = "diskimage-bundle-type")]
    diskimage_bundle_type: String,

    size: usize,
}

fn main() {
    let metadata: Metadata = plist::from_file("data/none-fat.sparsebundle/Info.plist").unwrap();
    println!("metadata: {:?}", metadata);
}
