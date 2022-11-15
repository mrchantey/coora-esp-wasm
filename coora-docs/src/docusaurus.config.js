// @ts-check
const lightCodeTheme = require('prism-react-renderer/themes/github')
const darkCodeTheme = require('prism-react-renderer/themes/dracula')

/** @type {import('@docusaurus/types').Config} */
const config = {
	title: 'Coora',
	tagline: 'Instant Electronics Development',
	url: 'https://coora.dev',
	baseUrl: '/',
	onBrokenLinks: 'throw',
	onBrokenMarkdownLinks: 'warn',
	favicon: 'img/logo/favicon.ico',
	organizationName: 'mrchantey',
	projectName: 'coora',
	i18n: {
		defaultLocale: 'en',
		locales: ['en'],
	},
	presets: [
		[
			'classic',
			/** @type {import('@docusaurus/preset-classic').Options} */
			({
				docs: {
					sidebarPath: require.resolve('./sidebars.js'),
					editUrl: 'https://github.com/mrchantey/coora/tree/main/coora-docs/',
				},
				blog: {
					showReadingTime: true,
					// Remove this to remove the "edit this page" links.
					editUrl: 'https://github.com/mrchantey/coora/tree/main/coora-docs/',
				},
				theme: {
					customCss: require.resolve('./css/custom.css'),
				},
			}),
		],
	],
	themeConfig:
		/** @type {import('@docusaurus/preset-classic').ThemeConfig} */
		({
			metadata: [
				{ property: 'og:image', content: '/img/logo/og-thumb.png' },
				{ property: 'og:image:type', content: 'image/png' },
				{ property: 'og:image:width', content: '1200' },
				{ property: 'og:image:height', content: '630' },
			],
			navbar: {
				title: 'Coora',
				logo: {
					alt: 'Coora Logo',
					src: 'img/logo/logo.svg',
					href: 'https://coora.dev',
					target: '_self'
				},
				items: [
					{
						type: 'doc',
						docId: 'index',
						position: 'left',
						label: 'Docs',
					},
					{
						to: '/blog',
						label: 'Blog',
						position: 'left'
					},
					{
						href: 'https://github.com/mrchantey/coora',
						label: 'GitHub',
						position: 'right',
					},
				],
			},
			footer: {
				style: 'dark',
				copyright: `Copyright © ${new Date().getFullYear()} Coora`,
			},
			prism: {
				theme: lightCodeTheme,
				darkTheme: darkCodeTheme,
			},
		}),
}

module.exports = config
