#!/bin/bash

set -e

PROJECT_NAME="System Cooler Daemon"
DAEMON_NAME=systemcoolerd
CONF_ROOT=systemcooler
CONF_DIR=daemon
CONF_NAME=conf.ini
TARGET_DIR=target
VERSION=1.0.0
ARCH=armhf
DEB_DIR=$TARGET_DIR/DEBIAN
PACKAGE_NAME=$DAEMON_NAME_$VERSION_$ARCH.deb

# Check if the directory exists
if [ ! -d "$TARGET_DIR" ]; then
  echo "[ ERROR ] Failed to build $PROJECT_NAME Deb (No such build)."
  exit 1
fi

echo "[ INFO ] Creating $PROJECT_NAME Deb package structure..."
mkdir -p $TARGET_DIR/etc/$CONF_ROOT/$CONF_DIR
mkdir -p $TARGET_DIR/usr/bin
mkdir -p $TARGET_DIR/usr/lib/systemd/system
mkdir -p $TARGET_DIR/tmp/$CONF_ROOT/$CONF_DIR

echo "[ INFO ] Copying $PROJECT_NAME files to Deb package structure ..."
cp $TARGET_DIR/debug/$DAEMON_NAME $TARGET_DIR/usr/bin/systemcoolerd
cp conf/$CONF_NAME $TARGET_DIR/etc/$CONF_ROOT/$CONF_DIR
cp conf/$DAEMON_NAME.service $TARGET_DIR/usr/lib/systemd/system
cp pack/debian/control $DEB_DIR/
cp pack/debian/postinst $DEB_DIR/

echo "[ INFO ] Setting $PROJECT_NAME files permissions ..."
chmod 755 $DEB_DIR/postinst

echo "[ INFO ] Building $PROJECT_NAME Debian package ..."
dpkg-deb --build $TARGET_DIR $PACKAGE_NAME

echo "[ INFO ] Package of $PROJECT_NAME created: $PACKAGE_NAME"