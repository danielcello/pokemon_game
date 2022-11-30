use std::fmt::Debug;
use std::borrow::Cow;


#[derive(Debug)]
pub struct Type{
    pub name: Cow<'static, str>,
}

impl Type{
    
}

pub const NORMAL: Type = Type{Cow::Borrowed(("Normal"))};
const type_dict: &'static [&'static  = &[&[NORMAL, NORMAL.name]] 

