mod global;
pub use global::*;

pub struct CounterStorage<'a> {
    storage: Storage<'a>,
}

impl<'a> CounterStorage<'a> {
    const COUNTER_KEY: &'static [u8] = b"counter";

    pub fn new(storage: Storage) -> CounterStorage {
        CounterStorage { storage }
    }

    pub fn get(&self) -> u32 {
        self.storage.read(Self::COUNTER_KEY).unwrap().unwrap_or(0)
    }

    pub fn set(&mut self, val: u32) {
        self.storage.write(Self::COUNTER_KEY, &val);
    }

    pub fn update(&mut self, f: impl FnOnce(u32) -> u32) {
        let val = self.get();
        self.set(f(val));
    }
}
