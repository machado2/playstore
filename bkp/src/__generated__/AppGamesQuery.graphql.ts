/**
 * @generated SignedSource<<64a18a28f79fae4ba4d2dbe50e64920b>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Query } from 'relay-runtime';
export type AppGamesQuery$variables = {};
export type AppGamesQuery$data = {
  readonly listedApp: {
    readonly nodes: ReadonlyArray<{
      readonly id: string;
      readonly title: string;
    }>;
  };
};
export type AppGamesQuery = {
  response: AppGamesQuery$data;
  variables: AppGamesQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "alias": null,
    "args": null,
    "concreteType": "ListedAppConnection",
    "kind": "LinkedField",
    "name": "listedApp",
    "plural": false,
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "ListedApp",
        "kind": "LinkedField",
        "name": "nodes",
        "plural": true,
        "selections": [
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "id",
            "storageKey": null
          },
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "title",
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ],
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "AppGamesQuery",
    "selections": (v0/*: any*/),
    "type": "QueryRoot",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "AppGamesQuery",
    "selections": (v0/*: any*/)
  },
  "params": {
    "cacheID": "bec43d40cc797584aa5e9a25469e6389",
    "id": null,
    "metadata": {},
    "name": "AppGamesQuery",
    "operationKind": "query",
    "text": "query AppGamesQuery {\n  listedApp {\n    nodes {\n      id\n      title\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "bec84b8dd1b4127d3eb27082def3b38b";

export default node;
