#!/bin/sh
set -e

# Steal from llvm-sys build script!
curl -L https://bitbucket.org/tari/llvm-sys.rs/downloads/llvm-3.9.1.linux.tar.xz | tar -C $HOME -xJ

#VERSION="3.9.1"
#URL=
# Download llvm.tar.xz
#wget http://releases.llvm.org/$VERSION/clang+llvm-$VERSION-x86_64-linux-gnu-ubuntu-16.04.tar.xz -O llvm.tar.xz
# Extract files (no `v` arg to reduce web client spam)
#tar xfJ llvm.tar.xz
# Move llvm files over
#mv -v clang+llvm-$VERSION-x86_64-linux-gnu-ubuntu-16.04 $HOME/llvm-install
# Copy binaries over
#cp -v $HOME/llvm-install/bin/* $HOME/bin
