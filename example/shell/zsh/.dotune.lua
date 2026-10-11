if not Path.which 'zsh' then
	return
end

Dot.link((Path.here / '.zshrc'):to(Path.home))
