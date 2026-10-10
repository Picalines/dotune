if not Path.which 'bash' then
	return
end

local rc = '.bashrc'

Dot.link(Path.here / rc, Path.home / rc)
