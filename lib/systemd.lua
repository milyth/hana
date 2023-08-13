---@meta
---@class SystemdConfig
SystemdConfig = {
	---@type { [string]: table }
	zramGenerator = {},
}

---@param custom SystemdConfig
---@return nil
function systemd(custom)
	Hana.makeDirAll('etc/systemd')
	local zram = custom.zramGenerator

	if zram ~= nil then
		Hana.writeFile('etc/systemd/zram-generator.conf', Hana.toIni(zram))
	end
end
