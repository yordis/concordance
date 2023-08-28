type Target = String;
type AccountNumber = String;
type Timestamp = u128;

#[derive(PartialEq)]
enum AccountNumberStatus {
  Reserved,
  Released,
  Blocked,
}

pub struct AccountNumberAggregate {
  status: AccountNumberStatus,
}

pub struct AccountNumberReserved {
  account_number: AccountNumber,
  target: Target,
  reserved_at: Timestamp,
}

pub struct AccountNumberReleased {
  account_number: AccountNumber,
  released_at: Timestamp,
}

pub struct AccountNumberBlocked {
  account_number: AccountNumber,
  reserved_at: Timestamp,
}

pub enum AccountNumberEvent {
  AccountNumberReserved(AccountNumberReserved),
  AccountNumberReleased(AccountNumberReleased),
  AccountNumberBlocked(AccountNumberBlocked),
}

pub fn initial_state() -> AccountNumberAggregate {
  AccountNumberAggregate {
    status: AccountNumberStatus::Released,
  }
}

pub fn evolve(_aggregate: AccountNumberAggregate, event: AccountNumberEvent) -> AccountNumberAggregate {
  match event {
    AccountNumberEvent::AccountNumberReserved(_event) => AccountNumberAggregate {
      status: AccountNumberStatus::Reserved,
    },
    AccountNumberEvent::AccountNumberReleased(_event) => AccountNumberAggregate {
      status: AccountNumberStatus::Released,
    },
    AccountNumberEvent::AccountNumberBlocked(_event) => AccountNumberAggregate {
      status: AccountNumberStatus::Blocked,
    },
  }
}

// -------------------------------------------------------------------------------------------------

pub struct ReserveAccountNumber {
  account_number: AccountNumber,
  target: Target,
  reserved_at: Timestamp,
}

pub enum ReserveAccountNumberError {
  AccountNumberReservedAlready,
}

pub fn reserve_account_number(aggregate: AccountNumberAggregate, command: ReserveAccountNumber) -> Result<Vec<AccountNumberEvent>, ReserveAccountNumberError> {
  if aggregate.status == AccountNumberStatus::Reserved {
    return Err(ReserveAccountNumberError::AccountNumberReservedAlready);
  }

  Ok(vec![AccountNumberEvent::AccountNumberReserved(AccountNumberReserved {
    account_number: command.account_number,
    target: command.target,
    reserved_at: command.reserved_at,
  })])
}
