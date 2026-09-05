local src = Path.here / 'alacritty.toml'

return {
	src,
	src * 'windows.toml',
	src * 'common.toml',
}
-- -- можно просто выполнять проверки, а в терминале потом будет выведен их список
-- if not Path.which 'alacritty' then
-- 	return {
-- 		"Alacritty isn't installed",
-- 	}
-- end
--
-- local src = Path.config
-- local to = OS.match {
-- 	darwin = Path.xdgconfig / 'alacritty',
-- 	windows = Path.home / 'AppData' / '...',
-- 	--  для дефолта, либо краш
-- }
--
-- return {
-- 	Link {
-- 		-- Порядок путей неважен. Если один из них указывает на файл внутри репозитория - значит это источник
-- 		src / 'alacritty.toml',
-- 		to / 'alacritty.toml',
-- 	},
--
-- 	Link.prepend {
-- 		src,
-- 		to,
-- 		-- С одним параметром валидна только в prepend
-- 		Link 'alacritty.common.toml',
-- 		OS.is_windows and Link 'alacritty.windows.toml',
-- 	},
-- }
