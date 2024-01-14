use std::marker::PhantomData;

pub struct Present<T>(T);
pub struct Absent<T>(PhantomData<T>);
