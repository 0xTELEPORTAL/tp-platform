use crate::action::{
    Action, ActionKind, ActionResult, BoxedAction, BoxedActions, Collaction, CollactionResult,
};
use crate::baseline::BaselineKind;
use crate::contract::properties::TPData;
use crate::Realm;

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, TryRecvError};
use tracing;

use std::mem;

type TryApplyResult<T> = Result<CollactionResult<T>, TryRecvError>;
type ApplyResult<T> = Result<CollactionResult<T>, RecvTimeoutError>;

pub type ActionSender<T> = Sender<Collaction<T>>;

/// Manages reading and writing to the `Realm`.
///
/// # Threading architecture
/// The Engine has a queue of pending collactions that indend to mutate the
/// [`Realm`], as well as working copy of the `Realm` state. To avoid data races
/// the `Realm` is never simultaneously readable and writable at the same time.
///
/// The `Engine` cannot be simultaneously written and read from. For this
/// reason, typically things are done in two steps: a writer phase where
/// collactions are dequeued and applied as mutations on the `Realm` state, and
/// a reader phase where all reads of the data take place, free of any mutation.
/// Handling the transitions between these phases is the responsibility of the
/// API Client(s).
pub struct Engine<T: TPData> {
    realm: Realm,
    receiver: Receiver<Collaction<T>>,
}
impl<T: TPData + PartialEq> Engine<T> {
    pub fn new(realm: Realm, queue_capacity: Option<usize>) -> (Self, ActionSender<T>) {
        let (sender, receiver) = if let Some(cap) = queue_capacity {
            crossbeam_channel::bounded(cap)
        } else {
            crossbeam_channel::unbounded()
        };

        let this = Self { realm, receiver };
        (this, sender)
    }

    pub fn realm(&self) -> &Realm {
        &self.realm
    }

    pub fn realm_mut(&mut self) -> &mut Realm {
        &mut self.realm
    }

    /// Same as `apply_timeout()`, but immediately returns if there are no
    /// collactions pending.
    pub fn try_apply(&mut self) -> TryApplyResult<T> {
        let collaction = self.receiver.try_recv()?;
        let result = self.apply_collaction(collaction);
        Ok(result)
    }

    /// Blocks until a collaction is applied or rejected from the pending
    /// collactions, and returns the `CollactionResult`. If there are no
    /// collactions found by `timeout`, returns an error.
    pub fn apply_timeout(&mut self, timeout: std::time::Duration) -> ApplyResult<T> {
        let collaction = self.receiver.recv_timeout(timeout)?;
        let result = self.apply_collaction(collaction);
        Ok(result)
    }

    fn apply_collaction(&mut self, mut collaction: Collaction<T>) -> CollactionResult<T> {
        // Keep track of applied Actions
        let mut applied_actions: Vec<&mut BoxedAction<T>> = Vec::new();

        // Iterate through all Actions in this Collaction.
        let actions = collaction.actions();
        for action in actions {
            let action_result = self.apply_action(action);
            match action_result {
                Ok(()) => {
                    // Keep track of previously-applied Actions.
                    applied_actions.push(action);
                }
                Err(()) => {
                    // Reverse previously-applied Actions within this Collaction.
                    applied_actions.push(action);
                    self.reverse_actions(&mut applied_actions);

                    // Bail and reject this Collaction.
                    return Err(collaction);
                }
            }
        }

        // If all Actions succeeded, approve the Collaction.
        Ok(collaction)
    }

    fn apply_action(&mut self, action: &mut BoxedAction<T>) -> ActionResult {
        let mut was_successful = false;

        match action.kind() {
            ActionKind::StateAssert => {
                // Get data from the Action and compare it against the BaselineFork.
                let state_handle = action.state_handle();
                let data_new = action.raw_data();
                let state_result = self
                    .realm()
                    .baseline(BaselineKind::Fork)
                    .state(state_handle);

                match state_result {
                    Ok(state) => {
                        if &state.0 == data_new {
                            was_successful = true
                        }
                    }
                    Err(e) => {
                        panic!("[Engine] Could not apply StateAssert action: {}", e);
                    }
                }
            }
            ActionKind::StateWrite => {
                // Get data from the Action and apply it to the BaselineFork.
                let state_handle = action.state_handle();
                let data_new = action.raw_data();
                let state_result = self
                    .realm_mut()
                    .baseline_mut(BaselineKind::Fork)
                    .state_mut(state_handle);

                match state_result {
                    Ok(state) => {
                        // Swap the current value with the new data.
                        // This optimizes applying the Action and allows
                        // for its simple reversal if needed.
                        mem::swap(&mut state.0, data_new);

                        was_successful = true;
                    }
                    Err(e) => {
                        panic!("[Engine] Could not apply StateWrite action: {}", e);
                    }
                }
            }
            _ => {
                tracing::warn!(
                    "[Engine] Cannot apply Action of specified ActionKind: not yet implemented. Treating as no-op.",
                );

                was_successful = false;
            }
        }

        if was_successful {
            Ok(())
        } else {
            Err(())
        }
    }

    fn reverse_action(&mut self, action: &mut BoxedAction<T>) {
        // Reverse Action by applying the previous value to the BaselineFork,
        // where applicable.
        match action.kind() {
            ActionKind::StateAssert => {} // no-op
            ActionKind::StateWrite => {
                // Reverse by re-applying the Action.
                // This triggers a value swap.
                self.apply_action(action);
            }
            _ => {
                tracing::warn!(
                    "[Engine] Cannot reverse Action of specified ActionKind: not yet implemented. Treating as no-op."
                );
            }
        }
    }

    fn reverse_actions(&mut self, actions: &mut Vec<&mut BoxedAction<T>>) {
        // Go in FIFO order
        for action in actions.into_iter().rev() {
            self.reverse_action(action);
        }
    }
}
