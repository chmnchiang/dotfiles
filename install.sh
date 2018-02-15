mkdir -p $HOME/.config
INSTALL_PATH=$HOME/.config/dotfiles2
if [ -e $INSTALL_PATH ]; then
    echo -n "It seems that $INSTALL_PATH already exists, shall I remove it (y/n)? "
    read yn
    if echo "$yn" | grep -iq "^y"; then
        rm -rf $INSTALL_PATH
    else
        exit
    fi
fi
cp -r . $INSTALL_PATH
cargo install --force
cat << EOF
Remember to append \$HOME/.cargo/bin/ to your \$PATH, i.e.,
export PATH=\$PATH:\$HOME/.cargo/bin/
EOF
