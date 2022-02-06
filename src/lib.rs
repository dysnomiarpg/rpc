pub mod services {
	//! Provides Rust bindings for the gRPC "services" package.
	tonic::include_proto!("dysnomia.services");
}
