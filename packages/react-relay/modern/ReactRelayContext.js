/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @flow strict-local
 * @format
 */

'use strict';
const React = require('React');

import type {RelayEnvironmentInterface as ClassicEnvironment} from '../classic/store/RelayEnvironment';
import type {RelayClassicContext} from '../classic/tools/RelayTypes';
import type {GraphQLTaggedNode, IEnvironment, Variables} from 'relay-runtime';

export type ReactRelayCompatContext = {|
  environment: IEnvironment | ClassicEnvironment,
  query?: GraphQLTaggedNode,
  variables: Variables,
|};

export type ReactRelayModernContext = {|
  environment: IEnvironment,
  query?: GraphQLTaggedNode,
  variables: Variables,
|};

export type ReactRelayContextType =
  | ReactRelayModernContext
  | ReactRelayCompatContext
  | RelayClassicContext
  | null;

const ReactRelayContext = React.createContext<ReactRelayContextType>(null);

module.exports = ReactRelayContext;
