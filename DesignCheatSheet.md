# Design Cheat Sheet

### Good Names

Classes, functions, variables, etc. should have descriptive and sensible names.  If you're struggling to find one, might be a sign of problems..

### No Duplication

Repeated function, sections of code, even values should be removed, often by abstracting the duplicated part into its own function.

### Single Responsibilities

Classes, functions, etc. should have a single thing they do.

### Limit Coupling

A class or function should depend on as few external classes & functions as possible.  It's often better to have a dependency on an interface then a concrete class.

### High Cohesion

Functionality strongly related to the same thing should usually be kept together.
