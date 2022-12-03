extern crate bdk;

use bdk::{
    script::Script,
    transaction::{
        Transaction,
        TxIn,
        TxOut,
        SigHashType,
        OutPoint,
        TxOutBinding
    }
};

struct Wallet {
    // The script that represents the public key or address of the wallet.
    pubscript: Script,
    // The private key of the wallet, if in full operation mode.
    private_key: Option<Vec<u8>>,
}

impl Wallet {
    // Constructs a new Wallet in full operation mode.
    fn new(private_key: Vec<u8>, pubscript: Script) -> Self {
        Self {
            private_key: Some(private_key),
            pubscript,
        }
    }

    // Constructs a new Wallet in watch-only mode.
    fn new_watch_only(pubscript: Script) -> Self {
        Self {
            private_key: None,
            pubscript,
        }
    }

    // Sends the specified amount of bitcoin from this wallet to the specified
    // address. In full operation mode, the private key is used to sign the
    // transaction. In watch-only mode, the transaction is returned for the user
    // to sign on an air-gapped machine.
    fn send(&self, amount: u64, address: &str) -> Result<Transaction, String> {
        // Create the transaction output that sends the specified amount of
        // bitcoin to the specified address.
        let tx_out = TxOut {
            value: amount,
            script_pubkey: Script::from(address),
        };

        // Create the transaction input that spends the UTXO associated with
        // this wallet.
        let tx_in = TxIn {
            previous_output: OutPoint {
                txid: [0; 32],
                vout: 0,
            },
            sequence: 0,
            script_sig: self.pubscript.clone(),
        };

        let mut tx = Transaction {
            version: 2,
            inputs: vec![tx_in],
            outputs: vec![tx_out],
            lock_time: 0,
        };

        if let Some(private_key) = &self.private_key {
            // We are in full operation mode, so we can sign the transaction
            // using the private key.
            tx.sign(&private_key, &[], SigHashType::All);
            Ok(tx)
        } else {
            // We are in watch-only mode, so we need to return the transaction
            // for the user to sign on an air-gapped machine.
            Ok(tx)
        }
    }

    // Signs a transaction with the private key of this wallet. This method is
    // only available in full operation
      
    fn sign_transaction(&self, tx: &Transaction) -> Result<(), String> {
        if let Some(private_key) = &self.private_key {
            // We are in full operation mode, so we can sign the transaction
            // using the private key.
            tx.sign(&private_key, &[], SigHashType::All);
            Ok(())
        } else {
            // We are in watch-only mode, so we cannot sign the transaction.
            Err(String::from("Cannot sign transaction in watch-only mode"))
        }
    }   
        
    // Creates a Wallet in watch-only mode and sign a transaction using the sign_transaction method:
    fn main() {
        // Create a new Wallet in watch-only mode.
        let pubscript = Script::from("address");
        let mut wallet = Wallet::new_watch_only(pubscript);

        // Send some bitcoin from the wallet.
        let tx = wallet.send(1000, "other_address").unwrap();

        // Sign the transaction on an air-gapped machine.
        let private_key = vec![0x01, 0x02, 0x03];
        wallet.sign_transaction(&tx).unwrap();

        // Print the signed transaction.
        println!("{:?}", tx);
    }
}
