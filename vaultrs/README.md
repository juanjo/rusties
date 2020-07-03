## How to test the service

0. Install Vault
````
brew install vault
```

1. Start Vault in Development mode:
```
vault server -dev
```
2. Record the Root Token printed at startup time, and update _const TOKEN_ in main.rs
_TODO: Move these values to dotenv and read it from there_

3. Enable the Transit secrets engine:
_This step took me like one hour to figure it out. Vault was installed, all running, but the tests on my code were not passing. I changed everything, test a million other things... until I realized the Transit Engine was not running. Silly me, but you grasshopper... mind the gap._
```
vault secrets enable transit
```

4. Run the tests
```
cargo test
```


