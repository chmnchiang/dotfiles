#[macro_export]
macro_rules! call_with_all_mod {
    ($macro_name:ident) => {
        $macro_name!(
            neovim/Neovim
            tmux/Tmux
            zsh/Zsh
            tilix/Tilix
            git/Git
        );
    };
}

#[macro_export]
macro_rules! generate_command {
    ($name:ident, $context:ident; $($command:ident),*) => {
        match $name {
            $(stringify!($command) => {
                Self::$command($context)?;
            })*
            _ => unreachable!(),
        }
    }
}

//macro_rules! append_all_mod_on_app {
    //($expr:expr,) => { $expr };
    //($expr:expr, $name:ident/$Name:ident $($other:ident/$Other:ident) * ) => {
        //append_all_mod_on_app!($expr.subcommand($Name::build_cli()), $($other/$Other)*);
    //};
//}


