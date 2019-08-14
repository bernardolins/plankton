# Plankton

Plankton is a simple CLI tool to create containers according to the OCI spec.

**This project is under development and is not usable yet. Make sure you follow the updates. Contributions are welcome!**

## Setting up locally

Make sure you have [rust 1.34+ and cargo installed]([[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)](https://github.com/rust-lang/rust/releases)).

Clone the repository and go to the  project directory:

```
$ git clone https://github.com/bernardolins/plankton.git
$ cd plankton
```

Use `cargo` to run the tests and build the project:

```
$ cargo test
$ cargo build
```

## Running

Plankton does not have a release yet. Make sure you follow the steps [bellow](#setting-up-locally) before trying to run the project.

You will need an OCI bundle to run plankton. For now, you can create the bundle just like [runc does]([https://github.com/opencontainers/runc/#creating-an-oci-bundle](https://github.com/opencontainers/runc/#creating-an-oci-bundle)).

From the project directory, create a container using the bundle:

```
  # target/debug/plankton run mycontainer -b /path/to/bundle
```

You can check the container state:

```
# target/debug/plankton query mycontainer
{
  "bundle": "/opt/plankton",
  "id": "mycontainer",
  "pid": 21496,
  "status": "stopped"
}
```
