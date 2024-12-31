use vglang::opcode::{data::Data, variable::Variable};

pub trait Context {
    fn valueof<'a, T>(&'a self, variable: &'a Variable<T>) -> Option<&'a T>
    where
        Data: From<T>,
        for<'c> &'c T: TryFrom<&'c Data, Error = ()>;
}
