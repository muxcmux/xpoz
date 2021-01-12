export const getTokens = `
  query {
    tokens {
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
      id
    }
  }
`

