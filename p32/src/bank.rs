use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub name: String,
    pub credit_line: u64,
    pub balance: i64, // positive = debit, negative = credit
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bank {
    pub name: String,
    pub users: Vec<User>,
    pub credit_interest: u64, // in basis points (0.01%)
    pub debit_interest: u64,  // in basis points (0.01%)
}

#[derive(Debug)]
pub enum TransferError {
    UserNotFound(String),
    InsufficientFunds(String),
    CreditLimitExceeded(String),
}

impl fmt::Display for TransferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransferError::UserNotFound(name) => write!(f, "User {} not found", name),
            TransferError::InsufficientFunds(name) => {
                write!(f, "User {} has insufficient funds", name)
            }
            TransferError::CreditLimitExceeded(name) => {
                write!(f, "User {} would exceed credit line", name)
            }
        }
    }
}

impl Error for TransferError {}

impl User {
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self {
            name,
            credit_line,
            balance,
        }
    }
}

impl Bank {
    pub fn new(name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self {
            name,
            users: Vec::new(),
            credit_interest,
            debit_interest,
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn calc_balance(&self) -> (u64, u64) {
        let mut liabilities = 0;
        let mut assets = 0;

        for user in &self.users {
            if user.balance > 0 {
                liabilities += user.balance as u64;
            } else {
                assets += (-user.balance) as u64;
            }
        }

        (liabilities, assets)
    }

    pub fn transfer_funds(
        &mut self,
        from_name: &str,
        to_name: &str,
        amount: u64,
    ) -> Result<(), TransferError> {
        let amount_i64 = amount as i64;

        // Find user indices first
        let from_idx = self
            .users
            .iter()
            .position(|u| u.name == from_name)
            .ok_or(TransferError::UserNotFound(from_name.to_string()))?;

        let to_idx = self
            .users
            .iter()
            .position(|u| u.name == to_name)
            .ok_or(TransferError::UserNotFound(to_name.to_string()))?;

        // Check if transfer is possible
        if self.users[from_idx].balance < amount_i64 {
            return Err(TransferError::InsufficientFunds(from_name.to_string()));
        }

        if (self.users[from_idx].balance - amount_i64).unsigned_abs()
            > self.users[from_idx].credit_line
        {
            return Err(TransferError::CreditLimitExceeded(from_name.to_string()));
        }

        if (self.users[to_idx].balance + amount_i64).unsigned_abs() > self.users[to_idx].credit_line
        {
            return Err(TransferError::CreditLimitExceeded(to_name.to_string()));
        }

        // Execute transfer
        self.users[from_idx].balance -= amount_i64;
        self.users[to_idx].balance += amount_i64;

        Ok(())
    }

    pub fn accrue_interest(&mut self) {
        use std::cmp::Ordering;

        for user in &mut self.users {
            match user.balance.cmp(&0) {
                Ordering::Greater => {
                    // Debit interest with rounding
                    let interest = (user.balance * self.debit_interest as i64 + 5000) / 10000;
                    user.balance += interest;
                }
                Ordering::Less => {
                    // Credit interest with rounding
                    let abs_balance = (-user.balance) as u64;
                    let interest = ((abs_balance * self.credit_interest + 5000) / 10000) as i64;
                    user.balance -= interest;
                }
                Ordering::Equal => {} // No interest on zero balance
            }
        }
    }

    /// Merges another bank into this one, consuming the other bank
    pub fn merge_bank(&mut self, other: Bank) {
        for other_user in other.users {
            // Try to find existing user with same name
            if let Some(existing_user) = self.users.iter_mut().find(|u| u.name == other_user.name) {
                // Update balance if user exists in both banks
                existing_user.balance += other_user.balance;

                // Sum credit lines (since merging banks combines their capacity)
                existing_user.credit_line += other_user.credit_line;
            } else {
                // Add new user if they don't exist in this bank
                self.users.push(other_user);
            }
        }

        // other bank is now consumed/destroyed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("Alice".to_string(), 5000, 1000);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.credit_line, 5000);
        assert_eq!(user.balance, 1000);
    }

    #[test]
    fn test_bank_operations() {
        let mut bank = Bank::new("Rust Bank".to_string(), 500, 1000);
        bank.add_user(User::new("Bob".to_string(), 3000, -200));
        assert_eq!(bank.users.len(), 1);
    }

    #[test]
    fn test_calc_balance() {
        let mut bank = Bank::new("Test Bank".to_string(), 500, 1000);
        bank.add_user(User::new("Alice".to_string(), 5000, 2000));
        bank.add_user(User::new("Bob".to_string(), 3000, -1000));

        let (liabilities, assets) = bank.calc_balance();
        assert_eq!(liabilities, 2000);
        assert_eq!(assets, 1000);
    }

    #[test]
    fn test_transfer_funds() {
        let mut bank = Bank::new("Test Bank".to_string(), 0, 0);
        bank.add_user(User::new("Alice".to_string(), 5000, 2000));
        bank.add_user(User::new("Bob".to_string(), 3000, -500));

        bank.transfer_funds("Alice", "Bob", 500).unwrap();
        assert_eq!(bank.users[0].balance, 1500);
        assert_eq!(bank.users[1].balance, 0);

        assert!(bank.transfer_funds("Alice", "Bob", 2000).is_err());
    }

    #[test]
    fn test_accrue_interest() {
        let mut bank = Bank::new("Test Bank".to_string(), 500, 1000);
        bank.add_user(User::new("Alice".to_string(), 10000, 2000));
        bank.add_user(User::new("Bob".to_string(), 10000, -1000));

        bank.accrue_interest();
        assert_eq!(bank.users[0].balance, 2200);
        assert_eq!(bank.users[1].balance, -1050);
    }

    #[test]
    fn test_merge_bank() {
        let mut bank1 = Bank::new("Bank A".to_string(), 500, 1000);
        bank1.add_user(User::new("Alice".to_string(), 5000, 2000));
        bank1.add_user(User::new("Bob".to_string(), 3000, -500));

        let mut bank2 = Bank::new("Bank B".to_string(), 600, 900);
        bank2.add_user(User::new("Alice".to_string(), 4000, 1000));
        bank2.add_user(User::new("Charlie".to_string(), 2000, 1500));

        bank1.merge_bank(bank2);

        // Verify merged users
        assert_eq!(bank1.users.len(), 3);

        // Alice's balance and credit line should be combined (2000 + 1000)
        let alice = bank1.users.iter().find(|u| u.name == "Alice").unwrap();
        assert_eq!(alice.balance, 3000);
        assert_eq!(alice.credit_line, 9000);

        // Bob should remain unchanged
        let bob = bank1.users.iter().find(|u| u.name == "Bob").unwrap();
        assert_eq!(bob.balance, -500);

        // Charlie should be added
        let charlie = bank1.users.iter().find(|u| u.name == "Charlie").unwrap();
        assert_eq!(charlie.balance, 1500);
    }
}
