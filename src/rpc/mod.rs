//! The Module contains the implementation of json rpc
//! based file system api
//! the interface include create, read, write, remove
//! calls.
//! The implementation contains the buffer space which helps for
//! fast read write implementation.
//!
//!
// use jsonrpc_core::Result as JResult;
// use jsonrpc_derive::rpc;

// #[rpc]
// pub trait FileStore {
//     #[rpc(name = "write")]
//     fn write(&self, id: u32, pos: u32, data: Vec<u32>) -> JResult<u32>;

//     #[rpc(name = "read")]
//     fn read(&self, id: u32, pos: u32, length: u32) -> JResult<Vec<u8>>;

//     #[rpc(name = "remove")]
//     fn remove(&self, id: u32) -> JResult<bool>;

//     #[rpc(name = "create")]
//     fn create(&self, id: u32, name: String) -> JResult<bool>;
// }
