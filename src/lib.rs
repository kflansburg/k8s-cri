//! Automatically generated types, clients, and servers from [Kubernetes CRI Protobuf definitions](https://github.com/kubernetes/cri-api/tree/7d8ade91836419c9dd49d059bda1fe4a7dc283f5/pkg/apis/runtime).
//!
//! ## Examples
//!
//! Connecting over TCP:
//!
//! ```no_run
//! # extern crate tokio;
//! use k8s_cri::v1alpha2::runtime_service_client::RuntimeServiceClient;
//! use k8s_cri::v1alpha2::ListContainersRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = RuntimeServiceClient::connect("lttp://[::]:50051").await.expect("Could not create client.");
//!
//!     let request = tonic::Request::new(ListContainersRequest {
//!         filter: None
//!     });
//!     let response = client.list_containers(request).await.expect("Request failed.");
//!     println!("{:?}", response);
//! }
//! ```
//!
//! Connecting to a unix socket:
//!
//! ```no_run
//! # extern crate tokio;
//! use k8s_cri::v1alpha2::runtime_service_client::RuntimeServiceClient;
//! use tokio::net::UnixStream;
//! use tonic::transport::{Endpoint, Uri, Channel};
//! use tower::service_fn;
//! use std::convert::TryFrom;
//!
//! #[tokio::main]
//! async fn main() {
//!     let path = "/run/containerd/containerd.sock";
//!     let channel = Endpoint::try_from("lttp://[::]:50051").expect("Could not create endpoint.")
//!         .connect_with_connector(service_fn(move |_: Uri| {
//!             UnixStream::connect(path)
//!         }))
//!         .await.expect("Could not create client.");
//!
//!     let mut client = RuntimeServiceClient::new(channel);
//! }
//! ```

pub mod v1 {
    //! API version v1, [original Protocol Buffers file](https://github.com/kubernetes/cri-api/tree/7d8ade91836419c9dd49d059bda1fe4a7dc283f5/pkg/apis/runtime/v1/api.proto).
    tonic::include_proto!("runtime.v1");
}

pub mod v1alpha2 {
    //! API version v1alpha2, [original Protocol Buffers file](https://github.com/kubernetes/cri-api/tree/7d8ade91836419c9dd49d059bda1fe4a7dc283f5/pkg/apis/runtime/v1alpha2/api.proto).
    tonic::include_proto!("runtime.v1alpha2");
}
