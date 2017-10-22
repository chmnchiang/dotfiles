" Dein (package manager) {{{
if &compatible
    set nocompatible
endif
let g:dein_base_path = $HOME.'/.vim/dein.vim'
let g:plugin_base_path = $HOME.'/.vim/plugged'
let &runtimepath = &runtimepath . ',' . g:dein_base_path

if dein#load_state(g:plugin_base_path)
    call dein#begin(g:plugin_base_path)

    call dein#add(g:dein_base_path)
    
    " Plugins
    call dein#add('tpope/vim-surround')
    call dein#add('honza/vim-snippets')
    call dein#add('garbas/vim-snipmate')
    call dein#add('tomtom/tlib_vim')
    call dein#add('Shougo/echodoc.vim')

    " Nerdtree (file tree navigator)
    call dein#add('scrooloose/nerdtree')

    call dein#add('scrooloose/nerdcommenter')
    
    " Airline (bottom status line)
    call dein#add('bling/vim-airline')

    call dein#add('marcweber/vim-addon-mw-utils')
    call dein#add('Yggdroot/indentLine')
    call dein#add('jlanzarotta/bufexplorer')
    call dein#add('Shougo/deoplete.nvim')

    call dein#add('benekastah/neomake')
    " Git +/- in left bar
    call dein#add('airblade/vim-gitgutter')

    call dein#add('autozimu/LanguageClient-neovim')

    " Filetype plugin
    call dein#add('rust-lang/rust.vim', {'on_ft': 'rust'})
    call dein#add('hdima/python-syntax')
    call dein#add('hynek/vim-python-pep8-indent')
            
    call dein#end()
    call dein#save_state()
endif

if dein#check_install()
    call dein#install()
endif

"autocmd VimEnter * call dein#call_hook('post_source')
"}}}

" Basic settings {{{
filetype plugin indent on
syntax enable
colorscheme pika

" Tabs settings
" Expand tab to space.
set expandtab
" Default to 4 space per tab
set tabstop=4 shiftwidth=4 softtabstop=4

" Open up line number
set number relativenumber
" Allow cursor to move over lines
set whichwrap+=<,>,[,]
" Key stroke timeout
set timeoutlen=300
" Key sequence timeout, important for meta key to work
set ttimeoutlen=20
" Leave lines above and below
set scrolloff=4
" Don't move to start of line for command G, gg ...
set nostartofline

set hidden
set laststatus=2
" Include the current path
set path+=$PWD/**
set conceallevel=0

set foldenable
" Marker couldn't be use in plain text, but I bet you wouldn't
" need to fold there.
set foldmethod=marker

" Needed by IndentLine
set conceallevel=2
set concealcursor=nc
" Conceal sucks at json files...
autocmd Filetype json setl conceallevel=0
"}}}

" Key settings {{{
let mapleader=' '

" Define {j,k} to g{j,k}, useful when long lines
noremap j gj
noremap k gk
" ^ is more useful but hard to reach
noremap 0 ^
noremap ^ 0
" swap : and ,
noremap : ,
noremap , :

" Open Nerdtree
nnoremap <leader>n :NERDTreeToggle<CR>

" Buffer 
nnoremap <leader>l :bn<CR>
nnoremap <leader>h :bp<CR>
nnoremap <leader>q :bd<CR>
" Terminal
if has("nvim")
    " Tie usual escape to vim
    tnoremap <C-[> <C-\><C-n>
    " Use C-\ instead in terminal
    tnoremap <C-\> <C-[>
endif

" Language Client
nnoremap <silent> K :call LanguageClient_textDocument_hover()<CR>
" TODO: gd without leader
nnoremap <silent> <leader>gd :call LanguageClient_textDocument_definition()<CR>
nnoremap <silent> <leader>rn :call LanguageClient_textDocument_rename()<CR>

"}}}

" Plugin settings {{{
" Airline
let g:airline_powerline_fonts = 1
let g:airline#extensions#tabline#enabled = 1

" NERDTree
let NERDTreeShowHidden = 1

" Indent Line
let g:indentLine_setConceal = 0

" Language server
let g:LanguageClient_serverCommands = {
    \ 'rust': ['rustup', 'run', 'nightly', 'rls'],
    \ }
let g:LanguageClient_autoStart = 1

let g:deoplete#enable_at_startup = 1

" Git gutter
let g:gitgutter_map_keys = 0
"}}}
