/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @oncall relay
 *
 * @generated SignedSource<<f2979812fd9b9932043740c50d6803ad>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { Fragment, ReaderFragment } from 'relay-runtime';
import type { CounterSuspendsWhenOdd$key } from "./CounterSuspendsWhenOdd.graphql";
import type { LiveState, FragmentType } from "relay-runtime";
import {counter_suspends_when_odd as queryCounterSuspendsWhenOddResolverType} from "../CounterSuspendsWhenOdd.js";
import type { LiveResolverContextType } from "../../../../mutations/__tests__/LiveResolverContextType";
// Type assertion validating that `queryCounterSuspendsWhenOddResolverType` resolver is correctly implemented.
// A type error here indicates that the type signature of the resolver module is incorrect.
(queryCounterSuspendsWhenOddResolverType: (
  rootKey: CounterSuspendsWhenOdd$key,
  _: void,
  context: LiveResolverContextType,
) => LiveState<?number>);
declare export opaque type UserNameAndCounterSuspendsWhenOdd$fragmentType: FragmentType;
export type UserNameAndCounterSuspendsWhenOdd$data = {|
  +counter_suspends_when_odd: ?number,
  +me: ?{|
    +name: ?string,
  |},
  +$fragmentType: UserNameAndCounterSuspendsWhenOdd$fragmentType,
|};
export type UserNameAndCounterSuspendsWhenOdd$key = {
  +$data?: UserNameAndCounterSuspendsWhenOdd$data,
  +$fragmentSpreads: UserNameAndCounterSuspendsWhenOdd$fragmentType,
  ...
};
*/

var node/*: ReaderFragment*/ = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "UserNameAndCounterSuspendsWhenOdd",
  "selections": [
    {
      "alias": null,
      "args": null,
      "concreteType": "User",
      "kind": "LinkedField",
      "name": "me",
      "plural": false,
      "selections": [
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "name",
          "storageKey": null
        }
      ],
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "fragment": {
        "args": null,
        "kind": "FragmentSpread",
        "name": "CounterSuspendsWhenOdd"
      },
      "kind": "RelayLiveResolver",
      "name": "counter_suspends_when_odd",
      "resolverModule": require('./../CounterSuspendsWhenOdd').counter_suspends_when_odd,
      "path": "counter_suspends_when_odd"
    }
  ],
  "type": "Query",
  "abstractKey": null
};

if (__DEV__) {
  (node/*: any*/).hash = "1a64e6e5a1ce87aa12f684231ab0da09";
}

module.exports = ((node/*: any*/)/*: Fragment<
  UserNameAndCounterSuspendsWhenOdd$fragmentType,
  UserNameAndCounterSuspendsWhenOdd$data,
>*/);
