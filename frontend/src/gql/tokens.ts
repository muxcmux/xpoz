export const getTokens = `
  query {
    tokens {
      __typename
      id
      name
      sessionId
      sessionBound
      admin
      createdAt
    }
  }
`;

export const createToken = `
  mutation($name: String, sessionBound: Boolean!, $admin: Boolean!) {
    createToken(name: $name, sessionBound: $sessionBound, admin: $sessionBound) {
      __typename
      id
      name
      sessionId
      sessionBound
      admin
      createdAt
    }
  }
`;

export const removeToken = `
  mutation($id: String!) {
    deleteToken(id: $id) {
      __typename
      id
    }
  }
`

