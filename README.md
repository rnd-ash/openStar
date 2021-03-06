# OPENSTAR

An opensource diagnostic application for Daimler vehicles inspired by DAS and Xentry. Some of the work here is based on [OpenVehicleDiag](http://github.com/rnd-ash/openvehiclediag/)

**If you decide to use this software on your own vehicle, there is NO liability if something goes wrong! This software is not affiliated with Daimler AG in any way, shape or form. You will still need an existing Xentry installation to utilize this software, as files from the diagnostic suite WILL NOT be redistributed.**

Since Daimler have abandoned DAS, and appear to be gluing functionality to Xentry with tape, This program is an attempt at making a better alternative to both applications, which have loads of bugs and are overly bloated. For a full list of bugs and issues with Daimler's own software, see [DaimlerBugs.md](DaimlerBugs.md)

## Project goals
* Merge functionalities of DAS and Xentry into one program
* Cross platform support (Including use of SocketCAN on Linux)
* Modular structure with cbindgen creating C++ Headers for certain modules such as file loading or vehicle communication
* Fix multiple bugs in DAS or Caesar which Daimler doesn't seem to want to fix

## Not project goals
* Online Xentry functionality (Example: SCN Coding)
* Firmware flashing (For now)

## Using OpenStar

**OpenStar is currently nonfunctional**

Firstly, you will need an existing Xentry installation. Either Passthru or OpenShell editions will work fine. Upon launching OpenStar, you will be prompted to select the installation directory of your existing Mercedes-Benz folder.

## Submitting an issue
Issues should be submitted via the issues tab. IMPORTANT: If referencing a file from DAS/Xentry, **DO NOT** include the file as an attachment. Instead, just note the file path. If a file from DAS/Xentry is included then the issue will be removed without notice.

## Repository structure
* hardware - Hardware library for various adapters to allow communication with vehicle ECUs
* simloader - Loader and executor for Daimler's SIM files (ECU simulation)
* open_star - OpenStar diagnostic application
* filehandler - Handler API for files used by the software such as CBF,SMRD
* diagnostics - Library for implementation of KWP2000 and UDS

