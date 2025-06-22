Vagrant.configure("2") do |config|
    config.vm.define :firewall do |firewall|
        firewall.vm.box = "alvistack/ubuntu-24.04"
        firewall.vm.network :private_network,
            :type => "dhcp",
            :libvirt__network_name => "lan",
            :libvirt__network_address => '10.20.30.0',
            :libvirt__netmask => '255.255.255.0'

        firewall.vm.provider :libvirt do |libvirt|
            libvirt.memory = 1024
            libvirt.cpus = 2
        end
    end
  end
