//pub mod error;
//pub use self::error::Error;

#[cfg(target_os = "linux")] pub mod linux;
#[cfg(target_os = "linux")] pub use self::linux::container::Container;
#[cfg(target_os = "linux")] pub use self::linux::namespace::Namespace;
#[cfg(target_os = "linux")] pub use self::linux::namespace::NamespaceList;
#[cfg(target_os = "linux")] pub use self::linux::namespace::NamespaceType;
#[cfg(target_os = "linux")] pub use self::linux::mount::MountPoint;
#[cfg(target_os = "linux")] pub use self::linux::rlimit::Rlimit;
