let g:neomake_python_mypy_maker = {
    \ 'args': ['--fast-parser', '--python-version', '3.6'],
    \ 'errorformat': '%A%f:%l: error: %m',
    \ }
let g:neomake_python_enabled_makers = ['mypy']
