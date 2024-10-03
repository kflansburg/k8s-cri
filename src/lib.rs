//! Automatically generated types, clients, and servers from [Kubernetes CRI Protobuf definitions](https://github.com/kubernetes/cri-api/tree/7d8ade91836419c9dd49d059bda1fe4a7dc283f5/pkg/apis/runtime).
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
//! use hyper_util::rt::TokioIo;
//! use tonic::transport::{Channel, Endpoint, Uri};
//! use tower::service_fn;
//!
//! #[tokio::main]
//! async fn main() {
//!     let channel = Endpoint::try_from("http://[::]")
//!         .unwrap()
//!         .connect_with_connector(service_fn(|_: Uri| async {
//!             let path = "/run/containerd/containerd.sock";
//!             Ok::<_, std::io::Error>(TokioIo::new(UnixStream::connect(path).await?))
//! }))
//!         .await
//!         .expect("Could not create client.");
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
