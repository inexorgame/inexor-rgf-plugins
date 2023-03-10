import fetch from "isomorphic-fetch";

function introspectionProvider(query) {
  return fetch(
    window.location.origin + '/dynamic_graph',
    {
      method: 'post',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({query: query}),
    }
  ).then(response => response.json())
}

export default introspectionProvider;
