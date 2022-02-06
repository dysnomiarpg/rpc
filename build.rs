fn main() {
    tonic_build::compile_protos("protocol/dysnomia.proto").unwrap();
}
