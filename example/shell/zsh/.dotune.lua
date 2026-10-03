if not which 'zsh' then
	return {}
end

local zshrc = '.zshrc'

return {
	Link {
		Path.here / zshrc,
		Path.home / zshrc,
	},
}
