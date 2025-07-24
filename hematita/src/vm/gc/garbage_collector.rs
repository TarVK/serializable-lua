use std::{collections::HashMap, hash::Hash, sync::Arc};

pub struct GarbageCollector {
    suspect: Vec<Gc<dyn Trace>>,
}

pub trait Trace {
    fn trace(&self, visit: &mut dyn FnMut(&Gc<dyn Trace>));
}

pub struct Gc<D: Trace + ?Sized + 'static> {
    data: Option<Arc<D>>,
    suspected: bool,
}

// impl<D: Default> Hash for ArcGC<D> {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         todo!()
//     }
// }

impl GarbageCollector {
    pub fn collect() {}
}
