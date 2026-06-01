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

    // here actually doesnt matter if we take a ref or a value as it is
    // because rust does not move this values, rather just copies
    // even without ref, because its a number
    fn deposit(&mut self, amount: i32) -> i32{
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32{
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
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

    fn add_account(&mut self, account: Account){
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(| account | account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts.iter().map(| account | account.summary()).collect::<Vec<String>>()
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

fn print_num_accounts(bank: &Bank){
    println!("Num accounts: {}", bank.accounts.len());
}

fn main() {
    let mut bank = Bank::new();
    
    let mut account = Account::new(1, String::from("Alice"));

    account.deposit(500);
    account.withdraw(250);

    // println!("{}", account.summary());
    
    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{:#?}", bank.total_balance());


    // bank.accounts.push(account);

    // print_num_accounts(&bank);

    // References - instead of moving the variable we can use the references
    // this is Read only - similar to a const ref
    // print_account_by_ref(&account);

    // print_account(account);
    // print_account(account); // This line would cause a compile-time error because `account` has been moved to the function above.
    
    // let num = 5;
    // let anouther_num = num;
    // println("{}", num); // no error, this is a copy-able value

}
