# CR7

CR7 is a simple, (eventually) OCI compliant container runtime.

This project is under development and is not usable yet. Make sure you follow the updates. Contributions are welcome!

## Development

Currently, CR7 works only on Linux. To help those who are using other systems this repository comes with a Vagrantfile. To use it, make sure you have [Vagrant](https://www.vagrantup.com/) installed.

To create the VM, just run:

```
vagrant up
```
This command will mount the repository dir at `/opt/cr7`. After the VM setup, just log into it:

```
vagrant ssh
```

After that, install Rust and Cargo inside the virtual machine. Once the setup is ready, you can run the tests and compile the project using the `cargo` command.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
