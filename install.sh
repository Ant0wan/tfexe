#!/bin/bash
set -o errexit
get_machine_arch () {
	machine_arch=""
	case $(uname -m) in
		x86_64)   machine_arch="x86_64" ;;
		arm64)    machine_arch="aarch64" ;;
		aarch64)  machine_arch="aarch64" ;;
	esac
	echo $machine_arch
}
arch=$(get_machine_arch)
case "$(uname -s)" in
	Darwin*)
		bin="${arch}-apple-darwin"
		;;
	MINGW64*)
		bin="${arch}-pc-windows-gnu"
		;;
	MSYS_NT*)
		bin="${arch}-pc-windows-gnu"
		;;
	*)
		bin="${arch}-unknown-linux-musl"
		;;
esac
echo "bin=tfexe_$bin"
download_path=$(mktemp -d -t tfexe.XXXXXXXXXX)
download_executable="${download_path}/tfexe"

echo "Downloading tfexe latest version"
wget --quiet "https://github.com/Ant0wan/tfexe/releases/latest/download/tfexe_${bin}" -O "${download_executable}"
echo "Downloaded successfully"

if [ "$bin" = "${arch}-pc-windows-gnu" ]; then
	dest="${TFEXE_INSTALL_PATH:-/bin}/"
	echo "Installing tfexe to ${dest} ..."
	mv "${download_executable}" "$dest"
	retVal=$?
	if [ $retVal -ne 0 ]; then
		echo "Failed to install tfexe"
		exit $retVal
	else
		echo "tfexe installed at ${dest} successfully"
	fi
else
	dest="${TFEXE_INSTALL_PATH:-/usr/local/bin}/"
	echo "Installing tfexe to ${dest} ..."
	if [ -w "$dest" ]; then SUDO=""; else
		SUDO="sudo";
	fi
	$SUDO mkdir -p "$dest"
	$SUDO install -c -v "${download_executable}" "$dest"
	retVal=$?
	if [ $retVal -ne 0 ]; then
		echo "Failed to install tfexe"
		exit $retVal
	fi
fi
echo "Cleaning temporary downloaded files directory ${download_path} ..."
rm -rf "${download_path}"
echo "Current tfexe version"
#"${dest}/tfexe" -v
