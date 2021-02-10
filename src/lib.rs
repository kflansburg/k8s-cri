//! # k8s-cri
//!
//! Automatically generated types, clients, and servers from Kubernetes CRI Protobuf definitions.
//!
//! ## Examples
//!
//! Connecting over TCP:
//!
//! ```no_run
//! use k8s_cri::v1alpha2::runtime_service_client::RuntimeServiceClient;
//! use k8s_cri::v1alpha2::ListContainersRequest;
//! use tokio::main;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = RuntimeServiceClient::connect("http://[::1]:50051")
//!         .await
//!         .expect("Could not create client.");
//!
//!     let request = tonic::Request::new(ListContainersRequest { filter: None });
//!     let response = client
//!         .list_containers(request)
//!         .await
//!         .expect("Request failed.");
//!     println!("{:?}", response);
//! }
//! ```
//!
//! Connecting to a Unix domain socket:
//!
//! ```no_run
//! use std::convert::TryFrom;
//! use tokio::main;
//!
//! use k8s_cri::v1alpha2::runtime_service_client::RuntimeServiceClient;
//! use tokio::net::UnixStream;
//! use tonic::transport::{Channel, Endpoint, Uri};
//! use tower::service_fn;
//!
//! #[tokio::main]
//! async fn main() {
//!     let path = "/run/containerd/containerd.sock";
//!     let channel = Endpoint::try_from("http://[::]")
//!         .unwrap()
//!         .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect(path)))
//!         .await
//!         .expect("Could not create client.");
//!
//!     let mut client = RuntimeServiceClient::new(channel);
//! }
//! ```

pub mod v1alpha2 {
    tonic::include_proto!("runtime.v1alpha2");
}
