---
id: introduction
title: Introduction
slug: /guided-tour/
description: Relay guided tour
keywords:
- guided tour
- relay
- graphql
- documentation
---

import DocsRating from '@site/src/core/DocsRating';
import {OssOnly, FbInternalOnly} from 'internaldocs-fb-helpers';

import FbCrashCourse from './fb/FbCrashCourse.md';
import Sandpack from '@site/src/components/Sandpack';


<Sandpack>

```js index.js active
import './styles.css';
import {render} from 'react-dom';
import App from './App.js';

render(<App />, document.getElementById('root'));
```

```js App.js
export default function App() {
  return <h1>Hello, world!</h1>;
}
```

</Sandpack>

In this guided tour, we're going to go over how to use Relay to build out some of the more common use cases in apps. If you're interested in a detailed reference of our APIs, check out our **[API Reference](../api-reference/relay-environment-provider/)**.


## Before you read

Before getting started, bear in mind that we assume some level of familiarity with:

<FbInternalOnly>

* [Javascript](https://our.internmc.facebook.com/intern/wiki/JavaScript/)
* [React](https://our.internmc.facebook.com/intern/wiki/ReactGuide/)
* [GraphQL](https://our.internmc.facebook.com/intern/wiki/GraphQL/) and our internal [GraphQL Server](https://our.internmc.facebook.com/intern/wiki/Graphql-for-hack-developers/)

</FbInternalOnly>

<OssOnly>

* [Javascript](https://felix-kling.de/jsbasics/)
* [React](https://reactjs.org/docs/getting-started.html)
* [GraphQL](https://graphql.org/learn/)

</OssOnly>

<FbCrashCourse />

<DocsRating />
