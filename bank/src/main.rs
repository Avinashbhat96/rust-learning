// The Account struct represents a single bank account, with fields for the account balance, a unique identifier, 
// and the name of the account holder.
#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,   
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) ->Self {
        // id and holder are identical to the account struct, so dont need to mention them while assigning values to them.
        Account {
            balance: 0,
            id,
            holder,
        }
    }
}

// The Bank struct contains a vector of Account structs, representing the various accounts held at the bank.
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    // The new method initializes a new Bank instance with an empty vector of accounts.
    fn new() -> Self {
        Bank {
            accounts: Vec::new(),
        }
    }
}

fn print_account(account: Account) {
    // If you print only one variable, then it shows unused code warning
    println!("Account ID: {}, Holder: {}, Balance: {}", account.id, account.holder, account.balance);
}

fn print_account_by_ref(account: &Account) {
    // If you print only one variable, then it shows unused code warning
    println!("Account ID: {}, Holder: {}, Balance: {}", account.id, account.holder, account.balance);
}

fn main() {
    let mut bank = Bank::new();
    
    let mut account = Account::new(1, String::from("Alice"));
    
    // References - instead of moving the variable we can use the references
    print_account_by_ref(&account);

    print_account(account);
    // print_account(account); // This line would cause a compile-time error because `account` has been moved to the function above.
    
}
