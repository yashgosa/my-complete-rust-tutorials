Package: Collection of "cartes"; 1: 1 git repository
Crate: binary, libarary, etc
binary: program, app
library: .lib, .dll, .dylib

toml (a markup language) is the manifest, it has the metadata of the package and contains dependencies

.gitingnore: 1: 1 correspondence with git repo
|
|----> All the files inside /target
|
-----> What files to ignore

# Function Modules:

When we create an vinary crate, the program has to call the outermost function (`main()`) in `main.rs`

# Modules:

Modules are a way to group your functions and by default anything inside a module is private

There is a builtin formatter called `rustfmt`

We can nest rust modules and these will be private by default
