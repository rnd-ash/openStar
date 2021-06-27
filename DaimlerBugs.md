# A list of bugs found within Xentry and DAS that OpenStar attempts to fix

If you find any more bugs with Daimler's own software, feel free to add them here!

## General
* Passthru and OpenShell versions of the software suite come as 2 different 50GB installations, when all that really differs is the core caesar module library.
* Passthru errors are NOT displayed to the end user, instead causing a hard-crash of both DAS or Xentry. This goes against the Passthru APIs recommendation of logging or displaying the error in the event of `ERR_FAILED` using the `PassthruGetLastError()` function.

## Xentry
* Xentry attempts to scan every possible ECU variant in a vehicle, causing the initial setup scan of ECUs to take upwards of 10 minutes! (Compared to DAS where this takes only a few seconds)
* Xentry cannot deal with the Passthru API's error for too many communication channel creations, and instead just tries to continue adding more channels in an infinite loop.

## DAS
* Ignition tick box on Passthru devices requires an external tool to manually set (IgnitionEnabler)
* Resizing DAS's window causes the content of DAS to be placed in a scroll box rather than dynamically resize
* DAS can crash if Ignition status is modified during quick test whilst testing powertrain ECUs
