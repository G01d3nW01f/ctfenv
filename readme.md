build to same environment as ctf's target machine with docker 

#  Build
```
cargo build --release
```

#  Build Ubuntu16.04(default)
```
./target/release/ctfenv
```

#  Specify Version
```
./ctfenv ubuntu:20.04
```

# BuildEssential Tools automatically install
```
./ctfenv --install-tools
```
or
```
./ctfenv --install-tools ubuntu:18.04
```
