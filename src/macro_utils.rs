macro_rules! include_all_mod {
    ($($name:ident/$Name:ident) * ) => {
        $(
            mod $name;
            use $name::$Name;
        )*
    };
}

//macro_rules! append_all_mod_on_app {
    //($expr:expr,) => { $expr };
    //($expr:expr, $name:ident/$Name:ident $($other:ident/$Other:ident) * ) => {
        //append_all_mod_on_app!($expr.subcommand($Name::build_cli()), $($other/$Other)*);
    //};
//}

macro_rules! append_all_mod_on_app {
    ($($name:ident/$Name:ident) *) => {
        |x: App<'static, 'static>| x
            $(.subcommand($Name::build_cli()))*
            //append_all_mod_on_app!($expr.subcommand($Name::build_cli()), $($other/$Other)*);
    };
}

macro_rules! generate_match_all_mod {
    ($($name:ident/$Name:ident) * ) => {
        |x| match x {
            $(
                stringify!($name) => $Name::run_unwrap,
            )*
            _ => unreachable!()
        }
    };
}

