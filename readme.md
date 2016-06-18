# kwon-vm
a virtual machine implemented in rust.

## Preface

This is a hobby project, don't expect much from it at this point. Further, this virtual machine is language-agnostic and will only concern itself with its own intermediate representation. To make a concrete programming language run on this virtual machine, a compiler from that language to its intermediate representation must be implemented.

The debugger will implement a binary protocol and expose itself on a TCP port if the virtual machine runs in debug mode.

## Implemented features

* None

## Planned features

* Intermediate Representation of code (IR)
* Interpreter
* JIT compiler (most likely a trace JIT)
* Connectable Debugger, debugging protocol
