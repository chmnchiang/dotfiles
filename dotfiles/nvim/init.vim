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
    "call dein#add('Yggdroot/indentLine')
    call dein#add('nathanaelkane/vim-indent-guides')
    call dein#add('jlanzarotta/bufexplorer')
    call dein#add('Shougo/deoplete.nvim')

    "call dein#add('benekastah/neomake')
    " Git +/- in left bar
    call dein#add('airblade/vim-gitgutter')

    call dein#add('autozimu/LanguageClient-neovim', {'rev': 'next', 'build': 'bash install.sh'})

    " Filetype plugin
    call dein#add('rust-lang/rust.vim', {'on_ft': 'rust'})
    call dein#add('hdima/python-syntax')
    call dein#add('hynek/vim-python-pep8-indent')
    call dein#add('leafgarland/typescript-vim')
    call dein#add('digitaltoad/vim-pug')
    "call dein#add('vim-python/python-syntax')
    call dein#add('othree/yajs.vim')

    call dein#add('roxma/vim-hug-neovim-rpc')
    call dein#add('roxma/nvim-yarp')
            
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

" Since I don't use IndentLine anymore...
" Needed by IndentLine
"set conceallevel=2
"set concealcursor=nc

" Don't show the annoying preview window
set completeopt-=preview

" Always show sign column
set signcolumn=yes

" Set cursor to last known position in Neovim
if has('nvim')
    autocmd BufReadPost *
        \ if line("'\"") > 1 && line("'\"") <= line("$") && &ft !~# 'commit'
        \ |   exe "normal! g`\""
        \ | endif
endif


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
" ^A is used by tmux, change to + -
noremap + <C-a>
noremap - <C-x>
" ^A is used by tmux, change to ^B
inoremap <C-b> <C-a>

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
nnoremap <silent> <leader>gd :call LanguageClient_textDocument_definition()<CR>
nnoremap <silent> <leader>gr :call LanguageClient_textDocument_rename()<CR>
nnoremap <silent> <leader>gf :call LanguageClient_textDocument_formatting()<CR>
"nnoremap <silent> <leader>gs :LanguageClientStart<CR>

"}}}

" Plugin settings {{{
" Airline
let g:airline_powerline_fonts = 1
let g:airline#extensions#tabline#enabled = 1

" NERDTree
let NERDTreeShowHidden = 1

" Indent Line
let g:indentLine_setConceal = 0

" Indent guide
let g:indent_guides_enable_on_vim_startup = 1
let g:indent_guides_auto_colors = 0
let g:indent_guides_guide_size = 1
let g:indent_guides_start_level = 2
autocmd VimEnter,Colorscheme * :hi IndentGuidesEven  guibg=#242424 ctermbg=233
autocmd VimEnter,Colorscheme * :hi IndentGuidesOdd   guibg=#2c2c2c ctermbg=234

" Language server
let g:LanguageClient_serverCommands = {
    \ 'cpp': ['cquery', 
        \ '--log-file=/tmp/cq.log', 
        \ '--init={"cacheDirectory":"/tmp/cquery/"}'],
    \ 'rust': ['rls'],
    \ 'python': ['pyls'],
    \ 'javascript': ['javascript-typescript-stdio'],
    \ 'typescript': ['javascript-typescript-stdio'],
    \ }
let g:LanguageClient_autoStart = 1
let g:LanguageClient_useVirtualText = 0

let g:deoplete#enable_at_startup = 1


" Git gutter
let g:gitgutter_map_keys = 0

let g:python_highlight_all = 1

"}}}
