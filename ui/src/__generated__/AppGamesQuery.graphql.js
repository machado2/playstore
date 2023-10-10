/**
 * @generated SignedSource<<120287c5b4882bc94a69cec143fe6f3e>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

var node = (function(){
var v0 = [
  {
    "kind": "Literal",
    "name": "filters",
    "value": {
      "free": {
        "eq": true
      },
      "minInstalls": {
        "gte": 10000
      },
      "score": {
        "gte": 4
      }
    }
  },
  {
    "kind": "Literal",
    "name": "pagination",
    "value": {
      "pages": {
        "limit": 20,
        "page": 1
      }
    }
  }
],
v1 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
};
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "AppGamesQuery",
    "selections": [
      {
        "alias": null,
        "args": (v0/*: any*/),
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
              (v1/*: any*/),
              {
                "args": null,
                "kind": "FragmentSpread",
                "name": "ListedAppFragment"
              }
            ],
            "storageKey": null
          }
        ],
        "storageKey": "listedApp(filters:{\"free\":{\"eq\":true},\"minInstalls\":{\"gte\":10000},\"score\":{\"gte\":4}},pagination:{\"pages\":{\"limit\":20,\"page\":1}})"
      }
    ],
    "type": "QueryRoot",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "AppGamesQuery",
    "selections": [
      {
        "alias": null,
        "args": (v0/*: any*/),
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
              (v1/*: any*/),
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "title",
                "storageKey": null
              },
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "details",
                "storageKey": null
              }
            ],
            "storageKey": null
          }
        ],
        "storageKey": "listedApp(filters:{\"free\":{\"eq\":true},\"minInstalls\":{\"gte\":10000},\"score\":{\"gte\":4}},pagination:{\"pages\":{\"limit\":20,\"page\":1}})"
      }
    ]
  },
  "params": {
    "cacheID": "5e7d14d3b03cc14ac17fd5d28a87ad5b",
    "id": null,
    "metadata": {},
    "name": "AppGamesQuery",
    "operationKind": "query",
    "text": "query AppGamesQuery {\n  listedApp(pagination: {pages: {page: 1, limit: 20}}, filters: {score: {gte: 4}, minInstalls: {gte: 10000}, free: {eq: true}}) {\n    nodes {\n      id\n      ...ListedAppFragment\n    }\n  }\n}\n\nfragment ListedAppFragment on ListedApp {\n  id\n  title\n  details\n}\n"
  }
};
})();

node.hash = "7a087b08121c5ad4750e5f7a29111b9c";

module.exports = node;
