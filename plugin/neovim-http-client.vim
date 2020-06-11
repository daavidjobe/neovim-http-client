if !exists('s:httpClientJobId')
	let s:httpClientJobId = 0
endif

let s:bin = '~/.cargo/bin/neovim-http-client'
let s:Request = 'request'

function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "neovim-http-client: cannot start rpc process"
  elseif -1 == id
    echoerr "neovim-http-client: rpc process is not executable"
  else
    let s:httpClientJobId = id 
 
    call s:configureCommands()
  endif
endfunction


function! s:configureCommands()
  command! -nargs=+ Request :call s:request(<f-args>)
endfunction


function! s:request(...)
  let s:p = get(a:, 1, 0)
  let s:q = get(a:, 2, 0)

  call rpcnotify(s:httpClientJobId, s:Request, str2nr(s:p), str2nr(s:q))
endfunction


" Initialize RPC
function! s:initRpc()
  if s:httpClientJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:httpClientJobId
  endif
endfunction
