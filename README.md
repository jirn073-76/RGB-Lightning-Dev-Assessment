# RGB-Lightning-Dev-Assessment
An open source implementation of Bitfinex's RGB Lightning Dev Assessment

A minimal Rust project that uses bdk to create a Wallet struct that implements methods to send bitcoin. This Wallet struct can operate in two modes: full operation (with access to private keys) and watch-only operation (without access to private keys).

In watch-only mode, the Wallet delegates signing to the user (e.g. on an air-gapped machine) and makes the transaction exchange (for signing) as interoperable as possible with other wallets.
To make the transaction exchange (for signing) as interoperable as possible with other wallets, the Wallet struct uses standard Bitcoin script and transaction formats, as defined in the bdk library. This ensures that the Wallet can communicate with other wallets using the same formats, regardless of the underlying implementation details.

To compile and run this program, you would first need to install the Rust programming language and the bdk library. 
You can do this by running the following commands:

`curl https://sh.rustup.rs -sSf | sh`\
`cargo install bdk`

Once you have installed Rust and bdk, you can compile and run the program using the following commands:
 
`rustc wallet.rs` \
`./wallet`
