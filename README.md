# unifi-block

Simple command line utility to block/unblock Wifi clients by MAC
address on Ubiquiti Unifi access points.

# Usage

    USAGE:
        unifi-block [OPTIONS] --command <command> --config <config_file_path>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -x, --command <command>            Unifi command to execute [possible values: Block, Unblock]
        -c, --config <config_file_path>    Path to YAML config file
        -p, --password <password>          Unifi password to login with
        -u, --user <user>                  Unifi user to login with

# Configuration File Format

A simple .yaml file:

    base_url: https://unifi.home:8443
    site: default
    accept_invalid_certs: true | false
    client_macs:
      - f3:d2:c7:61:ad:64
      - 4e:b2:43:06:9b:86
      - 8b:ae:e6:41:43:a5

# Further Resources

* [Unifi Shell API](https://dl.ubnt.com/unifi/5.6.42/unifi_sh_api)
* [Reverse Engineered REST APIs](https://ubntwiki.com/products/software/unifi-controller/api)
* [Another's Adventure](https://bartsimons.me/playing-around-with-the-ubiquiti-unifi-controller)
