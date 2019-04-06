#[derive(Debug)]
pub enum NamespaceType {
    PID,
    UTS,
    IPC,
    USER,
    MOUNT,
    CGROUP,
    NETWORK,
}
