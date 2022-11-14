import clsx from 'clsx'
import React from 'react'
import styles from './styles.module.css'

type FeatureItem = {
	title: string
	Svg: React.ComponentType<React.ComponentProps<'svg'>>
	description: JSX.Element
}

const FeatureList: FeatureItem[] = [
	{
		title: 'Code Your Way',
		Svg: require('@site/static/img/features/rainbow-line.svg').default,
		description: (
			<>
				Whether in your favourite IDE in the lab, or from your ipad on the couch,
				Coora provides the flexibility you need.
			</>
		),
	},
	{
		title: 'Extendible Ecosystem',
		Svg: require('@site/static/img/features/open-arm-line.svg').default,
		description: (
			<>
				The Coora engine is 100% open source. Write your own plugins, target unique boards, or customize the entire experience.
			</>
		),
	},
	{
		title: 'Cheap as Chips',
		Svg: require('@site/static/img/features/scales-3-line.svg').default,
		description: (
			<>
				Popular WIFI enabled dev boards like the ESP32-C3 can be purchased for
				<a href="https://www.aliexpress.com/item/1005004490215444.html?gatewayAdapt=4itemAdapt"> under $5!</a>
			</>
		),
	},
]

function Feature({ title, Svg, description }: FeatureItem) {
	return (
		<div className={clsx('col col--4')}>
			<div className="text--center">
				<Svg className={styles.featureSvg} role="img" />
			</div>
			<div className="text--center padding-horiz--md">
				<h3>{title}</h3>
				<p>{description}</p>
			</div>
		</div>
	)
}

export default function HomepageFeatures(): JSX.Element {
	return (
		<section className={styles.features}>
			<div className="container">
				<div className="row">
					{FeatureList.map((props, idx) => (
						<Feature key={idx} {...props} />
					))}
				</div>
			</div>
		</section>
	)
}
