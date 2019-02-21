pub mod path;
pub mod git;
use crate::common::*;
//pub mod cmd;

pub trait Op {
    fn commit(&self, context: &Context) -> Result<()>;
}

pub struct Seq {
    inner: Vec<Box<dyn Op>>,
}

//pub struct Seq<T>(pub T);

macro_rules! peel {
    ($macro:ident ; $name:ident $($other:ident)*) => ($macro! { $($other)* })
}

macro_rules! impl_from_for_seq {
    () => ();
    ( $($name:ident)+ ) => {
        #[allow(non_snake_case)] 
        impl<$($name:Op + 'static,)*> From<($($name,)*)> for Seq {
            fn from(tup: ($($name,)*)) -> Seq {
                let ($($name,)*) = tup;
                Seq {
                    inner: vec![
                        $(Box::new($name),)*
                    ]
                }
            }
        }
        peel! { impl_from_for_seq ; $($name)* }
    }
}

impl Op for Seq {
    fn commit(&self, context: &Context) -> Result<()> {
        for op in &self.inner {
            op.commit(context)?;
        }

        Ok(())
    }
}

impl_from_for_seq!(T0 T1 T2 T3 T4 T5 T6 T7);

//macro_rules! call_down {
    //($cont:ident;) => {};
    //($cont:ident; $first:ident $($rest:ident)*) => {
        //call_down!($cont ; $($rest)*);
        //$first.down($cont)?;
    //}
//}

//macro_rules! impl_revop_for_seq {
    //() => ();
    //( $($name:ident)+ ) => {
        //#[allow(non_snake_case)] 
        //impl<$($name:ReversibleOp,)*> ReversibleOp for Seq<($($name,)*)> {
            //fn down(&self, context: &Context) -> Result<()> {
                //let ($(ref $name,)*) = self.0;
                //call_down!(context; $($name)*);
                //Ok(())
            //}
        //}
        //peel! { impl_revop_for_seq ; $($name)* }
    //}
//}

//impl_revop_for_seq!(T0 T1 T2 T3 T4 T5 T6 T7);
