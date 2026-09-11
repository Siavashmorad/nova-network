use std::collections::BTreeMap;

use nova_types::{Account, Address, Amount};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    accounts: BTreeMap<Address, Account>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateError {
    InsufficientBalance,
    NonceMismatch,
    Overflow,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn account(&self, address: &Address) -> Account {
        self.accounts.get(address).cloned().unwrap_or_default()
    }

    pub fn set_account(&mut self, address: Address, account: Account) {
        self.accounts.insert(address, account);
    }

    pub fn transfer(
        &mut self,
        from: Address,
        to: Address,
        amount: Amount,
        nonce: u64,
    ) -> Result<(), StateError> {
        let sender = self.account(&from);
        if sender.nonce != nonce {
            return Err(StateError::NonceMismatch);
        }
        if sender.balance < amount {
            return Err(StateError::InsufficientBalance);
        }

        if from == to {
            let new_balance = sender
                .balance
                .checked_sub(amount)
                .and_then(|value| value.checked_add(amount))
                .ok_or(StateError::Overflow)?;
            let new_nonce = sender.nonce.checked_add(1).ok_or(StateError::Overflow)?;
            self.set_account(from, Account { balance: new_balance, nonce: new_nonce });
            return Ok(());
        }

        let receiver = self.account(&to);
        let new_sender_balance = sender
            .balance
            .checked_sub(amount)
            .ok_or(StateError::InsufficientBalance)?;
        let new_receiver_balance = receiver
            .balance
            .checked_add(amount)
            .ok_or(StateError::Overflow)?;
        let new_sender_nonce = sender.nonce.checked_add(1).ok_or(StateError::Overflow)?;

        self.set_account(
            from,
            Account {
                balance: new_sender_balance,
                nonce: new_sender_nonce,
            },
        );
        self.set_account(
            to,
            Account {
                balance: new_receiver_balance,
                nonce: receiver.nonce,
            },
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_updates_balances_and_nonce() {
        let from = [1; 20];
        let to = [2; 20];
        let mut state = State::new();
        state.set_account(from, Account { balance: 100, nonce: 0 });
        state.transfer(from, to, 40, 0).unwrap();
        assert_eq!(state.account(&from).balance, 60);
        assert_eq!(state.account(&from).nonce, 1);
        assert_eq!(state.account(&to).balance, 40);
    }
}
