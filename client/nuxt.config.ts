// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({

	modules: [
		'@nuxt/eslint',
		'nuxt-svgo',
		'nuxt-auth-utils',
		'@pinia/nuxt',
		'@vee-validate/nuxt',
	],

	pages: {
		pattern: ['**/index.vue', '*.vue'],
	},

	components: [
		'~/components',
		{
			path: '~/pages',
			pattern: '**/components/**',
			pathPrefix: false,
		},
	],
	devtools: { enabled: true },
	app: {
		pageTransition: { name: 'page', mode: 'out-in' },
	},
	css: ['~/assets/main.scss'],
	runtimeConfig: {
		public: {
			apiBase: '',
			wsBase: '',
			wsDfpwm: '',
		},
	},

	compatibilityDate: '2025-07-15',

	// Proxy для WebSocket в режиме разработки
	nitro: {
		devProxy: {
			'/ws': {
				target: 'http://localhost:8080/api/v1/ws/ws',
				ws: true,
			},
		},
	},

	vite: {
		css: {
			preprocessorOptions: {
				scss: {
					additionalData: `
          @use "~/assets/_colors.scss" as *;
          @use "~/assets/_variables.scss" as *;
          `,
				},
			},
		},
	},

	eslint: {
		config: {
			stylistic: {
				indent: 'tab',
				quotes: 'single',
				semi: true,
			},
		},
	},

	svgo: {
		defaultImport: 'component',
		global: false,
		svgoConfig: {
			plugins: [
				{
					name: 'preset-default',
					params: {
						overrides: {
							removeViewBox: false,
							inlineStyles: false,
						},
					},
				},
			],
		},
	},
});
