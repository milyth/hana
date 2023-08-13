function users(custom)
	for userName, futureProps in pairs(custom) do
		local user = Hana.getUserByName(userName)
		Hana.assert(user ~= nil, string.format('User %s must exist!', userName))

		if not user then
			return
		end

		for k, v in pairs(user) do
			if k == 'groups' then
				print('groups', table.concat(v, ' '))
			else
				print(k, v)
			end
		end
	end
end
