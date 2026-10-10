if not Path.which 'zsh' then
	return
end

local zshrc = '.zshrc'

Dot.link(Path.here / zshrc, Path.home / zshrc)
