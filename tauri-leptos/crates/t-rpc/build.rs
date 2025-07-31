use tonic_build::Config;

fn main() -> std::io::Result<()> {
    let header = &["./protos"];
    let protos = &["./protos/internal.proto"];

    let mut config = Config::new();
    config.prost_path("crate::prost");
    config.include_file("protos.rs");

    if cfg!(feature = "rkyv-codec") {
        let rkyv_attr = "#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]";
        config.type_attribute(".", rkyv_attr);
    }

    let codec_path = if cfg!(feature = "rkyv-codec") {
        "crate::codec::rkyv::Codec"
    } else {
        "tonic::codec::ProstCodec"
    };

    let builder = tonic_build::configure().codec_path(codec_path);
    builder.compile_protos_with_config(config, protos, header)
}
