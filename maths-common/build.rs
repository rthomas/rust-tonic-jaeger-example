fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = vec!["proto/maths.proto", "proto/add.proto", "proto/mul.proto"];

    for proto in protos {
        println!("cargo:rerun-if-changed={}", proto);
        tonic_build::compile_protos(proto)?;
    }
    Ok(())
}
