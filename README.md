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

* Free Space on Disk: 55 MB
* Rust 
``` 
curl https://sh.rustup.rs -sSf | sh
```

Cargo requires brew:
```
xcode-select --install
ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
brew doctor
brew install caskroom/cask/brew-cask
```
* Cargo
```
brew cask install cargo
```




Compile using:
```
cargo run
```

You can verify the hash of the main.rs file under 'src' folder,
the generated hash of the file as of June 11, 2019 2:57 PM [IST] is below:

6b4b5a4b87620dd5fe7d7664d2f4dd5188dfdc1869f14c59a688da0cc046bdfb967adb42a14e51cac67a7b87d29fe1d19cc678697d2bdcf35be2e2f6ca96beab

this HASH digest is also available in the "MAIN_HASH.txt" file in this same directory, and can be used to verify the main.rs