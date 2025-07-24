pub struct ArcGC<D: Default> {
    data: Arc<D>,
}

// pub trait GCDataChildren {
//     type 
//     type C: Iterator<Item = GCRef>;
//     fn get_children(&self) -> C;
// }

// pub trait GCRef {}

impl<D: Default> ArcGC<D> {
    pub fn check_children
}
