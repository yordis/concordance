mod bankaccount;

type Decide<TAggregate, TCommand, TEvent, TError> =
fn(aggregate: TAggregate, command: TCommand) -> Result<Vec<TEvent>, TError>;
type Evolve<TAggregate, TEvent> = fn(aggregate: TAggregate, event: TEvent) -> TAggregate;
type InitialState<TAggregate> = fn() -> TAggregate;

struct EventSourceSuite<TAggregate, TCommand, TEvent, TError> {
  initial_state: InitialState<TAggregate>,
  decide: Decide<TAggregate, TCommand, TEvent, TError>,
  evolve: Evolve<TAggregate, TEvent>,
  test_cases: Vec<EventSourceTestCase<TAggregate, TCommand, TEvent, TError>>,
}

struct EventSourceTestCase<TAggregate, TCommand, TEvent, TError> {
  description: String,
  given_event: Vec<TEvent>,
  when_command: Option<TCommand>,
  then_error: Option<TError>,
  then_event: Vec<TEvent>,
  then_state: Option<TAggregate>,
}

impl<TAggregate, TCommand, TEvent, TError> EventSourceTestCase<TAggregate, TCommand, TEvent, TError> {
  fn new(description: String) -> Self {
    Self { description, then_error: None, then_event: Vec::new(), when_command: None, given_event: Vec::new(), then_state: None }
  }

  fn given_event(&mut self, event: TEvent) -> &mut Self {
    self.given_event.push(event);
    self
  }

  fn when_command(&mut self, command: TCommand) -> &mut Self {
    self.when_command = Some(command);
    self
  }

  fn then_error(&mut self, error: TError) -> &mut Self {
    self.then_error = Some(error);
    self
  }

  fn then_event(&mut self, event: TEvent) -> &mut Self {
    self.then_event.push(event);
    self
  }

  fn then_state(&mut self, state: TAggregate) -> &mut Self {
    self.then_state = Some(state);
    self
  }
}

impl<TAggregate, TCommand, TEvent, TError> EventSourceSuite<TAggregate, TCommand, TEvent, TError> {
  fn new(
    initial_state: InitialState<TAggregate>,
    evolve: Evolve<TAggregate, TEvent>,
    decide: Decide<TAggregate, TCommand, TEvent, TError>,
  ) -> Self {
    Self {
      initial_state,
      decide,
      evolve,
      test_cases: Vec::new(),
    }
  }

  fn create_test_case(&mut self, description: String) -> &EventSourceTestCase<TAggregate, TCommand, TEvent, TError> {
    let test_case = EventSourceTestCase::new(description);
    self.test_cases.push(test_case);
    self.test_cases.last().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut suite = EventSourceSuite::new(
      bankaccount::initial_state,
      bankaccount::evolve,
      bankaccount::reserve_account_number,
    );

    suite
      .create_test_case("reserve account number".to_string());

    suite.run();

    assert_eq!(2, 4);
  }
}
