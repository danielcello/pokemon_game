use std::fmt::Debug;
use std::borrow::Cow;


#[derive(Debug)]
pub struct Type{
    pub name: Cow<'static, str>,
}

impl Type{
    
}

pub const NORMAL: Type = Type{
    name: Cow::Borrowed("Normal")
};