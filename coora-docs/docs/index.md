# Introduction

:::info Is Coora for me?
This project is highly experimental, and ongoing maintenance is not guaranteed. That said, if you're interested in experimenting with Web Assembly on embedded devices then you're in the right place!
:::


## Overview

Coora is a platform for rapid electronics development. The ecosystem has the following benefits
- **Price:** We're targeting inexpensive boards, you can get started for [under $5!][esp32-c3]
- **Speed:** Imagine a delay of **0.5 seconds** between saving your file, and the changes *running on your device!*
- **Flexibility:** Development is fully wireless and can be done from within your browser, or your favourite IDE.
- **Language:** Choose any language you like, if it [compiles to wasm][wasm-langs], it can run on Coora

### How Coora works

Put simply we provide an engine which contains all the drivers for your hardware, and runs a WebAssembly runtime. This means that all you need to update it is an internet connection and away you go!

### How Coora compares to...

#### Arduino

Arduino is a surprisingly powerful platform, and is relatively beginner friendly too, but it does have a few pain points. 
- **Compilation:** Even a minor change in your code requires a full recompile and for the board to be flashed again, usually over a wired connection. Coora only recompiles your *application logic* on every change.
- **Language:** If you're coding arduino you're coding C++, which is not the most ergonomic language. The Coora Engine is built in rust, the [most loved language][most-loved] 7 years running, and if your only coding in the application layer, you can choose [any language][wasm-langs] that compiles to wasm!

#### Lego NXT

The NXT is fun and easy to get started with, but has these drawbacks:
- **Cost:** A starter kit may set you back over $500 AUD, which isn't feasable for many people. Coora runs on readily available chips & [sensor/motor kits][motor-kit] costing 10x less than that.
- **Standardization:** Lego has abstracted the robotics ecosystem with proprietary layers which, while easy to get started with, delays the inevitable shift to more universal languages and hardware.

[motor-kit]: https://www.aliexpress.com/item/1005003057841508.html?spm=a2g0o.productlist.0.0.208f2401zpqpqd&algo_pvid=4cba2acf-fdde-4eb4-aa0e-01be67ad53c8&algo_exp_id=4cba2acf-fdde-4eb4-aa0e-01be67ad53c8-11&pdp_ext_f=%7B%22sku_id%22%3A%2212000023649904820%22%7D&pdp_npi=2%40dis%21AUD%2147.64%2129.06%21%21%213.34%21%21%402101d8f416684681534123140e43ab%2112000023649904820%21sea&curPageLogUid=aOB5JugjmxpB
[wasm-langs]: https://github.com/appcypher/awesome-wasm-langs
[most-loved]: https://survey.stackoverflow.co/2022/?utm_source=thenewstack&utm_medium=website&utm_content=inline-mention&utm_campaign=platform#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages
[esp32-c3]: https://www.aliexpress.com/item/1005004490215444.html?gatewayAdapt=4itemAdapt