export const getTokens = `
  query {
    tokens {
      name
      token
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
      name
      token
      sessionId
      sessionBound
      admin
      createdAt
    }
  }
`;

