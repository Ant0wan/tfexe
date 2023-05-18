<br />
<p align="center">
  <a href="">
    <img src=".logo.png" alt="Logo" height="80">
  </a>

  <p align="center"><i>Rust-powered wrapper for seamless execution of tfswitch and Terraform with version control.</i>
  </p>
</p>

---

## Table of Contents
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [License](#license)

## Prerequisites
Before installing and using tfexe, you will need to have the following tools installed on your system:
- `wget`: A command-line utility used for downloading files.
- `tfswitch`: A tool used for switching between different versions of TensorFlow.

## Installation
To install tfexe, simply run the `install.sh` script provided in this repository. The script will automatically download and install the necessary dependencies, and set up tfexe on your system.

```shell
wget -q -O -  https://raw.githubusercontent.com/Ant0wan/tfexe/master/install.sh | sh
```

## Usage

1. Install ***tfexe*** by following the installation instructions in the tfexe GitHub repository.
2. Create a Terraform configuration file (e.g., main.tf, providers.tf...) with the following content:
```hcl
terraform {
  required_version = "~> 1.3.0"
}

# Define your Terraform resources and providers here
```
3. In the command line instead of using the classic Terraform binary, use **tfexe** to execute Terraform commands. **tfexe** will automatically detect the required version of Terraform specified in your Terraform configuration file (required_version in this case, which is set to "~> 1.3.0") and download and execute the appropriate Terraform binary for you. For example:
```shell
# Run Terraform init with tfexe
$ tfexe init
Reading required version from terraform file
Reading required version from constraint: ~> 1.3.0
Matched version: 1.3.9
Installing terraform at /home/antoine/bin
Switched terraform to version "1.3.9"

Initializing the backend...

Initializing provider plugins...

Terraform has been successfully initialized!

You may now begin working with Terraform. Try running "terraform plan" to see
any changes that are required for your infrastructure. All Terraform commands
should now work.

If you ever set or change modules or backend configuration for Terraform,
rerun this command to reinitialize your working directory. If you forget, other
commands will detect it and remind you to do so if necessary.

```

Note: tfexe will automatically execute tfswitch internally to switch to the specified Terraform version before running the Terraform command, based on the content of the providers.tf file in your Terraform configuration directory.

If no version of Terraform is specified in tf files, it will display the same prompt as tfswitch:
```shell
$ tfexe
Use the arrow keys to navigate: ↓ ↑ → ←
? Select Terraform version:
    1.4.5 *recent
    1.3.9 *recent
    1.2.9 *recent
  ▸ 1.4.4
↓   1.4.3
```

Note: You can set an alias in your shell, such as Bash or Zsh, to use **tfexe** as a replacement for the **terraform** command. This can be done by adding the line to your shell profile file (e.g., *.bashrc*, *.bash_profile*, or *.zshrc*). Once the alias is set, you can use terraform in any script or wrapper call, and it will automatically execute **tfexe** with the proper Terraform version based on your configuration.
```shell
alias terraform='tfexe'
```

That's it! You can now use **tfexe** as a drop-in replacement for the classic Terraform binary to execute Terraform commands, and it will automatically manage the versioning for you based on the required_version specified in your Terraform configuration file and the content of your providers.tf file.

## How it works

```mermaid
flowchart LR
    A(tfexe) -->|read .tf| S(tfswitch)
    S -->|select version| T(terraform)
```

## To be done

- [ ] unit tests
- [x] rust complete CI - lint, test, builds, release
- [x] cross-platform build
- [ ] install script similar to tfswitch
- [ ] documentation update
- [ ] packaging rpm and deb

## License

This repository is protected by the GPL3 (GNU General Public License v3.0). You can find the full text of the license in the LICENSE file. Please review and comply with the terms and conditions of the GPL3 license before using or contributing to this project.

For any questions, bug reports, or contributions, please feel free to open an issue or submit a pull request. Thank you for using tfexe!

