# Rust test application
This is the Rust application used in an experiment as a part of graduation. The application itself is a simple HTTP server. 

*setup*
The application some external dependencies. To install those run `make build`. This downloads all dependencies and creates an executable.

*ARMv5*
The experiment itself was designed to be executed on an ARMv5 architecture. Ufortunately crosscompiling to ARMv5 is not something that Rust supports out of the box. Thats why this repository contains a pre-build ARmv5 executable.
