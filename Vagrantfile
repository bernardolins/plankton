Vagrant.configure("2") do |config|
  config.vm.box = "fedora/29-cloud-base"
  config.vm.network "private_network", type: "dhcp"
  config.vm.synced_folder "./", "/opt/plankton", type: "nfs"
end
