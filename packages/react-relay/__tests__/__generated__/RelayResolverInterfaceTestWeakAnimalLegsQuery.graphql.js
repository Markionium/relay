/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @oncall relay
 *
 * @generated SignedSource<<20beace133802bf49b0ab116a3769efa>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { ClientRequest, ClientQuery } from 'relay-runtime';
import type { RelayResolverInterfaceTestWeakAnimalLegsFragment$fragmentType } from "./RelayResolverInterfaceTestWeakAnimalLegsFragment.graphql";
import {weak_animal as queryWeakAnimalResolverType} from "../../../relay-runtime/store/__tests__/resolvers/WeakAnimalQueryResolvers.js";
// Type assertion validating that `queryWeakAnimalResolverType` resolver is correctly implemented.
// A type error here indicates that the type signature of the resolver module is incorrect.
(queryWeakAnimalResolverType: (
  args: {|
    request: WeakAnimalRequest,
  |},
) => ?Query__weak_animal$normalization);
import type { Query__weak_animal$normalization } from "./../../../relay-runtime/store/__tests__/resolvers/__generated__/Query__weak_animal$normalization.graphql";
export type WeakAnimalRequest = {|
  ofType: string,
|};
export type RelayResolverInterfaceTestWeakAnimalLegsQuery$variables = {|
  request: WeakAnimalRequest,
|};
export type RelayResolverInterfaceTestWeakAnimalLegsQuery$data = {|
  +weak_animal: ?{|
    +$fragmentSpreads: RelayResolverInterfaceTestWeakAnimalLegsFragment$fragmentType,
  |},
|};
export type RelayResolverInterfaceTestWeakAnimalLegsQuery = {|
  response: RelayResolverInterfaceTestWeakAnimalLegsQuery$data,
  variables: RelayResolverInterfaceTestWeakAnimalLegsQuery$variables,
|};
*/

var node/*: ClientRequest*/ = (function(){
var v0 = [
  {
    "defaultValue": null,
    "kind": "LocalArgument",
    "name": "request"
  }
],
v1 = [
  {
    "kind": "Variable",
    "name": "request",
    "variableName": "request"
  }
],
v2 = [
  {
    "alias": null,
    "args": null,
    "kind": "ScalarField",
    "name": "__relay_model_instance",
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": {
      "hasClientEdges": true
    },
    "name": "RelayResolverInterfaceTestWeakAnimalLegsQuery",
    "selections": [
      {
        "kind": "ClientEdgeToClientObject",
        "concreteType": null,
        "modelResolvers": null,
        "backingField": {
          "alias": null,
          "args": (v1/*: any*/),
          "fragment": null,
          "kind": "RelayResolver",
          "name": "weak_animal",
          "resolverModule": require('./../../../relay-runtime/store/__tests__/resolvers/WeakAnimalQueryResolvers').weak_animal,
          "path": "weak_animal",
          "normalizationInfo": {
            "kind": "OutputType",
            "concreteType": null,
            "plural": false,
            "normalizationNode": require('./../../../relay-runtime/store/__tests__/resolvers/__generated__/Query__weak_animal$normalization.graphql')
          }
        },
        "linkedField": {
          "alias": null,
          "args": (v1/*: any*/),
          "concreteType": null,
          "kind": "LinkedField",
          "name": "weak_animal",
          "plural": false,
          "selections": [
            {
              "args": null,
              "kind": "FragmentSpread",
              "name": "RelayResolverInterfaceTestWeakAnimalLegsFragment"
            }
          ],
          "storageKey": null
        }
      }
    ],
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "RelayResolverInterfaceTestWeakAnimalLegsQuery",
    "selections": [
      {
        "kind": "ClientEdgeToClientObject",
        "backingField": {
          "name": "weak_animal",
          "args": (v1/*: any*/),
          "fragment": null,
          "kind": "RelayResolver",
          "storageKey": null,
          "isOutputType": true
        },
        "linkedField": {
          "alias": null,
          "args": (v1/*: any*/),
          "concreteType": null,
          "kind": "LinkedField",
          "name": "weak_animal",
          "plural": false,
          "selections": [
            {
              "alias": null,
              "args": null,
              "kind": "ScalarField",
              "name": "__typename",
              "storageKey": null
            },
            {
              "kind": "InlineFragment",
              "selections": [
                {
                  "name": "legs",
                  "args": null,
                  "fragment": {
                    "kind": "InlineFragment",
                    "selections": (v2/*: any*/),
                    "type": "Octopus",
                    "abstractKey": null
                  },
                  "kind": "RelayResolver",
                  "storageKey": null,
                  "isOutputType": true
                }
              ],
              "type": "Octopus",
              "abstractKey": null
            },
            {
              "kind": "InlineFragment",
              "selections": [
                {
                  "name": "legs",
                  "args": null,
                  "fragment": {
                    "kind": "InlineFragment",
                    "selections": (v2/*: any*/),
                    "type": "PurpleOctopus",
                    "abstractKey": null
                  },
                  "kind": "RelayResolver",
                  "storageKey": null,
                  "isOutputType": true
                }
              ],
              "type": "PurpleOctopus",
              "abstractKey": null
            }
          ],
          "storageKey": null
        }
      }
    ],
    "clientAbstractTypes": {
      "__isIWeakAnimal": [
        "Octopus",
        "PurpleOctopus"
      ]
    }
  },
  "params": {
    "cacheID": "a7c1768c48c387258635d29981a0a126",
    "id": null,
    "metadata": {},
    "name": "RelayResolverInterfaceTestWeakAnimalLegsQuery",
    "operationKind": "query",
    "text": null
  }
};
})();

if (__DEV__) {
  (node/*: any*/).hash = "618354efaad641b1957a94e6cf22400d";
}

module.exports = ((node/*: any*/)/*: ClientQuery<
  RelayResolverInterfaceTestWeakAnimalLegsQuery$variables,
  RelayResolverInterfaceTestWeakAnimalLegsQuery$data,
>*/);