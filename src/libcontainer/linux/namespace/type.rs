#[derive(Debug, PartialEq)]
pub enum NamespaceType {
    PID,
    UTS,
    IPC,
    USER,
    MOUNT,
    CGROUP,
    NETWORK,
}
