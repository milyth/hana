require('lib.systemd')
require('lib.etc')
require('lib.networking')
require('lib.users')

systemd {
	zramGenerator = {
		zram0 = {
			['zram-size'] = 'ram / 2',
			['compression-algorithm'] = 'lz4',
			['swap-priority'] = 999,
		},
	},
}

etc {
	hostName = 'testMachine',
}

networking {
	networkManager = {
		main = {
			['rc-manager'] = 'resolveconf',
		},

		device = {
			wifi = {
				backend = 'iwd',
			},
		},
	},

	resolvconf = {
		name_servers = '127.0.0.1 ::1',
		resolve_conf_options = 'edns0 single-request-reopen trust-ad',
	},

	iwd = {
		General = {
			AddressRandomization = 'network',
			EnableNetworkConfiguration = true,
		},

		Network = {
			EnableNetworkConfiguration = true,
		},

		Scan = {
			DisablePeriodicScan = true,
		},
	},
}

users {
	root = {
		groups = { 'wheel', 'test' },
		homeDir = '/home/test',
		shell = '/usr/bin/bash',
	},
}
