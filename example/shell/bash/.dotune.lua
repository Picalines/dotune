if not Path.which 'bash' then
	return {}
end

local bashrc = '.bashrc'

return {
	Link {
		Path.here / bashrc,
		Path.home / bashrc,
	},
}
