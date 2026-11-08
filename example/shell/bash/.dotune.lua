if not Path.which 'bash' then
	return
end

print 'bash is available on PATH'

local rc = '.bashrc'

Dot.link(Path.here / rc, Path.home / rc)
