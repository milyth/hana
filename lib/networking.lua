---@meta
---@class NetworkingConfig
NetworkingConfig = {
	---@type table?
	networkManager = nil,

	---@type table?
	iwd = nil,

	---@type table?
	resolvconf = nil,
}

---@param custom NetworkingConfig
function networking(custom)
	if custom.networkManager ~= nil then
		Hana.makeDirAll('etc/NetworkManager/conf.d')

		Hana.writeFile('etc/NetworkManager/conf.d/hana.conf', Hana.toToml(custom.networkManager))
	end

	if custom.iwd ~= nil then
		Hana.makeDirAll('etc/iwd')
		Hana.writeFile('etc/iwd/main.conf', Hana.toToml(custom.iwd))
	end

	if custom.resolvconf ~= nil then
		custom.resolvconf.resolv_conf = '/etc/resolv.conf'
		Hana.makeDirAll('etc')
		res = {}

		for k, v in pairs(custom.resolvconf) do
			table.insert(res, string.format('%s="%s"', k, v))
		end

		Hana.writeFile('etc/resolv.conf', table.concat(res, '\n'))
	end
end
