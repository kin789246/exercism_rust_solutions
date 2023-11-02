/// `InputCellId` is a unique identifier for an input cell.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId {
    id: usize,
}

impl InputCellId {
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self { id: COUNTER.fetch_add(1, Ordering::Relaxed) }
    }
}

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId {
    id: usize,
}

impl ComputeCellId {
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self { id: COUNTER.fetch_add(1, Ordering::Relaxed) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId {
    id: usize,
}

impl CallbackId {
    fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self { id: COUNTER.fetch_add(1, Ordering::Relaxed) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct ComputeCell<'a, T> {
    func: Box<dyn 'a + Fn(&[T])->T>,
    dependencies: Vec<CellId>,
    callbacks: HashMap<CallbackId, Box<dyn 'a + FnMut(T)>>
}

impl<'a, T> ComputeCell<'a, T> 
where T: Copy + PartialEq, {
    fn new<F: 'a + Fn(&[T])->T>(deps: &[CellId], compute_func: F) -> Self {
        Self {
            func: Box::new(compute_func),
            dependencies: deps.iter().cloned().collect(),
            callbacks: HashMap::new()
        }
    }

    fn value(&self, reactor: &Reactor<'a, T>) -> T {
        let args = self.dependencies.iter()
            .map(|&id| reactor.value(id).unwrap())
            .collect::<Vec<_>>();
        (*self.func)(&args)
    }

    fn add_callback<F: 'a + FnMut(T)>(&mut self, id: CallbackId, callback: F) {
        self.callbacks.insert(id, Box::new(callback));
    }

    fn remove_callback(&mut self, id: CallbackId) -> Result<(), RemoveCallbackError> {
        self.callbacks
            .remove(&id)
            .ok_or(RemoveCallbackError::NonexistentCallback)
            .map(|_| ())
    }

    fn on_callbacks(&mut self, value: T) {
        self.callbacks.values_mut().for_each(|ca| ca(value));
    }
}

pub struct Reactor<'a, T> {
    input_values: HashMap<InputCellId, T>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self{
            input_values: HashMap::new(),
            compute_cells: HashMap::new()
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellId {
        let input = InputCellId::new();
        self.input_values.insert(input, _initial);
        input
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + Copy + 'static>(
        &mut self,
        _dependencies: &[CellId],
        _compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        if let Some(id) = _dependencies.iter().find(|&id| self.value(*id).is_none()) {
            return Err(*id);
        }
        let c_id = ComputeCellId::new();
        self.compute_cells.insert(c_id, ComputeCell::new(_dependencies, _compute_func));
        Ok(c_id)
    }
    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(i) => self.input_values.get(&i).copied(),
            CellId::Compute(c) => self.compute_cells
                .get(&c)
                .map(|cell| cell.value(self)),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, _id: InputCellId, _new_value: T) -> bool {
        if !self.input_values.contains_key(&_id) {
            return false;
        }
        let old_values = self.compute_cells.iter()
            .map(|(&c_id, cell)| (c_id, cell.value(self)))
            .collect::<Vec<_>>();

        if let Some(value) = self.input_values.get_mut(&_id) {
            *value = _new_value;
        }

        old_values.iter().for_each(|(c_id, o_val)| {
            let cell = self.compute_cells.get(&c_id).unwrap();
            let n_val = cell.value(self);
            if n_val != *o_val {
                let cell = self.compute_cells.get_mut(&c_id).unwrap();
                cell.on_callbacks(n_val);
            }
        });
        true
    }
    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F:'a + FnMut(T)>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        if let Some(cell) = self.compute_cells.get_mut(&_id) {
            let cb_id = CallbackId::new();
            cell.add_callback(cb_id, _callback);
            return Some(cb_id);
        }
        else {
            return None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(c) = self.compute_cells.get_mut(&cell) {
            return c.remove_callback(callback);
        }
        else {
            return Err(RemoveCallbackError::NonexistentCell);
        }
      }
}
