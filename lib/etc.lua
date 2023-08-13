---@meta

---@class EtcConfig
EtcConfig = {
	---@type string
	hostName = 'hana-machine',
}

---@param custom EtcConfig
function etc(custom)
	custom = custom or EtcConfig
	if custom.hostName ~= nil then
		Hana.writeFile('etc/hostname', custom.hostName)
	end
end
