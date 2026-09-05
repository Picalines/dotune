if not Path.which 'bash' then
	return { 'nope!' }
end

return {
	'bash',
	Path.here,
}
