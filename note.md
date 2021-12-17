* Cargo.toml 
```
[lib]
crate-type=["cdylib"]
```

* Cargo.toml
```
#Enable ndk logger
features=["logger"]
ndk-glue={ git="https://github.com/pigfall/android-ndk-rs", tag= "ndk-glue-0.5.0" , features=["logger"]}
```
