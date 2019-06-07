# BLAME-er x.0

Description: A **CLI ENCRYPTION Application** uses Honey Encryption Method to encrypt message.

A Command Line Application, written in Rust.
Digests the required string, text file, object with HASH and can verify a HASH with another string, text file, object.
Currently Implementing Honey Encryption Method.

Currently Capable of HASHing files and string, verification function will be added very soon.


 * Developers:
> Rajdeep Bandopadhyay - bandoprp@mail.uc.edu


 * Special Instructions:
> No special compilations instructions.



Prerequisites:

* Rust 
``` 
curl https://sh.rustup.rs -sSf | sh
```

* Cargo
```
brew cask install cargo
```
Cargo requires brew:
```
xcode-select --install
ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
brew doctor
brew install caskroom/cask/brew-cask
```

* Compile using:
```
cargo run
```
